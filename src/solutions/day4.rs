use super::super::read_file;
use regex::Regex;

const DAY: usize = 4;

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");

    // let result_pt1 = calculate_p1(&input);

    let pile = Pile::new(&input);
       
    let result_pt1 = pile.points_in_game();
    let result_pt2 = pile.cards_in_game();
    
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug)]
struct Card {
    id: usize,
    matching: u32,
    copies: usize,
}

impl Card {
    fn new(id: usize) -> Self {
        Card {
            id,
            matching: 0,
            copies: 1,
        }
    }

    fn add_copies(&mut self, copies: usize) {
        self.copies += copies;
    }
}

#[derive(Debug)]
struct Pile {
    pile: Vec<Card>,
}

impl Pile {
    pub fn new(input: &str) -> Self {

        let mut pile = vec![];
        for n in 1..input.lines().count()+1 {
            pile.push(Card::new(n))
        }
        
        let mut pile = Pile { pile };
        
        for line in input.lines() {
            pile.parse_card(line);
        }

        pile
    }

    pub fn points_in_game(&self) -> u32 {
        let mut points = 0;

        for card in &self.pile {
            if card.matching > 0 {
                points += 2u32.pow(card.matching - 1)
            }
        }

        points
    }

    pub fn cards_in_game(&self) -> usize {
        let mut total = 0;
        for card in &self.pile {
            total += card.copies;
        }
        total
    }

    // internal methods following 

    fn add_copies_to_next_cards(&mut self, id: usize, matching_numbers: u32, ) {
        if let Some(card) = self.get_card(id) {
            let copies = card.copies;
            
            for n in id..(id+matching_numbers as usize) {
                if let Some(next_card) = self.get_card(n+1) {
                    next_card.add_copies(copies);
                }
            }
        }
    }

    fn get_card(&mut self, id: usize) -> Option<&mut Card> {
        
        self.pile.iter_mut().find(|card| card.id == id)
        
        //// Clippy suggested to use the above instead of:
        //
        // for card in &mut self.pile {
        //     if card.id == id {
        //         return Some(card);
        //     } 
        // }
        // None
    }
     

    fn parse_card(&mut self, line: &str) {
        let mut line = line.split(':');

        let id = line.next()
                    .unwrap()
                    .split(' ')
                    .last()         // number is the last
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                
        // seperate winning nummbers from owner's numbers
        let mut seperated_numbers = line.next().unwrap().split('|');
    
        let winning = Pile::extract_numbers(seperated_numbers.next().unwrap());
        let owned = Pile::extract_numbers(seperated_numbers.next().unwrap());
    
        let mut matching = 0;
        for n in owned {
            if winning.contains(&n) {
                matching += 1;
            }
        }
        self.get_card(id).unwrap().matching = matching;
        self.add_copies_to_next_cards(id, matching);

    }


    //
    // not really a method on Pile. Should this be organized differently?
    //
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
        
        // part 2
        let pile = Pile::new(input);
        
        assert_eq!(13, pile.points_in_game());
        assert_eq!(30, pile.cards_in_game());
    }
}
