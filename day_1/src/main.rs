extern crate utils;

/// This is the entire first half of the solution
fn find_calibration_value(line: &str) -> u32 {
    let first = line.chars().find(|symbol| symbol.is_digit(10)).unwrap();
    let last = line
        .chars()
        .rev()
        .find(|symbol| symbol.is_digit(10))
        .unwrap();

    (first.to_digit(10).unwrap() * 10) + last.to_digit(10).unwrap()
}

/// Looks at `line` and tries to find a digit, whether it'll be a character in 0-9 range, or a
/// string representation of a digit.
///
/// Returns new reference to the end of the line, and found digit.
fn find_next_digit<'a>(line: &'a str, digits: &[(String, u32)]) -> Option<(&'a str, u32)> {
    // First, let's see if there's a number anywhere.
    let number_info = line.find(|c: char| c.is_numeric()).map(|index| {
        (
            index,
            1usize, // This is length of the value in characters
            line.chars()
                .skip(index)
                .next()
                .unwrap()
                .to_digit(10)
                .unwrap(),
        )
    });

    // We can return immediately if it's the first character, and if it's not, we can still use that
    // information later.
    if let Some((index, _, number)) = number_info {
        if index == 0 {
            return Some((&line[1..], number));
        }
    }

    // Now, we can assume that the string starts with a letter (or is empty), and there may be more
    // letters that may spell a number, or not. So, let's see if there's a match.
    let mut digit_occurrences: Vec<_> = digits
        .iter()
        // Filter out digits that can't be found in current line, and for those that do,
        // return their index, length in characters, and numeric value - we'll need all that later.
        .filter_map(|(digit, value)| line.find(digit).map(|index| (index, digit.len(), *value)))
        .collect();

    // Also add info about our raw number, if it exist.
    if let Some(info) = number_info {
        digit_occurrences.push(info);
    }

    // Sort everything
    digit_occurrences.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // Get the value that occurred first, if there's any, and map it to return type
    digit_occurrences
        .first()
        .map(|occurrence| (&line[(occurrence.0 + occurrence.1)..], occurrence.2))
}

fn find_real_calibration_value(line: &str) -> u32 {
    let digits = [
        ("one".to_owned(), 1u32),
        ("two".to_owned(), 2),
        ("three".to_owned(), 3),
        ("four".to_owned(), 4),
        ("five".to_owned(), 5),
        ("six".to_owned(), 6),
        ("seven".to_owned(), 7),
        ("eight".to_owned(), 8),
        ("nine".to_owned(), 9),
    ];
    let reversed_digits = digits
        .iter()
        .map(|(digit, value)| (digit.chars().rev().collect::<String>(), *value))
        .collect::<Vec<_>>();

    let first_digit = find_next_digit(line, &digits).unwrap().1;
    let last_digit = find_next_digit(
        line.to_owned().chars().rev().collect::<String>().as_str(),
        reversed_digits.as_slice(),
    )
    .unwrap()
    .1;

    (first_digit * 10) + last_digit
}

fn main() {
    let input = utils::load_aoc_input();
    let lines: Vec<&str> = input.lines().collect();
    let calibration_values: Vec<u32> = lines
        .iter()
        .map(|line| find_calibration_value(line))
        .collect();

    let sum_of_calibration_values: u32 = calibration_values.iter().sum();
    println!("Sum of calibration values is {sum_of_calibration_values}");

    let real_calibration_values: Vec<u32> = lines
        .iter()
        .map(|line| find_real_calibration_value(line))
        .collect();

    let sum_of_real_calibration_values: u32 = real_calibration_values.iter().sum();
    println!("Sum of real calibration values is {sum_of_real_calibration_values}");
}

mod tests {

    #[test]
    fn test_real_digit_parsing() {
        use crate::find_real_calibration_value;

        let test_inputs = [
            ("96xjbsjmkgxgbknqckcrq", 96),
            ("cdgjhklphq3chkgtwofour2", 32),
            ("nine9mqfbxdspvn17zkbl", 97),
            ("3flbbpjgfh", 33),
            ("two9gfn7three1", 21),
            ("zjcksix1dpkdbsxbrseven58", 68),
            ("oner9eight9mtspfsix", 16),
            ("1hhdz156qpfmmrb", 16),
            ("oneeight9", 19),
            ("crznkknhn716djrhfivetwo7", 77),
            ("thkoneight54nsix", 16),
            ("crzrgqk66five5ngdh", 65),
            ("xzjlmmtqgrtqrpmchfdbjpdjkqppxhsvfnzrth7", 77),
            ("sixfourjqbgsix3", 63),
            ("5eightnine54", 54),
            ("26gnqghndfcpmcvngqvzmfjpr8", 28),
            ("eightninethreebjjjvsgnonetwogflphxkvmz9seven", 87),
            ("6qjqkmpstzc", 66),
            ("three97sevenvxone67four", 34),
            ("35ninetmdxqngxr7", 37),
            ("2sbrlnqz1", 21),
            ("1oneight", 18),
        ];

        for (input, expected_output) in test_inputs {
            assert_eq!(find_real_calibration_value(input), expected_output);
        }
    }
}
