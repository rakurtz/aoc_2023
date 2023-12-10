use super::super::read_file;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Mutex;

const DAY: usize = 7; // change

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");
    let mut hands = Hands::new_from_input(&input);

    let result_pt1 = hands.ranked_bid_sum();
    
    // part 2
    hands.jokerize();
    let result_pt2 = hands.ranked_bid_sum();

    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum HandType {
    Unknown,
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

static CARDS_WITHOUT_JACK: [Card; 12] = [
                                    Card::Two, 
                                    Card::Three, 
                                    Card::Four, 
                                    Card::Five, 
                                    Card::Six, 
                                    Card::Seven, 
                                    Card::Eight, 
                                    Card::Nine, 
                                    Card::Ten, 
                                    Card::Queen, 
                                    Card::King, 
                                    Card::Ace
                                    ];



#[derive(Debug, Eq, Clone)]
struct Hand {
    hand: Vec<Card>,
    bid: usize,
    hand_type: HandType,
    jokerized: Option<Vec<Card>>,
}

impl Hand {
    fn new_from_line(line: &str) -> Self {
        let mut cards = vec![];
        let mut line_split = line.split(' ');

        for c in line_split.next().unwrap().chars() {
            cards.push(match c {
                '2' => Card::Two,
                '3' => Card::Three,
                '4' => Card::Four,
                '5' => Card::Five,
                '6' => Card::Six,
                '7' => Card::Seven,
                '8' => Card::Eight,
                '9' => Card::Nine,
                'T' => Card::Ten,
                'J' => Card::Jack,
                'Q' => Card::Queen,
                'K' => Card::King,
                'A' => Card::Ace,
                _ => panic!("unrecognized card {:?}", c),
            });
        }

        let bid = line_split.next().unwrap().trim().parse::<usize>().unwrap();

        let mut hand = Hand {
            hand: cards,
            bid,
            hand_type: HandType::Unknown,
            jokerized: None,
        };
        hand.determine_hand_type();

        hand
    }

    fn determine_hand_type(&mut self) {
        let mut hash_map = HashMap::new();
        for card in &self.hand {
            *hash_map.entry(card).or_insert(0) += 1;
        }

        self.hand_type = match hash_map.values().fold(1, |akk, x| akk * x) {
            _ if hash_map.len() == 1 => HandType::Five,
            4 if hash_map.len() == 2 => HandType::Four,
            6 if hash_map.len() == 2 => HandType::FullHouse,
            3 if hash_map.len() == 3 => HandType::Three,
            4 if hash_map.len() == 3 => HandType::TwoPair,
            2 if hash_map.len() == 4 => HandType::OnePair,
            1 => HandType::HighCard,
            _ => panic!("Could not parse HandType of Hand {:?}", self.hand),
        }
    }

    fn jokerize_hand_type(&mut self) {
        
        // find indexes of Jack-Cards
        let j_indexes: Vec<usize> = self.hand.iter().enumerate()
        .filter_map(|(i, &val)| if val == Card::Jack { Some(i) } else { None })
        .collect();
    
        // generate all combinations of subistitute cards for the amount of jacks 
        let combinations = Hand::generate_combinations(&CARDS_WITHOUT_JACK, j_indexes.len());
        
        if j_indexes.len() > 0 {
            let other = Mutex::new(self.clone()); // using a Mutex to satisfy the borrow checker.
            
            // insert each combination of substitutes at the jack-indexes and compare hand_types
            for combination in combinations {
                for (&idx, &card) in j_indexes.iter().zip(combination.iter()) {
                    let mut _other = other.lock().unwrap();
                    _other.hand[idx] = card;
                    _other.determine_hand_type();
                    
                    if _other.hand_type > self.hand_type {
                        self.hand_type = _other.hand_type;
                        self.jokerized = Some(_other.hand.clone());
                    }
                }
            }       
        }    
    }

    fn generate_combinations<T: Clone>(input: &[T], tuple_size: usize) -> Vec<Vec<T>> {

        // with a little help of ChatGPT. Had to adjust only a bit to make it work.

        if tuple_size == 0 {
            return vec![vec![]];
        }
    
        if input.is_empty() {
            return vec![];
        }
    
        let mut result = Vec::new();
    
        for (i, &ref item) in input.iter().enumerate() {
            let mut remaining = input.to_vec();
            
            for mut combination in Hand::generate_combinations(&remaining, tuple_size - 1) {
                combination.insert(0, item.clone());
                result.push(combination);
            }
            remaining.remove(i);
        }
    
        result
    }

}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (x, y) in self.hand.iter().zip(other.hand.iter()) {
                if x != y {
                    if self.jokerized.is_some() || other.jokerized.is_some() {
                        if *x == Card::Jack {
                            return Ordering::Less;
                        } else if *y == Card::Jack {
                            return Ordering::Greater;
                        }
                    }
                    return x.cmp(&y);
                }
            }
            panic!("identical hands: \n {:?} \n{:?}", self, other);
        } 
        self.hand_type.cmp(&other.hand_type)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

#[derive(Debug)]
struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    fn new_from_input(input: &str) -> Self {
        Hands {
            hands: input
                .lines()
                .map(|line| Hand::new_from_line(line))
                .collect(),
        }

    }
    fn ranked_bid_sum(&mut self) -> usize {
        
        self.hands.sort();
        
        let mut ranked_bid_sum = 0;
        for (idx, hand) in self.hands.iter().enumerate() {
            ranked_bid_sum += (idx + 1) * hand.bid;
        }
        ranked_bid_sum
    }

    fn jokerize(&mut self) {
        self.hands.iter_mut().for_each(|hand| hand.jokerize_hand_type());
        self.hands.sort();
    }

    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_7() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        // check comparisons 
    
        assert!(Card::Two < Card::Three);
        assert!(Card::Ace > Card::King);
    
        assert!(HandType::Five > HandType::FullHouse);
        assert!(HandType::Four > HandType::FullHouse);
    
        let mut hands = Hands::new_from_input(input);

        // part 1
        assert_eq!(hands.ranked_bid_sum(), 6440);
        
        // part 2
        hands.jokerize();

        assert_eq!(hands.ranked_bid_sum(), 5905); 
    }
}
