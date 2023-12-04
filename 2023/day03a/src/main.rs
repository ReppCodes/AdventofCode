use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
struct SymbolCoordinates {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq, Hash)]
struct HorizontalSegment {
    x_min: i32,
    x_max: i32,
}

struct EngineSchematic {
    symbols: HashMap<SymbolCoordinates, char>,
    numbers: Vec<Vec<(HorizontalSegment, u32)>>,
}

fn parse_input(reader: BufReader<File>) -> EngineSchematic {
    let mut symbols = HashMap::new();
    let mut numbers = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        let mut line_numbers = Vec::new();

        let mut is_number = false;
        let mut number = 0;
        let mut number_x_min = 0;
        let mut number_x_max = 0;

        for (x, character) in line.unwrap().chars().enumerate() {
            match character {
                digit if digit.is_ascii_digit() => {
                    if !is_number {
                        is_number = true;
                        number_x_min = x as i32;
                    }

                    number_x_max = x as i32;

                    number = 10 * number + digit.to_digit(10).unwrap();
                }
                symbol => {
                    if is_number {
                        line_numbers.push((
                            HorizontalSegment {
                                x_min: number_x_min,
                                x_max: number_x_max,
                            },
                            number,
                        ));

                        is_number = false;
                        number = 0;
                    }

                    match symbol {
                        '.' => {}
                        _ => {
                            symbols.insert(
                                SymbolCoordinates {
                                    x: x as i32,
                                    y: y as i32,
                                },
                                symbol,
                            );
                        }
                    }
                }
            }
        }

        if is_number {
            line_numbers.push((
                HorizontalSegment {
                    x_min: number_x_min,
                    x_max: number_x_max,
                },
                number,
            ));
        }

        numbers.push(line_numbers)
    }

    EngineSchematic { symbols, numbers }
}

fn main() {
    let input_path = Path::new(r"C:\Users\windo\Documents\AdventofCode\2023\day03a\input.txt");
    let reader = BufReader::new(File::open(input_path).expect("Cannot open file.txt"));
    let engine_schematic = parse_input(reader);

    let mut result: u32 = 0;
    for (line_y, line) in engine_schematic.numbers.iter().enumerate() {
        for (segment, number) in line.iter() {
            for x in segment.x_min - 1..=segment.x_max + 1 {
                if engine_schematic.symbols.contains_key(&SymbolCoordinates {
                    x,
                    y: line_y as i32 - 1,
                }) || engine_schematic.symbols.contains_key(&SymbolCoordinates {
                    x,
                    y: line_y as i32 + 1,
                }) {
                    result += number;
                    break;
                }
            }

            if engine_schematic.symbols.contains_key(&SymbolCoordinates {
                x: segment.x_min - 1,
                y: line_y as i32,
            }) || engine_schematic.symbols.contains_key(&SymbolCoordinates {
                x: segment.x_max + 1,
                y: line_y as i32,
            }) {
                result += number;
                break;
            }
        }
    }

    println!("result is: {result}");
    // 311864 is too low
    // 539713 is correct
}