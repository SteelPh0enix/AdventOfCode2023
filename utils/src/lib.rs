//! Module with various utilities used in my Advent of Code 2023 solutions

use std::fs;

pub const INPUT_PATH: &str = "./input";

/// Reads the [`INPUT_PATH`] and returns it as a string.
/// Panics with appropriate message if can't find/read the file.
pub fn load_aoc_input() -> String {
    fs::read_to_string(INPUT_PATH).unwrap_or_else(|_| {
        panic!("Cannot read the input data!\nYou must run the solution from it's directory, where `input` file is!")
    })
}
