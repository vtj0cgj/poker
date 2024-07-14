struct Player {
    name: String,
    hand: Vec<Card>,
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
    players: Vec<Player>,
}

impl Game {
    fn new(player_names: Vec<&str>) -> Game {
        let mut deck = Deck::new();
        deck.shuffle();

        let players = player_names
            .into_iter()
            .map(|name| Player::new(name))
            .collect();

        Game { deck, players }
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
        self.deal_cards(5);
        for player in &self.players {
            println!("{}'s hand:", player.name);
            player.show_hand();
        }
    }
}
