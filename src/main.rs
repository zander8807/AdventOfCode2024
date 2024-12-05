mod solutions;

use std::{env, fs, path::Path};

use solutions::{day_01::DayOneSolver, Solver};

use crate::solutions::normalize_input;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day_part_split = &args[1].split_once('.').unwrap();
    let day = day_part_split.0;
    let part = day_part_split.1;

    let path_string = format!("resources/inputs/day{}.txt", day);
    let input_path = Path::new(&path_string);
    let input = fs::read_to_string(input_path).unwrap();
    let input = normalize_input(&input).unwrap();

    println!("Finding solution for day {}, part {}", day, part);

    let solver = provide_solver(day);

    let solution = match part {
        "1" => solver.part_1(&input).unwrap(),
        "2" => solver.part_2(&input).unwrap(),
        _ => panic!("unknown part {}", part),
    };

    println!("Solution is {}", solution);
}

fn provide_solver(day: &str) -> Box<dyn Solver> {
    match day {
        "01" => Box::new(DayOneSolver {}),
        _ => todo!(),
    }
}
