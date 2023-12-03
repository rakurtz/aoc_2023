use super::super::read_file;
use regex::Regex;

pub fn run() {
    // read file to string
    let input = read_file(2).expect("Couldn't read file");

    // part of the GamesPuzzle:
    let bag_part_1 = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games = Games::new_from_input_regex_parser(&input);
    println!(
        "Day 2, part 1 - {}",
        games.sum_possible_game_ids(&bag_part_1)
    );
    println!("Day 2, part 2 - {}", games.sum_of_powers_of_minumum_bags());
}

#[derive(Debug)]
struct Games {
    games: Vec<Game>,
}

impl Games {
    fn new_from_input_split_parser(input: &str) -> Self {
        let mut games = vec![];
        for line in input.lines() {
            games.push(Game::new_from_str(line));
        }
        Games { games }
    }

    fn new_from_input_regex_parser(input: &str) -> Self {
        let mut games = vec![];
        let re_id = Regex::new(r"(?<id>\d+):").unwrap();
        let re_sets = Regex::new(r"(?<amount>\d+)\s+(?<color>\w+)(?:[,]|$)").unwrap();

        for line in input.lines() {
            let mut id = None;
            let mut sets: Vec<GameSet> = vec![];

            // game number
            if let Some(captures) = re_id.captures(line) {
                let (_, [captured_id]) = captures.extract();
                id = Some(captured_id.parse::<u32>().unwrap());
            }

            // sets
            for set in line.split(';') {
                let mut game_set = GameSet::new_empty();

                for (_, [amount, color]) in re_sets.captures_iter(set).map(|c| c.extract()) {
                    match color {
                        "red" => {
                            game_set.red = amount.parse::<u32>().unwrap();
                        }
                        "green" => {
                            game_set.green = amount.parse::<u32>().unwrap();
                        }
                        "blue" => {
                            game_set.blue = amount.parse::<u32>().unwrap();
                        }
                        _ => panic!("not a color"),
                    }
                }

                sets.push(game_set);
            }
            if let Some(id) = id {
                games.push(Game { id, sets })
            }
        }
        Games { games }
    }

    fn sum_possible_game_ids(&self, bag: &GameSet) -> u32 {
        let mut sum_ids = 0;
        for game in &self.games {
            if game.possible_for_bag(bag) {
                sum_ids += game.id;
            }
        }
        sum_ids
    }

    fn sum_of_powers_of_minumum_bags(&self) -> u32 {
        let mut sum_powers = 0;
        for game in &self.games {
            sum_powers += game.minimum_fit_bag().power();
        }
        sum_powers
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

impl Game {
    fn new_from_str(line: &str) -> Self {
        let mut values = line.split(':');
        let mut sets = vec![];

        let id = values
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let line_iter_sets = values.next().unwrap().split(';');
        for set in line_iter_sets {
            sets.push(GameSet::new_from_str(set.trim()));
        }

        Game { id, sets }
    }

    fn possible_for_bag(&self, bag: &GameSet) -> bool {
        for set in &self.sets {
            if set.blue > bag.blue || set.green > bag.green || set.red > bag.red {
                return false;
            }
        }
        true
    }

    fn minimum_fit_bag(&self) -> GameSet {
        let mut minimum_bag = GameSet::new_empty();
        for set in &self.sets {
            minimum_bag.red = minimum_bag.red.max(set.red);
            minimum_bag.green = minimum_bag.green.max(set.green);
            minimum_bag.blue = minimum_bag.blue.max(set.blue);
        }
        minimum_bag
    }
}

#[derive(Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameSet {
    fn new_from_str(set: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for amount_color in set.split(',') {
            let mut amount_color = amount_color.trim().split(' ');
            let amount = amount_color.next().unwrap();
            let color = amount_color.next().unwrap();

            match color {
                "red" => red = amount.parse::<u32>().unwrap(),
                "green" => green = amount.parse::<u32>().unwrap(),
                "blue" => blue = amount.parse::<u32>().unwrap(),
                other => panic!("not a known color: {}", other),
            }
        }

        GameSet { red, green, blue }
    }

    fn new_empty() -> Self {
        GameSet {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        // technically only useful for bags. But since bags are of type GameSet, it fits here
        let mut not_zero = vec![];
        if self.red > 0 {
            not_zero.push(self.red);
        }
        if self.green > 0 {
            not_zero.push(self.green);
        }
        if self.blue > 0 {
            not_zero.push(self.blue);
        }
        not_zero.iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_pt1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let bag = GameSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let games = Games::new_from_input_split_parser(input);

        // internal
        assert_eq!(5, games.games.len());

        // part 1
        assert_eq!(8, games.sum_possible_game_ids(&bag));

        // part 2
        assert_eq!(2286, games.sum_of_powers_of_minumum_bags());
    }

    #[test]
    fn day_2_regex_patterns() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let oneliner = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";

        let bag = GameSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let games = Games::new_from_input_regex_parser(input);

        // internal
        assert_eq!(5, games.games.len());

        // part 1
        assert_eq!(8, games.sum_possible_game_ids(&bag));

        // part 2
        assert_eq!(2286, games.sum_of_powers_of_minumum_bags());
    }
}
