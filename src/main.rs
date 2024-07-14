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
    }
}

struct Player {
    name: String,
    hand: Vec<Card>
}

impl Player {
    fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
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
    }
}

struct Game {
    deck: Deck,
    dealer: Dealer,
    players: Vec<Player>,
}

impl Game {
    fn new(player_names: Vec<&str>) -> Game {
        let mut deck = Deck::new();
        deck.shuffle();

        let dealer = Dealer::new();

        let players = player_names
            .into_iter()
            .map(|name| Player::new(name))
            .collect();

        Game { deck, dealer, players }
    }

    fn deal_cards(&mut self, cards_per_player: usize) {
        for _ in 0..cards_per_player {
            for player in &mut self.players {
                if let Some(card) = self.deck.deal() {
                    player.add_card(card);
                }
            }
        }
    }

    fn play(&mut self) {
        self.deal_cards(2);
        for player in &self.players {
            println!("{}'s hand:", player.name);
            player.show_hand();
        }
    }
}


fn main() {
    let _args: Vec<String> = env::args().collect();
    let player_names = vec!["Alice", "Bob"];
    let mut game = Game::new(player_names);
    game.play();
}