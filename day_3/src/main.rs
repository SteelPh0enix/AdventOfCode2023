use std::ops::RangeInclusive;

use utils::TextBlock;

extern crate utils;

/*
We have a "map" in text form as an input, with `.` characters being "empty spaces", numbers, and
symbols. For first task, we need to find all numbers (part numbers) that are adjacent to symbols.

The simplest idea i have is to create a three-line scrolling window over the map. The middle line
will be the "currently parsed" one, and top/bottom lines will be there as middle line's neighborhood.
*/

type PartNumber = u32;

fn contains_symbol(text: &str) -> bool {
    text.chars().any(|c| c != '.' && c.is_ascii_punctuation())
}

fn get_map_window(
    start_index: usize,
    end_index: usize,
    line_width: usize,
) -> RangeInclusive<usize> {
    let start = start_index.saturating_sub(1);
    let end = (end_index + 1).clamp(1, line_width - 1);

    start..=end
}

fn line_contains_symbol_in_block(line: Option<&str>, block: &TextBlock) -> bool {
    line.is_some_and(|line| {
        contains_symbol(&line[get_map_window(block.start_index, block.end_index, line.len())])
    })
}

fn extract_part_numbers<'a>(
    top_line: Option<&'a str>,
    current_line: &'a str,
    bottom_line: Option<&'a str>,
) -> Vec<(PartNumber, TextBlock<'a>)> {
    utils::extract_all_unsigned_numbers(current_line)
        .iter()
        .filter_map(|&(value, block)| {
            if line_contains_symbol_in_block(top_line, &block)
                || line_contains_symbol_in_block(Some(current_line), &block)
                || line_contains_symbol_in_block(bottom_line, &block)
            {
                Some((value, block))
            } else {
                None
            }
        })
        .collect()
}

/// Returns list of part numbers
fn extract_all_part_numbers(input: &str) -> Vec<Vec<(PartNumber, TextBlock)>> {
    let mut part_numbers = Vec::new();
    let mut lines = input.lines();

    let mut top_line: Option<&str> = None;
    let mut current_line: &str = lines.next().unwrap();
    let mut bottom_line: Option<&str> = lines.next();

    while let Some(next_line) = lines.next() {
        part_numbers.push(extract_part_numbers(top_line, current_line, bottom_line));
        top_line = Some(current_line);
        current_line = bottom_line.unwrap();
        bottom_line = Some(next_line);
    }

    // Repeat for last two lines, but push dummy line as bottom one.
    part_numbers.push(extract_part_numbers(top_line, current_line, bottom_line));

    top_line = Some(current_line);
    current_line = bottom_line.unwrap();
    bottom_line = None;

    part_numbers.push(extract_part_numbers(top_line, current_line, bottom_line));

    part_numbers
}

fn main() {
    let input = utils::load_aoc_input();
    let part_numbers = extract_all_part_numbers(input.as_str());

    let part_numbers_sum: u32 = part_numbers.iter().fold(0, |sum, current| {
        sum + current.iter().fold(0, |sum, current| sum + current.0)
    });
    println!("Sum of part numbers: {part_numbers_sum}");
}
