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
            println!("USED");
            card_number = rng.gen_range(1..53);
        }

        used_cards.push(card_number);
        let card_vec = card_map.get(&card_number).unwrap();
        println!("{:?}", card_vec);

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
        return card_string;
    }
}


fn main() {
    let mut used_cards: Vec<u8> = Vec::new();

    let player = Card::new(&mut used_cards);
    let player_string = Card::card_string(&player);

    println!("USED: {:?}", used_cards);
}
