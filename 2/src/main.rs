use std::env;
use std::fs;

use day_2::Color;
use day_2::Configuration;

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents = parse_file(&args).to_string();

    // println!("With text:\n{contents}");

    let games = day_2::parse(&contents);

    // println!("{:?}", games)

    let mut sum: u16 = 0;
    for game in &games {
        let r = game.total(Color::Red);
        let g = game.total(Color::Green);
        let b = game.total(Color::Blue);

        let config = Configuration {
            red: 12,
            green: 13,
            blue: 14,
        };

        let possible = compare(r, g, b, config);
        if possible {
            // println!("\n=====\ngame {:?}", game.id);
            // println!("{}", game.id);
            sum += game.id
        }
        // for cube in game.cubes {
        //     println!("{:?}", cube.total())
        // }
    }
    let mut powersum: u32 = 0;
    for game in games {
        let r = game.total(Color::Red) as u32;
        let g = game.total(Color::Green) as u32;
        let b = game.total(Color::Blue) as u32;

        powersum += r * g * b;
    }
    println!("sum part1: {}", sum);
    println!("powersum part2: {}", powersum);
}

fn parse_file(args: &[String]) -> String {
    let file_path = &args[1];

    // println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

fn compare(r: u8, g: u8, b: u8, config: Configuration) -> bool {
    if config.red < r || config.green < g || config.blue < b {
        return false;
    }
    true
}
