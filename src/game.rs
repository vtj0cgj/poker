use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub current_bet: u64,
    pub pot: u64,
    pub phase: GamePhase,
    pub dealer_position: usize,
    pub current_player: usize,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            deck: Deck::new(),
            current_bet: 0,
            pot: 0,
            phase: GamePhase::WaitingForPlayers,
            dealer_position: 0,
            current_player: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn start_game(&mut self) {
        self.phase = GamePhase::PreFlop;
        self.deal_hands();
    }

    pub fn deal_hands(&mut self) {
        self.deck.shuffle();
        for player in &mut self.players {
            player.hand.push(self.deck.draw_card());
            player.hand.push(self.deck.draw_card());
        }
        self.current_player = (self.dealer_position + 1) % self.players.len();
    }

    pub fn place_bet(&mut self, player_id: usize, amount: u64) {
        if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
            if player.cash >= amount {
                player.cash -= amount;
                player.bet += amount;
                self.pot += amount;
                self.current_bet = self.current_bet.max(amount);
                self.advance_to_next_player();
            }
        }
    }

    pub fn fold(&mut self, player_id: usize) {
        if let Some(player) = self.players.iter_mut().find(|p| p.id == player_id) {
            player.is_active = false;
            self.advance_to_next_player();
        }
    }

    pub fn advance_to_next_player(&mut self) {
        loop {
            self.current_player = (self.current_player + 1) % self.players.len();
            if self.players[self.current_player].is_active {
                break;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub cash: u64,
    pub hand: Vec<Card>,
    pub bet: u64,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut deck = Deck { cards: Vec::with_capacity(52) };
        for suit in &["Hearts", "Diamonds", "Clubs", "Spades"] {
            for rank in 1..=13 {
                deck.cards.push(Card { suit: suit.to_string(), rank });
            }
        }
        deck
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub suit: String,
    pub rank: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum GamePhase {
    WaitingForPlayers,
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}
