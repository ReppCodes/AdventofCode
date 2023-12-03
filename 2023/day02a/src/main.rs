use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Index;
use std::path::Path;
use std::cmp::max;

fn main() {
    let input_path = Path::new(r"C:\Users\windo\Documents\AdventofCode\2023\day02a\input.txt");
    let reader = BufReader::new(File::open(input_path).expect("Cannot open file.txt"));

    let redlim: u32 = 12;
    let greenlim: u32 = 13;
    let bluelim: u32 = 14;
    let mut result: u32 = 0;
    for line in reader.lines() {
        let (id, redmax, greenmax, bluemax) = process_line(line.unwrap());
        if redmax <= redlim && greenmax <= greenlim && bluemax <= bluelim {
            result += id;
        }
    }
    println!("result is: {result}");
    // 5050 is too high
}

fn process_line(line: String) -> (u32, u32, u32, u32) {
    let mut redmax = 0;
    let mut greenmax = 0;
    let mut bluemax = 0;
    // split out game ID and capture it
    let parts: Vec<String> = line.split(":").map(str::to_string).collect();
    let game_id_string = parts.index(0);
    let game_id_string = &game_id_string[5..].to_string();
    let id = game_id_string.parse::<u32>().unwrap();

    // split out games, then process them
    let crop_string = parts.index(1).trim();
    let games = crop_string.split(";");
    for game in games {
        let game_string = String::from(game);
        let colors = game_string.split(",");
        for color in colors {
            let mut color_string = String::from(color);
            color_string = color_string.trim().to_string();
            let game_items: Vec<String> = color_string.split(" ").map(str::to_string).collect();
            let val = game_items.index(0).parse::<u32>().unwrap();
            let color_string_val = game_items.index(1);
            // determine color, update appropriate max
            if color_string_val.starts_with("r") {
                redmax = max(val, redmax);
            } else if color_string_val.starts_with("g") {
                greenmax = max(val, greenmax);
            } else if color_string_val.starts_with("b") {
                bluemax = max(val, bluemax);
            }
        }
    }

    return (id, redmax, greenmax, bluemax);
}
