use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;

fn get_digits(instr: String) -> u32 {
    let mut i: usize = 0;
    let mut j = instr.len()-1;
    let mut firstdig: char = 'a';
    let mut lastdig: char  = 'a';
    while firstdig == 'a' || lastdig == 'a' {
        let icheck: char = instr.chars().nth(i).unwrap();
        if firstdig == 'a' && icheck.is_digit(10) {
            firstdig = icheck;
        }
        let jcheck: char = instr.chars().nth(j).unwrap();
        if lastdig == 'a' && jcheck.is_digit(10) {
            lastdig = jcheck;
        }
        j = j.saturating_sub(1);
        i = i.saturating_add(1);
    }

    let mut result = String::from("");
    result.push(firstdig);
    result.push(lastdig);
    println!("Digits found: {result}");
    return result.parse::<u32>().expect("Couldn't parse digits");
}
fn main() {
    let input_path = Path::new(r"C:\Users\windo\Documents\AdventofCode\2023\day01a\input.txt");
    let showme = input_path.display();
    println!("Input path is: {showme}");

    let reader = BufReader::new(File::open(input_path).expect("Cannot open file.txt"));
    let mut result: u32 = 0;
    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            result = result + get_digits(String::from(word));
        }
    }
    println!("result is: {result}");
}
