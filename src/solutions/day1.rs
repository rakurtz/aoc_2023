use super::super::read_file;
use std::cmp::Ordering;

const DIGITS: [(&'static str, u32); 9] = [("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)];
const NUMBERS: [(&'static str, u32); 9] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

pub fn run() {
    // read file to string
    let input = read_file(1).expect("Couldn't read file");
  
    println!("Day 1, part 1 - {}", calculate(&input, &[&DIGITS]));
    println!("Day 1, part 2 - {}", calculate(&input, &[&DIGITS, &NUMBERS]));

}

#[derive(Copy, Clone, Debug, Eq)]
struct NumberWithPosition {
    number: u32,
    index: usize,
}

impl PartialEq for NumberWithPosition {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}


impl PartialOrd for NumberWithPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index.cmp(&other.index))
    }
}

impl Ord for NumberWithPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

fn calculate(input: &str, numbers: &[&[(&'static str, u32)]]) -> u32 {
    let mut sum = 0;
    
    for line in input.lines() {
        let mut nums: Option<(NumberWithPosition, NumberWithPosition)> = None;
        for n in numbers {
            if let Some((first_written, last_written)) = get_written_numbers(line, n) {
                nums = match nums {
                    Some((first, last)) => Some((first.min(first_written), last.max(last_written))),
                    None => Some((first_written, last_written))
                };
            }
        }
        if let Some((first, last)) = nums {
            sum += first.number *10 + last.number;
        } else {
            panic!("No digit nor written number in line: \n{line}")
        }

    }
    sum
}


fn get_written_numbers(line: &str, numbers: &[(&str, u32)]) -> Option<(NumberWithPosition, NumberWithPosition)> {
    let mut nums = None;
    for num in numbers {
        for f in [str::find, str::rfind] {
            if let Some(index) = f(line, num.0){
                let found = NumberWithPosition {number: num.1, index };
                nums = match nums {
                    None => Some((found, found)),
                    Some((first, last)) => Some((first.min(found), last.max(found)))
                };
            } else {
                break;
            }
        }
    }
    nums
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_pt1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
       
        assert_eq!(142, calculate(input, &[&DIGITS]));
    }

    #[test]
    fn test_day_1_pt2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, calculate(input, &[&DIGITS, &NUMBERS]));
    }
}



