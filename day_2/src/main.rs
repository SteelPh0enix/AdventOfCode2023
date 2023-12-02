use std::collections::HashMap;

extern crate utils;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub grabs: Vec<Grab>,
}

type Grab = Vec<Cubes>;

#[derive(Debug)]
struct Cubes {
    pub color: Color,
    pub amount: u32,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl From<&str> for Game {
    fn from(game: &str) -> Self {
        let (id, separator_index) = extract_game_id(game);
        // move to the beginning of grabs, +1 to offset the first space
        let game = &game[(separator_index + 1)..];
        let grabs = game
            .split(';')
            .map(|grab| parse_grab(grab))
            .collect::<Vec<Grab>>();

        Game { id, grabs }
    }
}

/// returns game ID and separator's index
fn extract_game_id(game: &str) -> (u32, usize) {
    let game_id_index = game.find(' ').unwrap() + 1;
    let separator_index = game.find(':').unwrap();
    (
        game[game_id_index..separator_index].parse().unwrap(),
        separator_index,
    )
}

fn parse_grab(grab_str: &str) -> Grab {
    grab_str
        .split(',')
        .map(|cubes_str| {
            let cubes_str = cubes_str.trim();
            let (amount_str, color_str) = cubes_str.split_at(cubes_str.find(' ').unwrap());
            Cubes {
                color: color_str.trim().into(),
                amount: amount_str.parse().unwrap(),
            }
        })
        .collect()
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("unknown color '{value}'"),
        }
    }
}

fn get_minimum_cubes_for_game(game: &Game) -> HashMap<Color, u32> {
    let mut minimum_cubes = HashMap::<Color, u32>::new();

    for grab in &game.grabs {
        for cubes in grab {
            if let Some(minimum_amount) = minimum_cubes.get_mut(&cubes.color) {
                if *minimum_amount < cubes.amount {
                    *minimum_amount = cubes.amount
                }
            } else {
                minimum_cubes.insert(cubes.color, cubes.amount);
            }
        }
    }

    minimum_cubes
}

fn main() {
    let input = utils::load_aoc_input();
    let games = input.lines().map(|game| game.into()).collect::<Vec<Game>>();

    let first_part_answer: u32 = games
        .iter()
        .filter(|&game| {
            game.grabs.iter().all(|grab| {
                grab.iter().all(|cubes| match cubes.color {
                    Color::Red => cubes.amount <= 12,
                    Color::Green => cubes.amount <= 13,
                    Color::Blue => cubes.amount <= 14,
                })
            })
        })
        .map(|game| game.id)
        .sum();

    println!("ID of possible games for part 1: {first_part_answer}");

    let second_part_answer: u32 = games
        .iter()
        .map(|game| get_minimum_cubes_for_game(game).values().product::<u32>())
        .sum();
    println!("Second part answer: {second_part_answer}");
}
