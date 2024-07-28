use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;

#[derive(Debug, Clone)]
struct Card {
    rank: String,
    suit: String,
}

impl Card {
    fn new(rank: &str, suit: &str) -> Card {
        Card {
            rank: rank.to_string(),
            suit: suit.to_string(),
        }
    }

    fn display(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }
}

struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let ranks = vec![
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A",
        ];
        let suits = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
        let mut cards = Vec::new();

        for suit in &suits {
            for rank in &ranks {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

struct Dealer {
    hand: Vec<Card>
}

impl Dealer {
    fn new() -> Dealer {
        Dealer {
            hand: Vec::new(),
        }
    }

    fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn show_hand(&self) {
        for card in &self.hand {
            println!("{}", card.display());
        }
        println!();
    }
}

struct Player {
    name: String,
    hand: Vec<Card>,
    chips: u32,
}

impl Player {
    fn new(name: &str, chips: u32) -> Player {
        Player {
            name: name.to_string(),
            hand: Vec::new(),
            chips,
        }
    }

    fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn show_hand(&self) {
        for card in &self.hand {
            println!("{}", card.display());
        }
        println!();
    }

    fn place_bet(&mut self, amount: u32) -> bool {
        if self.chips >= amount {
            self.chips -= amount;
            true
        } else {
            false
        }
    }

    fn receive_winnings(&mut self, amount: u32) {
        self.chips += amount;
    }
}

struct Game {
    deck: Deck,
    dealer: Dealer,
    players: Vec<Player>,
    pot: u32,
}

impl Game {
    fn new(player_names: Vec<(&str, u32)>) -> Game {
        let mut deck = Deck::new();
        deck.shuffle();

        let dealer = Dealer::new();

        let players = player_names
            .into_iter()
            .map(|(name, chips)| Player::new(name, chips))
            .collect();

        Game { deck, dealer, players, pot: 0 }
    }

    fn deal_to_dealer(&mut self) {
        if let Some(card) = self.deck.deal() {
            self.dealer.add_card(card);
        }
    }

    fn deal_to_players(&mut self, cards_per_player: usize) {
        for _ in 0..cards_per_player {
            for player in &mut self.players {
                if let Some(card) = self.deck.deal() {
                    player.add_card(card);
                }
            }
        }
    }

    fn next_card(&mut self) {
        self.deal_to_dealer();
        println!("Table cards:");
        self.dealer.show_hand();
    }

    fn place_bets(&mut self) {
        for player in &mut self.players {
            println!("{} has {} chips.", player.name, player.chips);
            println!("{}'s turn to bet.", player.name);
            // For simplicity, assume each player bets a fixed amount (e.g., 10 chips)
            let bet_amount = 10;
            if player.place_bet(bet_amount) {
                self.pot += bet_amount;
                println!("{} bets {} chips. Remaining chips: {}", player.name, bet_amount, player.chips);
            } else {
                println!("{} doesn't have enough chips to bet.", player.name);
            }
        }
        println!("Total pot: {} chips.", self.pot);
    }

    fn play(&mut self) {
        self.deal_to_players(2);
        for player in &self.players {
            println!("{}'s hand:", player.name);
            player.show_hand();
        }
    }

    fn game_loop(&mut self) {
        self.play();
        loop {
            self.place_bets();
            self.next_card();
            // Add logic to determine if the game should continue or end
            // For simplicity, let's assume we break after dealing 5 cards to the dealer
            if self.dealer.hand.len() >= 5 {
                break;
            }
        }
        println!("Game over. Final table cards:");
        self.dealer.show_hand();
        // Determine winner, distribute pot, etc.
    }
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    let player_names = vec![("Alice", 100), ("Bob", 100)];
    let mut game = Game::new(player_names);
    game.game_loop();
}
