use super::super::read_file;


pub fn run() {
    // read file to string
    let input = read_file(1).expect("Couldn't read file");
  

    println!("Day 1, part 1 - {}", calculate_part1(input.clone()));
    println!("Day 1, part 2 - {}", calculate_part2(input.clone()));
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct NumberWithPosition {
    number: Option<usize>,
    index: Option<usize>,
}


fn calculate_part1(input: String) -> usize {

    todo!(); // refactored struct
    let mut sum = 0;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let (first, last) = get_digits(line);
        sum += first.number.unwrap_or_else() * 10 + last.unwrap().number;
    }
    sum
}

fn calculate_part2(input: String) -> usize {

    todo!(); // refactored struct

    let mut sum = 0;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let (first_digit, last_digit) = get_digits(line.clone());
        // let first_digit = first_digit.unwrap();
        // let last_digit = last_digit.unwrap();


        let (first_written, last_written) = get_written_nubers(line.clone());

        println!("first written: {:?}, last written: {:?}", first_written, last_written);
        println!("first digit: {:?}, last digit: {:?}", first_digit, last_digit);

        let mut first = 0;
        let mut last = 0;

        if let Some(number_position_pair) = first_written {
            if number_position_pair.index.le(&first_digit.index) {
                first = number_position_pair.number;
            } else {
                first = first_digit.number;
            }
        }

        if let Some(number_position_pair) = last_written {
            if number_position_pair.index.le(&last_digit.index) {
                first = number_position_pair.number;
            } else {
                first = last_digit.number;
            }
        } else {
            if let Some(number_position_pair) = first_written {
                if number_position_pair.index.le(&last_digit.index) {
                    last = number_position_pair.number;
                } else {
                    last = last_digit.number;
                }
            }
        }

        sum += first*10 + last;

    }
    sum
}



fn get_digits(line: &str) -> (NumberWithPosition, NumberWithPosition) {
    let mut first = NumberWithPosition{number: None, index: None};
    let mut last = NumberWithPosition{number: None, index: None};
    
    //quick and dirty index counter
    let mut index_first = 0;
    for c in line.chars() {
        if let Some(digit) = c.to_digit(10) {
            first.number = Some(usize::try_from(digit).unwrap());
            first.index = Some(index_first);
            break;
        }
        index_first += 1;
    }

    let mut index_last_rev = 1;     // starting with one because of .rev()!
    for c in line.chars().rev() {
        if let Some(digit) = c.to_digit(10) {
            let index_last = line.len() - index_last_rev;
            // from beginning: 0 1 2 3 4 5     .len() is 6
            // from end:          3 2 1     6 - 3 = 3
            last.number= Some(usize::try_from(digit).unwrap());
            last.index = Some(index_last);
            break;
        }
        index_last_rev += 1;
    }

    (first, last)
        
}


fn get_written_nubers(line: &str) -> (Option<NumberWithPosition>, Option<NumberWithPosition>) {

    let written_numbers = vec![("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    let mut first: Option<NumberWithPosition> = None;
    let mut last: Option<NumberWithPosition> = None;

    // find first written number

    for written_number in written_numbers.clone() {
        if let Some(found_at_index) = line.find(written_number.0){
            // println!("searching number: {}, found at index: {}", written_number.0, found_at_index);
           
            if let Some(mut cp_first) = first {
                if cp_first.index > found_at_index {
                    cp_first.number = written_number.1;
                    cp_first.index = found_at_index;
                    first = Some(cp_first);
                } 
            } else {
                first = Some(NumberWithPosition {number: written_number.1, index: found_at_index})
            }

            if let Some(mut cp_last) = last {
                if cp_last.index < found_at_index {
                    cp_last.number = written_number.1;
                    cp_last.index = found_at_index;
                    last = Some(cp_last);
                } 
            } else {
                last = Some(NumberWithPosition {number: written_number.1, index: found_at_index})
            }

        } 
    }

    if first.is_some() && last.is_some() {   
        if first.unwrap() == last.unwrap() {
            last = None;
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



