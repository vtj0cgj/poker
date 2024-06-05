use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::deck::{Deck, Card};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub id: usize,
    pub name: String,
    pub cash: u64,
    pub hand: Vec<Card>,
    pub bet: u64,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameState {
    pub players: HashMap<usize, Player>,
    pub pot: u64,
    pub deck: Deck,
    pub community_cards: Vec<Card>,
    pub current_bet: u64,
    pub phase: GamePhase,
    pub dealer_position: usize,
    pub current_player: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GamePhase {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: HashMap::new(),
            pot: 0,
            deck: Deck::new(),
            community_cards: Vec::new(),
            current_bet: 0,
            phase: GamePhase::PreFlop,
            dealer_position: 0,
            current_player: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }

    pub fn start_game(&mut self) {
        self.deck.shuffle();
        self.deal_hands();
        self.phase = GamePhase::PreFlop;
    }

    pub fn deal_hands(&mut self) {
        for player in self.players.values_mut() {
            player.hand.push(self.deck.deal().unwrap());
            player.hand.push(self.deck.deal().unwrap());
        }
    }

    pub fn deal_community_cards(&mut self, count: usize) {
        for _ in 0..count {
            self.community_cards.push(self.deck.deal().unwrap());
        }
    }

    pub fn next_phase(&mut self) {
        match self.phase {
            GamePhase::PreFlop => {
                self.phase = GamePhase::Flop;
                self.deal_community_cards(3);
            }
            GamePhase::Flop => {
                self.phase = GamePhase::Turn;
                self.deal_community_cards(1);
            }
            GamePhase::Turn => {
                self.phase = GamePhase::River;
                self.deal_community_cards(1);
            }
            GamePhase::River => {
                self.phase = GamePhase::Showdown;
            }
            GamePhase::Showdown => {
                // Determine winner and reset game or handle accordingly
            }
        }
    }

    pub fn place_bet(&mut self, player_id: usize, amount: u64) {
        let player = self.players.get_mut(&player_id).unwrap();
        player.cash -= amount;
        player.bet += amount;
        self.pot += amount;
        self.current_bet = amount;
        self.advance_turn();
    }

    pub fn fold(&mut self, player_id: usize) {
        let player = self.players.get_mut(&player_id).unwrap();
        player.is_active = false;
        self.advance_turn();
    }

    fn advance_turn(&mut self) {
        // Logic to advance the turn to the next active player
        // Here you'd add code to find the next active player.
    }

    // Add method to determine winner at showdown
}
