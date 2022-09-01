use std::collections::HashMap;
use rand::prelude::*;

#[derive(Debug)]
struct Card {
    suit: u8, // 1 = Heart, 2 = Diamonds, 3 = Spades, 4 = Clubs
    card_type: u8  // 1 = Ace, 2 = 2, 3 = 3.. 11 = Jack, 12 = Queen, 13 = King
}

impl Card {
    fn new(used_cards: &mut Vec<u8>) -> Card {
        // Randomly generates a new card
        let card_map = HashMap::from([
            (1, vec![1, 1]),   (2, vec![1, 2]),    (3, vec![1, 3]),   (4, vec![1, 4]),
            (5, vec![1, 5]),   (6, vec![1, 6]),    (7, vec![1, 7]),   (8, vec![1, 8]),
            (9, vec![1, 9]),   (10, vec![1, 10]),  (11, vec![1, 11]), (12, vec![1, 12]),
            (13, vec![1, 13]), (14, vec![2, 1]),   (15, vec![2, 2]),  (16, vec![2, 3]),
            (17, vec![2, 4]),  (18, vec![2, 5]),   (19, vec![2, 6]),  (20, vec![2, 7]),
            (21, vec![2, 8]),  (22, vec![2, 9]),   (23, vec![2, 10]), (24, vec![2, 11]),
            (25, vec![2, 12]), (26, vec![2, 13]),  (27, vec![3, 1]),  (28, vec![3, 2]),
            (29, vec![3, 3]),  (30, vec![3, 4]),   (31, vec![3, 5]),  (32, vec![3, 6]),
            (33, vec![3, 7]),  (34, vec![3, 8]),   (35, vec![3, 9]),  (36, vec![3, 10]),
            (37, vec![3, 11]), (38, vec![3, 12]),  (39, vec![3, 13]), (40, vec![4, 1]),
            (41, vec![4, 2]),  (42, vec![4, 3]),   (43, vec![4, 4]),  (44, vec![4, 5]),
            (45, vec![4, 6]),  (46, vec![4, 7]),   (47, vec![4, 8]),  (48, vec![4, 9]),
            (49, vec![4, 10]), (50, vec![4, 11,]), (51, vec![4, 12]), (52, vec![4, 13])
        ]);

        let mut rng = rand::thread_rng();
        let mut card_number = rng.gen_range(1..53);

        while used_cards.contains(&card_number) {
            // println!("USED");
            card_number = rng.gen_range(1..53);
        }

        used_cards.push(card_number);
        let card_vec = card_map.get(&card_number).unwrap();

        return Card { suit: card_vec[0], card_type: card_vec[1] };
    }

    fn card_string(card_struct: &Card) -> String {
        // Takes Card struct and returns it as a string
        let mut suit = String::new();
        let mut card = String::new();

        match card_struct.suit {
            1 => suit = "Hearts".to_string(),
            2 => suit = "Diamonds".to_string(),
            3 => suit = "Spades".to_string(),
            4 => suit = "clubs".to_string(),
            _ => println!("Not valid suit")
        }

        match card_struct.card_type {
            1 => card = "Ace".to_string(),
            2 => card = "Two".to_string(),
            3 => card = "Three".to_string(),
            4 => card = "Four".to_string(),
            5 => card = "Five".to_string(),
            6 => card = "Six".to_string(),
            7 => card = "Seven".to_string(),
            8 => card = "Eight".to_string(),
            9 => card = "Nine".to_string(),
            10 => card = "Ten".to_string(),
            11 => card = "Jack".to_string(),
            12 => card = "Queen".to_string(),
            13 => card = "King".to_string(),
            _ => println!("Not valid card_type")
        }

        let card_string = format!("{} of {}", card, suit);
        card_string
    }
}

#[derive(Debug)]
struct Hand {
    // struct for a players pre-flop hand
    card_one: Card,
    card_two: Card
}

impl Hand {
    fn new(used_cards: &mut Vec<u8>) -> Self {
        // Generates a player's preflop hand
        let first_card = Card::new(used_cards);
        let second_card = Card::new(used_cards);

        Hand { card_one: first_card, card_two: second_card }
    }
}

enum GameResult {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard
}

impl GameResult {
    fn get_game_result(hand: &Vec<Card>) {
        let mut suits: Vec<u8> = Vec::new();
        let mut cards: Vec<u8> = Vec::new();

        let mut royal_flush: bool = false;
        let mut straight_flush: bool = false;
        let mut four_of_a_kind: bool= false;
        let mut full_house: bool = false;
        let mut flush: bool = false;
        let mut straight: bool = false;
        let mut three_of_a_kind: bool = false;
        let mut two_pair: bool = false;
        let mut pair: bool = false;
        let mut high_card: bool = false;

        for i in 0..7 {
            suits.push(hand[i].suit);
            cards.push(hand[i].card_type);
        }

        suits.sort();
        cards.sort();

        //Flush check
        let mut heart: u8 = 0;
        let mut diamond: u8 = 0;
        let mut spades: u8 = 0;
        let mut clubs: u8 = 0;
        for i in suits.iter() {
            match i {
                1 => heart += 1,
                2 => diamond += 1,
                3 => spades += 1,
                4 => clubs += 1,
                _ => println!("ERROR")
            };

            if heart >= 5|| diamond >= 5 || spades >= 5 || clubs >= 5 {
                flush = true;
            }
        }

        println!("{:?}", suits);
        println!("{:?}", cards);

    }
}

fn main() {
    let mut used_cards: Vec<u8> = Vec::new();
    let mut player: Vec<Card> = Vec::new();

    for i in 0..8 {
        player.push(Card::new(&mut used_cards));
    }

    GameResult::get_game_result(&player);
}


// Game loop
