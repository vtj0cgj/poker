use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use crate::game::{GameState, Player};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Join(String), // Player joining with a name.
    Bet(usize, u64), // Player ID and bet amount.
    Fold(usize), // Player ID.
    // Add more message types as needed.
}

pub async fn start_server(address: &str, starting_cash: u64) {
    let listener = TcpListener::bind(address).await.unwrap();
    println!("Server listening on {}", address);

    let game_state = Arc::new(Mutex::new(GameState::new()));
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn({
        let game_state = game_state.clone();
        async move {
            while let Some((id, message)) = rx.recv().await {
                let mut game_state = game_state.lock().unwrap();
                match message {
                    Message::Join(name) => {
                        let player = Player { id, name, cash: starting_cash, hand: Vec::new(), bet: 0, is_active: true };
                        game_state.add_player(player);
                        if game_state.players.len() >= 2 {
                            game_state.start_game();
                        }
                        // Broadcast updated game state to all players.
                    }
                    Message::Bet(player_id, amount) => {
                        game_state.place_bet(player_id, amount);
                        // Broadcast updated game state to all players.
                    }
                    Message::Fold(player_id) => {
                        game_state.fold(player_id);
                        // Broadcast updated game state to all players.
                    }
                    // Handle other message types.
                }
            }
        }
    });

    let mut player_id = 0;

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let tx = tx.clone();
        tokio::spawn(handle_client(socket, tx, player_id));
        player_id += 1;
    }
}

async fn handle_client(mut socket: TcpStream, tx: mpsc::Sender<(usize, Message)>, player_id: usize) {
    let mut buffer = vec![0; 1024];

    loop {
        let n = socket.read(&mut buffer).await.unwrap();
        if n == 0 {
            return;
        }

        let message: Message = serde_json::from_slice(&buffer[..n]).unwrap();
        tx.send((player_id, message)).await.unwrap();
    }
}

pub async fn start_client(address: &str, name: String) {
    let mut socket = TcpStream::connect(address).await.unwrap();
    let message = Message::Join(name);
    let message = serde_json::to_vec(&message).unwrap();
    socket.write_all(&message).await.unwrap();

    // Client read loop to handle incoming messages and keep the connection alive.
    let mut buffer = vec![0; 1024];

    loop {
        let n = socket.read(&mut buffer).await.unwrap();
        if n == 0 {
            break;
        }

        let message: Message = serde_json::from_slice(&buffer[..n]).unwrap();
        println!("Received message: {:?}", message);

        // Handle different types of messages here as needed.
    }
}
