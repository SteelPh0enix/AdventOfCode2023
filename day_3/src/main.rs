use std::ops::RangeInclusive;

extern crate utils;

/*
We have a "map" in text form as an input, with `.` characters being "empty spaces", numbers, and
symbols. For first task, we need to find all numbers (part numbers) that are adjacent to symbols.

The simplest idea i have is to create a three-line scrolling window over the map. The middle line
will be the "currently parsed" one, and top/bottom lines will be there as middle line's neighborhood.
*/

type PartNumber = u32;

fn contains_symbol(text: &str) -> bool {
    text.chars().any(|c| c.is_ascii_punctuation() && c != '.')
}

fn get_map_window(
    start_index: usize,
    end_index: usize,
    line_width: usize,
) -> RangeInclusive<usize> {
    start_index.saturating_sub(1)..=(if end_index < (line_width - 1) {
        end_index + 1
    } else {
        end_index
    })
}

fn extract_part_numbers_to(
    output: &mut Vec<PartNumber>,
    top_line: &str,
    current_line: &str,
    bottom_line: &str,
) {
    utils::extract_all_unsigned_numbers(current_line)
        .iter()
        .filter_map(|&(value, block)| {
            print!("Found {value} @ {block:?} ");
            if contains_symbol(
                &top_line[get_map_window(block.start_index, block.end_index, top_line.len())],
            ) || contains_symbol(
                &current_line
                    [get_map_window(block.start_index, block.end_index, current_line.len())],
            ) || contains_symbol(
                &bottom_line[get_map_window(block.start_index, block.end_index, bottom_line.len())],
            ) {
                println!("and it's a P/N");
                Some(value)
            } else {
                println!();
                None
            }
        })
        .for_each(|value| output.push(value));
}

/// Returns list of part numbers
fn parse_input(input: &str) -> Vec<PartNumber> {
    let mut lines = input.lines();

    let mut current_line: &str = lines.next().unwrap();
    let mut bottom_line: &str = lines.next().unwrap();
    let line_width = current_line.len();
    let dummy_line = String::from_iter((0..line_width).map(|_| '.'));
    let mut top_line: &str = dummy_line.as_str();

    let mut part_numbers = Vec::<PartNumber>::new();

    while let Some(next_line) = lines.next() {
        extract_part_numbers_to(&mut part_numbers, top_line, current_line, bottom_line);
        top_line = current_line;
        current_line = bottom_line;
        bottom_line = next_line;
    }

    // Repeat one last time, but push dummy line as bottom one.
    top_line = current_line;
    current_line = bottom_line;
    bottom_line = dummy_line.as_str();
    extract_part_numbers_to(&mut part_numbers, top_line, current_line, bottom_line);

    part_numbers
}

fn main() {
    let input = utils::load_aoc_input();
    let part_numbers = parse_input(input.as_str());

    println!(
        "Sum of part numbers is {}",
        part_numbers.iter().sum::<u32>()
    );
}
