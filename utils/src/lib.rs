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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextBlock<'a> {
    /// Content of the text block
    pub content: &'a str,
    /// Index of the first character of text block in original string
    pub start_index: usize,
    /// Index of the last character of text block in original string
    pub end_index: usize,
}

/// Extracts all continuous text blocks that contain characters for which test `check_criteria` will
/// return `true`, separated from each other by characters that do not pass this test.
///
/// `check_criteria` should be a functor returning `bool` and accepting three arguments:
/// * current character (`char`)
/// * index of current character in original string (`usize`)
/// * reference to original string (`&str`)
///
/// Returns a list of [`TextBlock`]s. If no text blocks are found, the list is empty.
pub fn extract_all_continuous_text_blocks<F>(text: &str, check_criteria: F) -> Vec<TextBlock>
where
    F: Fn(char, usize, &str) -> bool,
{
    let mut text_blocks = Vec::<TextBlock>::new();
    let mut start_of_block_index = 0usize;
    let mut end_of_block_index = 0usize;
    let mut block_found = false;

    for (index, character) in text.char_indices() {
        if check_criteria(character, index, text) {
            if !block_found {
                start_of_block_index = index;
                block_found = true;
            }
            end_of_block_index = index;
        } else {
            if block_found {
                text_blocks.push(TextBlock {
                    content: &text[start_of_block_index..=end_of_block_index],
                    start_index: start_of_block_index,
                    end_index: end_of_block_index,
                });
                block_found = false;
            }
        }
    }

    // Add last block, if last character was valid
    if block_found {
        text_blocks.push(TextBlock {
            content: &text[start_of_block_index..=end_of_block_index],
            start_index: start_of_block_index,
            end_index: end_of_block_index,
        });
    }

    text_blocks
}

/// A wrapper for `extract_all_continuous_text_blocks` that extracts unsigned numbers.
pub fn extract_all_unsigned_numbers(text: &str) -> Vec<(u32, TextBlock)> {
    extract_all_continuous_text_blocks(text, |c, _, _| c.is_digit(10))
        .iter()
        .map(|&block| (block.content.parse().unwrap(), block))
        .collect()
}

mod tests {
    #[test]
    fn test_extract_all_unsigned_numbers() {
        use crate::{extract_all_unsigned_numbers, TextBlock};

        let check = |text: &str, expected_matches: &[(u32, TextBlock)]| {
            let real_matches = extract_all_unsigned_numbers(text);
            assert_eq!(real_matches.len(), expected_matches.len());
            for (real_match, expected_match) in real_matches.iter().zip(expected_matches.iter()) {
                assert_eq!(real_match, expected_match);
            }
        };

        check(
            "qwe123asd",
            &[(
                123,
                TextBlock {
                    content: "123",
                    start_index: 3,
                    end_index: 5,
                },
            )],
        );

        check(
            "0q1w2e3r5",
            &[
                (
                    0,
                    TextBlock {
                        content: "0",
                        start_index: 0,
                        end_index: 0,
                    },
                ),
                (
                    1,
                    TextBlock {
                        content: "1",
                        start_index: 2,
                        end_index: 2,
                    },
                ),
                (
                    2,
                    TextBlock {
                        content: "2",
                        start_index: 4,
                        end_index: 4,
                    },
                ),
                (
                    3,
                    TextBlock {
                        content: "3",
                        start_index: 6,
                        end_index: 6,
                    },
                ),
                (
                    5,
                    TextBlock {
                        content: "5",
                        start_index: 8,
                        end_index: 8,
                    },
                ),
            ],
        );
    }
}
