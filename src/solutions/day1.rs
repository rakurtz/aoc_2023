use super::super::read_file;
use std::cmp::Ordering;

pub fn run() {
    // read file to string
    let input = read_file(1).expect("Couldn't read file");
  
    println!("Day 1, part 1 - {}", calculate_part1(input.clone()));
    println!("Day 1, part 2 - {}", calculate_part2(input.clone()));
}

#[derive(Copy, Clone, Debug, Eq)]
struct NumberWithPosition {
    number: Option<usize>,
    index: Option<usize>,
}

impl PartialEq for NumberWithPosition {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}


impl PartialOrd for NumberWithPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.index.as_ref(), other.index.as_ref()) {
            (Some(self_index), Some(other_index)) => Some(self_index.cmp(other_index)),
            // (Some(_), None) => Some(Ordering::Greater),
            // (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => None,
            (None, Some(_)) => None,
            (None, None) => Some(Ordering::Equal),
        }
    }
}

impl Ord for NumberWithPosition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}



fn calculate_part1(input: String) -> usize {

    let mut sum = 0;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let (first, last) = get_digits(line);
        sum += first.number.unwrap_or_else(|| 0) * 10 + last.number.unwrap_or_else(|| 0);
    }
    sum
}

fn calculate_part2(input: String) -> usize {

    let mut sum = 0;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut line_results = Vec::new();

        let (first_digit, last_digit) = get_digits(line);
        let (first_written, last_written) = get_written_numbers(line);


        if first_digit.number.is_some() { line_results.push(first_digit); }
        if last_digit.number.is_some() { line_results.push(last_digit); }
        if first_written.number.is_some() { line_results.push(first_written); }
        if last_written.number.is_some() { line_results.push(last_written); }

        let first = line_results.iter().min().unwrap();
        let last = line_results.iter().max().unwrap();

        sum += first.number.unwrap()*10 + last.number.unwrap();  // we know here, that first and last have Some(number)

    }
    sum
}



fn get_digits(line: &str) -> (NumberWithPosition, NumberWithPosition) {
    let mut first = NumberWithPosition{number: None, index: None};
    let mut last = NumberWithPosition{number: None, index: None};
    
    let mut position_counter = 0;
    for c in line.chars() {
        if let Some(digit) = c.to_digit(10) {
            let found = NumberWithPosition {number: Some(usize::try_from(digit).unwrap()), index: Some(position_counter)};

            if first.number.is_none() {
                first = found;
            }

            if last.number.is_none() {
                last = found;
            }

            if found < first {
                first = found;
            }
            if found > last {
                last = found;
            }
        }
        position_counter += 1;
    }


    (first, last)
        
}


fn get_written_numbers(line: &str) -> (NumberWithPosition, NumberWithPosition) {

    let written_numbers = vec![("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    let mut first = NumberWithPosition { number: None, index: None }; 
    let mut last = NumberWithPosition { number: None, index: None };

    for written_number in written_numbers.clone() {
        if let Some(found_at_index) = line.find(written_number.0){
            // println!("searching number: {}, found at index: {}", written_number.0, found_at_index);
           
            let found = NumberWithPosition {number: Some(written_number.1), index: Some(found_at_index) };

            if first.number.is_none() {
                first = found;
            }

            if last.number.is_none() {
                last = found;
            }

            if found < first {
                first = found;
            }

            if found > last {
                last = found;
            }

        } 
    }


    (first, last)

}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_pt1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();
       
        assert_eq!(142, calculate_part1(input));
    }

    #[test]
    fn test_day_1_pt2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();

let input_oneliner = "two1nine".to_string();

        // assert_eq!(Some(4), input_oneliner.find("nine"));
       
        assert_eq!(281, calculate_part2(input));
    }

   
}



