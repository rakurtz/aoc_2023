use std::iter::zip;

use super::super::read_file;
use regex::Regex;

const DAY: usize = 6; // change

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");
    let mut races = Races::new(&input);
    races.calculate_all_strategies();
    
    let result_pt1 = races.multiply_all_winning_strategies();
    
    let result_pt2 = "not yet implemented";
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug, Clone)]
struct Race {
    time: usize,
    distance: usize,
    winning_strategies: Vec<usize>,

}

impl Race {
    fn new(time: usize, distance: usize) -> Self {
        Race {
            time,
            distance,
            winning_strategies: Vec::new(),
        }
    }

    fn check_strategy(&mut self, push_button_ms: usize) -> bool {
        if (self.time - push_button_ms) * push_button_ms > self.distance {
            self.winning_strategies.push(push_button_ms);
            return true;
        }
        false
    }

    fn check_all_strategies(&mut self) {
        for ms in 0..self.time {
            self.check_strategy(ms);
        } 
    }
    
}

#[derive(Debug)]
struct Races {
    races: Vec<Race>,
}

impl Races {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"(\d+)").unwrap();
        let mut races = vec![];
        let mut times = vec![];
        let mut distances = vec![];

        let mut lines = input.lines();
        for captures in re.captures_iter(lines.next().unwrap()) {
            if let Some(Some(capture)) = captures.iter().next() {
                times.push(capture.as_str().parse::<usize>().unwrap());
            }
        }
        for captures in re.captures_iter(lines.next().unwrap()) {
            if let Some(Some(capture)) = captures.iter().next() {
                distances.push(capture.as_str().parse::<usize>().unwrap());
            }
        }

        for (time, distance) in zip(times, distances) {
            races.push(Race::new(time, distance));
        }

        Races {
            races
        }
    }

    fn calculate_all_strategies(&mut self) {
        self.races.iter_mut().for_each(|race| {race.check_all_strategies()});
    }

    fn multiply_all_winning_strategies(&self) -> usize {
        self.races.iter().fold(1, |akk, race|akk * race.winning_strategies.len())
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let mut races = Races::new(input);
        races.calculate_all_strategies();

        // calculate winning for first race
        let race1 = &races.races[0];
        let race2 = &races.races[1];
        

        assert_eq!(race1.winning_strategies, [2, 3, 4, 5]);

        assert_eq!(race2.winning_strategies.iter().min(), Some(&4usize));
        assert_eq!(race2.winning_strategies.iter().max(), Some(&11usize));

        assert_eq!(races.multiply_all_winning_strategies(), 288);

        // part 2
        assert_eq!(0, 0);
    }
}

