use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Index;
use std::path::Path;

fn main() {
    let input_path = Path::new(r"C:\Users\windo\Documents\AdventofCode\2023\day04a\input.txt");
    let reader = BufReader::new(File::open(input_path).expect("Cannot open file.txt"));

    let mut result: u32 = 0;
    for line in reader.lines() {
        result += process_game(line.unwrap());
    }
    println!("result is: {result}");
    // 50188 is too high
    // 25183 was it
}

fn process_game(game: String) -> u32 {
    // split out game ID and capture it
    let parts: Vec<String> = game.split(":").map(str::to_string).collect();
    let game_id_string = parts.index(0);
    let game_id_string = &game_id_string[5..].to_string();
    let _ = game_id_string.trim().parse::<u32>().unwrap();

    // split out games, then process them
    let crop_string = parts.index(1).trim();
    let game_sections: Vec<String> = crop_string.split("|").map(str::to_string).collect();
    let numbers: Vec<u32> = game_sections.index(0).trim().split(" ").filter_map(|g| g.parse::<u32>().ok()).collect(); 
    let winners: Vec<u32> = game_sections.index(1).trim().split(" ").filter_map(|g| g.parse::<u32>().ok()).collect();

    let mut currval = 1;
    let mut score = 0;
    // println!("game values: {:?}", numbers);
    // println!("winning values: {:?}", winners);
    for num in numbers {
        if winners.contains(&num) {
            score = currval;
            currval *= 2;
        }
    }
    return score;
}