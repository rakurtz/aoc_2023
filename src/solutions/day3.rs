use regex::Regex;

use super::super::read_file;

pub fn run() {
    // read file to string
    let input = read_file(3).expect("Couldn't read file");
    let map = Map::new(input);

    println!("Day 3, part 1 {}", map.part_sum());
    println!("Day 3, part 2 {}", map.sum_gear_ratio());

}

#[derive(Debug)]
struct Number {
    value: u32,
    row: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct PotentialGear {
    row: usize,
    position: usize,
}

struct Map {
    map: String,
    numbers: Vec<Number>,
    potential_gears: Vec<PotentialGear>,
}

impl Map {
    fn new(input: String) -> Self {
        let numbers = vec![];
        let potential_gears = vec![];
        let mut map = Map {
            map: input,
            numbers,
            potential_gears,
        };

        // calculations:
        map.find_numbers();
        map.part_sum();
        map.find_potential_gears();

        map
    }

    fn find_numbers(&mut self) {
        let re = Regex::new(r"(\d+)").unwrap();

        for (row, line) in self.map.lines().enumerate() {
            for captures in re.captures_iter(line).map(|c| c) {
                if let Some(capture) = captures.iter().next() {
                    if let Some(capture) = capture {
                        let start = capture.start();
                        let end = capture.end();
                        self.numbers.push(Number {
                            value: capture.as_str().parse::<u32>().unwrap(),
                            row,
                            start,
                            end,
                        });
                    }
                }
            }
        }
    }

    fn is_part_number(&self, number: &Number) -> bool {
        let take_lines;
        let skip_rows;

        if number.row == 0 {
            take_lines = 2;
            skip_rows = 0;
        } else {
            take_lines = 3;
            skip_rows = number.row - 1;
        };

        let take_characters;
        let skip_characters;

        if number.start == 0 {
            take_characters = 1 + number.end - number.start;
            skip_characters = number.start;
        } else {
            take_characters = 2 + number.end - number.start;
            skip_characters = number.start - 1;
        };

        let mut line_iter = self.map.lines().skip(skip_rows).take(take_lines);

        while let Some(line) = line_iter.next() {
            let mut surrounding_characters = line.chars().skip(skip_characters).take(take_characters);
            
            while let Some(c) = surrounding_characters.next() {
                if !c.is_digit(10) && c != '.' {
                    return true;
                }
            }
        }

        false
    }

    fn part_sum(&self) -> u32 {
        let mut sum = 0;

        for number in &self.numbers {
            if self.is_part_number(number) {
                sum += number.value;
            }
        }

        sum
    }

    fn find_potential_gears(&mut self) {
        let re = Regex::new(r"(\*)").unwrap();
        
        for (row, line) in self.map.lines().enumerate() {
            for captures in re.captures_iter(line).map(|c| c) {
                if let Some(capture) = captures.iter().next() {
                    if let Some(capture) = capture {
                        let position = capture.start();
                        let end = capture.end();
                        self.potential_gears.push(PotentialGear { 
                            row,
                            position,
                
                        });
                    }
                }
            }
        }
        
    }

    fn is_gear(&self, gear: &PotentialGear) -> Option<Vec<&Number>> {
        let mut connected_gears = vec![];
        
        for number in &self.numbers {
            
            if (gear.row > 0 && gear.row - 1 == number.row) 
                || gear.row == number.row
                || gear.row + 1 == number.row {

                    let start = if number.start == 0 {
                        0
                    } else {
                       number.start -1
                    };

                    let end = number.end + 1;

                    let range = start..end;

                    if range.contains(&gear.position)  {
                        connected_gears.push(number);
                    }

                }
        }

        assert!(!connected_gears.len() > 2);
        if connected_gears.len() == 2 {
            Some(connected_gears)
        } else {
            None
        }
    }


    fn sum_gear_ratio(&self) -> u32 {
        let mut sum = 0;

        for gear in &self.potential_gears {
            if let Some(gear_numbers) = self.is_gear(&gear) {
                let ratio = gear_numbers[0].value * gear_numbers[1].value;  // safe because is_gear will alway return Option<Vec<Number>> with 2 Numbers!
                sum += ratio;
            }
        }

        sum
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_pt1_no_regex() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            .to_string();

        let map = Map::new(input);

        // part 1
        assert_eq!(4361, map.part_sum());

        // part 2
        assert_eq!(3, map.potential_gears.len());
        assert_eq!(467835, map.sum_gear_ratio());
    }
}
