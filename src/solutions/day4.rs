use std::vec;

use super::super::read_file;
use regex::Regex;

const DAY: usize = 4;

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");

    let result_pt1 = calculate_p1(&input);
    let result_pt2 = "";
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

fn calculate_p1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        sum += parse_line(line);
    }

    sum
}

fn parse_line(line: &str) -> u32 {
    let mut card_points = 0;
    let mut line = line.split(':');

    // seperate winning nummbers from owner's numbers and skip "Card" part of the line
    let mut seperated_numbers = line.nth(1).unwrap().split('|');


    let winning = extract_numbers(seperated_numbers.next().unwrap());
    let owned = extract_numbers(seperated_numbers.next().unwrap());

    for n in owned {
        if winning.contains(&n) {
            card_points += 1;
        }
    }

    if card_points > 0 {
        2u32.pow(card_points - 1)
    } else {
        0
    }
}

fn extract_numbers(haystack: &str) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut numbers = vec![];

    for captures in re.captures_iter(haystack) {
        if let Some(Some(capture)) = captures.iter().next() {
            numbers.push(capture.as_str().parse::<u32>().unwrap());
        }
    }
    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        // part 1
        assert_eq!(13, calculate_p1(input));

        // part 2
        assert_eq!(0, 0);
    }
}
