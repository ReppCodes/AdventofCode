use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Index;
use std::path::Path;

fn main() {
    let input_path = Path::new(r"C:\Users\windo\Documents\AdventofCode\2023\day04b\input.txt");
    let reader = BufReader::new(File::open(input_path).expect("Cannot open file.txt"));
    let mut card_map: HashMap<u32, Card> = HashMap::new();

    let mut work_stack: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let (_, card, id) = process_game(line.unwrap());
        card_map.insert(id, card);
        work_stack.push(id);
    }

    let mut result: u32 = 0;
    while !work_stack.is_empty() {
        result += 1;
        let game_id = work_stack.pop().unwrap();
        let game_card = card_map.get(&game_id).unwrap();
        // println!("Processing game: {:?}", game_card);
        let (score, _, id) = process_game(game_card.orig_string.clone());

        if score > 0 {
            for n in id+1..id+1+score {
                work_stack.push(n);
            }
        }
        println!("Stack size is: {}", work_stack.len());
    }
    println!("result is: {result}");
}

#[derive(Debug)]
struct Card {
    id: u32,
    orig_string: String,
    numbers: Vec<u32>,
    winners: Vec<u32>,
    score: u32,
}

fn process_game(game: String) -> (u32, Card, u32) {
    // split out game ID and capture it
    let parts: Vec<String> = game.split(":").map(str::to_string).collect();
    let game_id_string = parts.index(0);
    let game_id_string = &game_id_string[5..].to_string();
    let id = game_id_string.trim().parse::<u32>().unwrap();

    // split out games, then process them
    let crop_string = parts.index(1).trim();
    let game_sections: Vec<String> = crop_string.split("|").map(str::to_string).collect();
    let numbers: Vec<u32> = game_sections.index(0).trim().split(" ").filter_map(|g| g.parse::<u32>().ok()).collect(); 
    let winners: Vec<u32> = game_sections.index(1).trim().split(" ").filter_map(|g| g.parse::<u32>().ok()).collect();

    let mut score = 0;
    for num in &numbers {
        if winners.contains(&num) {
            score += 1;
        }
    }

    let card = Card{
        id: id,
        orig_string: game,
        numbers: numbers,
        winners: winners,
        score: score,
    };

    return (score, card, id);
}