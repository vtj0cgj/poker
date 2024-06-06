use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use crate::game::{GameState, Player};
use std::sync::{Arc};
use tokio::sync::{Mutex, mpsc};
use std::io::{self, Write};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Join(String), // Player joining with a name.
    Bet(usize, u64), // Player ID and bet amount.
    Fold(usize), // Player ID.
    Update(GameState), // Updated game state.
    // Add more message types as needed.
}

pub async fn start_server(address: &str, starting_cash: u64) {
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Server listening on {}", address);

    let game_state = Arc::new(Mutex::new(GameState::new()));
    let (tx, mut rx) = mpsc::channel(32);
    let clients = Arc::new(Mutex::new(HashMap::new()));

    tokio::spawn({
        let game_state = game_state.clone();
        let clients = clients.clone();
        async move {
            while let Some((id, message)) = rx.recv().await {
                let mut game_state = game_state.lock().await;
                match message {
                    Message::Join(name) => {
                        let player = Player { id, name, cash: starting_cash, hand: Vec::new(), bet: 0, is_active: true };
                        game_state.add_player(player);
                        if game_state.players.len() >= 2 {
                            game_state.start_game();
                        }
                        broadcast_game_state(&clients, &game_state).await;
                    }
                    Message::Bet(player_id, amount) => {
                        game_state.place_bet(player_id, amount);
                        broadcast_game_state(&clients, &game_state).await;
                    }
                    Message::Fold(player_id) => {
                        game_state.fold(player_id);
                        broadcast_game_state(&clients, &game_state).await;
                    }
                    Message::Update(_) => {
                        // This case is handled for completeness.
                    }
                }
                // Print the game state for debugging
                println!("{:?}", *game_state);
            }
        }
    });

    let mut player_id = 0;

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let clients = clients.clone();
        let client_id = player_id;
        player_id += 1;

        tokio::spawn(async move {
            handle_client(socket, tx, clients, client_id).await;
        });
    }
}

async fn handle_client(socket: TcpStream, tx: mpsc::Sender<(usize, Message)>, clients: Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>, player_id: usize) {
    let mut buffer = vec![0; 1024];
    let socket = Arc::new(Mutex::new(socket));
    
    {
        let mut clients = clients.lock().await;
        clients.insert(player_id, socket.clone());
    }

    loop {
        let n = socket.lock().await.read(&mut buffer).await.unwrap();
        if n == 0 {
            return;
        }

        let message: Message = serde_json::from_slice(&buffer[..n]).unwrap();
        tx.send((player_id, message)).await.unwrap();
    }
}

async fn broadcast_game_state(clients: &Arc<Mutex<HashMap<usize, Arc<Mutex<TcpStream>>>>>, game_state: &GameState) {
    let message = Message::Update(game_state.clone());
    let message = serde_json::to_vec(&message).unwrap();

    let clients = clients.lock().await;
    for socket in clients.values() {
        let mut socket = socket.lock().await;
        socket.write_all(&message).await.unwrap();
    }
}

pub async fn start_client(address: &str, name: String) {
    let socket = TcpStream::connect(address).await.unwrap();
    let socket = Arc::new(Mutex::new(socket));
    let join_message = Message::Join(name.clone());
    let join_message = serde_json::to_vec(&join_message).unwrap();
    
    {
        let mut socket = socket.lock().await;
        socket.write_all(&join_message).await.unwrap();
    }

    println!("Connected to server as {}", name);

    let socket_clone = Arc::clone(&socket);
    let mut buffer = vec![0; 1024];

    tokio::spawn(async move {
        let socket_clone = socket_clone;
        loop {
            let mut socket = socket_clone.lock().await;
            let n = socket.read(&mut buffer).await.unwrap();
            if n == 0 {
                break;
            }

            let message: Message = serde_json::from_slice(&buffer[..n]).unwrap();
            match message {
                Message::Update(game_state) => {
                    println!("Game State Updated: {:?}", game_state);
                }
                _ => {}
            }
        }
    });

    loop {
        print!("Enter command (bet [amount] / fold): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let message = if input.starts_with("bet ") {
            let amount: u64 = input[4..].parse().unwrap();
            Message::Bet(0, amount) // Player ID will be handled by the server
        } else if input == "fold" {
            Message::Fold(0) // Player ID will be handled by the server
        } else {
            println!("Invalid command");
            continue;
        };

        let message = serde_json::to_vec(&message).unwrap();
        
        {
            let mut socket = socket.lock().await;
            socket.write_all(&message).await.unwrap();
        }
    }
}
