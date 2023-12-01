use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::cmp::min;

fn main() {
    let input_path = Path::new(r"C:\Users\windo\Documents\AdventofCode\2023\day01b\input.txt");
    let reader = BufReader::new(File::open(input_path).expect("Cannot open file.txt"));
    let mut result: u32 = 0;
    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            result = result + get_digits(String::from(word));
        }
    }
    println!("result is: {result}");
}
fn check_written_num(instr: &String, index: usize) -> (bool, char) {
    // let showme = &instr[index..min(instr.len(), index+3)];
    // println!("Checking for one: {showme}");

    if instr[index..min(instr.len(), index+3)] == *"one" {
        return (true, '1');
    } else if instr[index..min(instr.len(), index+3)] == *"two" {
        return (true, '2');
    } else if instr[index..min(instr.len(), index+5)] == *"three" {
        return (true, '3');
    } else if instr[index..min(instr.len(), index+4)] == *"four" {
        return (true, '4');
    } else if instr[index..min(instr.len(), index+4)] == *"five" {
        return (true, '5');
    } else if instr[index..min(instr.len(), index+3)] == *"six" {
        return (true, '6');
    } else if instr[index..min(instr.len(), index+5)] == *"seven" {
        return (true, '7');
    } else if instr[index..min(instr.len(), index+5)] == *"eight" {
        return (true, '8');
    } else if instr[index..min(instr.len(), index+4)] == *"nine" {
        return (true, '9');
    } else if instr.chars().nth(index).unwrap() == '0' {
        return (true, '0');
    } else if instr.chars().nth(index).unwrap() == '1' {
        return (true, '1');
    } else if instr.chars().nth(index).unwrap() == '2' {
        return (true, '2');
    } else if instr.chars().nth(index).unwrap() == '3' {
        return (true, '3');
    } else if instr.chars().nth(index).unwrap() == '4' {
        return (true, '4');
    } else if instr.chars().nth(index).unwrap() == '5' {
        return (true, '5');
    } else if instr.chars().nth(index).unwrap() == '6' {
        return (true, '6');
    } else if instr.chars().nth(index).unwrap() == '7' {
        return (true, '7');
    } else if instr.chars().nth(index).unwrap() == '8' {
        return (true, '8');
    } else if instr.chars().nth(index).unwrap() == '9' {
        return (true, '9');
    } else {
        return (false, 'a');
    }
}

fn get_digits(instr: String) -> u32 {
    let mut i: usize = 0;
    let mut j = instr.len()-1;
    let mut firstdig: char = 'a';
    let mut lastdig: char  = 'a';

    while firstdig == 'a' || lastdig == 'a' {
        if firstdig == 'a' {
            let (i_is_digit, idigit) = check_written_num(&instr, i);
            if i_is_digit {
                firstdig = idigit;
            }
        }
        if lastdig == 'a' {
            let (j_is_digit, jdigit) = check_written_num(&instr, j);
            if j_is_digit {
                lastdig = jdigit;
            }
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
