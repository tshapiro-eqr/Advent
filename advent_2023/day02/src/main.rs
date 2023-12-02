use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
//use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn evaluate_game(game_line:&str) -> Vec<i32> {
    let (game_num, no_game) = game_line.split_once(": ").unwrap();
    let all_commas = str::replace(no_game, ";",","); //worthless semi colons
    let all_rolls = all_commas.split(", ").collect::<Vec<&str>>();

    let (mut max_r, mut max_g, mut max_b) = (0,0,0);
    let mut valid = str::replace(game_num, "Game ","").parse::<i32>().unwrap();

    for roll in all_rolls {
        let (tc, color) = roll.split_once(" ").unwrap();
        let count = tc.parse::<i32>().unwrap();
        
        valid = match color {
            "red"   => if count > 12 {0} else {valid},
            "green" => if count > 13 {0} else {valid},
            "blue"  => if count > 14 {0} else {valid},
            _ => panic!("no matching color!")
        };

        match color {
            "red"   => if count > max_r {max_r=count},
            "green" => if count > max_g {max_g=count},
            "blue"  => if count > max_b {max_b=count},
            _ => panic!("no matching color!")
        }
    }

    return vec![valid, max_r*max_g*max_b]
}


fn main() {
    let mut total_success = 0;
    let mut total_power = 0;
    // File must exist in the current path
    if let Ok(lines) = read_lines("./day02_input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_value) = line {
                let rets = evaluate_game(&line_value);
                total_success += rets.first().unwrap();
                total_power += rets.last().unwrap();
            }
        }
    }
    
    println!("Total success: {}",total_success);
    println!("Total power: {}",total_power);
}
