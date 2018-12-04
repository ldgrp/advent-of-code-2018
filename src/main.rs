extern crate clap;

use clap::{Arg, App};

mod day01;
mod day02;

fn main() {
    let matches = App::new("Advent of Code 2018")
        .author("Leo Orpilla <leoorpilla3@gmail.com>")
        .about("Solutions to AoC 2018 in Rust")
        .arg(Arg::with_name("day")
            .required(true)
            .help("Problem day"))
        .get_matches();

    let problem = matches.value_of("day").unwrap().parse::<u32>().unwrap();

    match problem {
        1 => day01::solve(),
        2 => day02::solve(),
        //3 => day03::solve();
        //4 => day04::solve();
        //5 => day05::solve();
        //6 => day06::solve();
        //7 => day07::solve();
        //8 => day08::solve();
        //9 => day09::solve();
        //10 => day10::solve();
        //11 => day11::solve();
        //12 => day12::solve();
        //13 => day13::solve();
        //14 => day14::solve();
        //15 => day15::solve();
        //16 => day16::solve();
        //17 => day17::solve();
        //18 => day18::solve();
        //19 => day19::solve();
        //20 => day20::solve();
        //21 => day21::solve();
        //22 => day22::solve();
        //23 => day23::solve();
        //24 => day24::solve();
        //25 => day25::solve();
        _ => eprintln!("Not yet implemented"),
    }
}
