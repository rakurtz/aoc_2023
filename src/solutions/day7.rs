use super::super::read_file;
use std::collections::HashMap;

const DAY: usize = 7; // change

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");
    let hands = Hands::new_sorted_from_input(&input);

    let result_pt1 = hands.ranked_bid_sum();
    let result_pt2 = "not yet implemented";
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Fice,
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

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    hand_type: HandType,
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
                '5' => Card::Fice,
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
            cards,
            bid,
            hand_type: HandType::Unknown,
        };
        hand.determine_hand_type();

        hand
    }

    fn determine_hand_type(&mut self) {
        let mut hash_map = HashMap::new();
        for card in &self.cards {
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
            _ => panic!("Could not parse HandType of Hand {:?}", self.cards),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type == other.hand_type {
            for (x, y) in self.cards.iter().zip(&other.cards) {
                if x != y {
                    return x.cmp(&y);
                }
            }
            panic!("identical hands: \n {:?} \n{:?}", self, other);
        } else {
            self.hand_type.cmp(&other.hand_type)
        }
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

struct Hands {
    hands: Vec<Hand>,
}

impl Hands {
    fn new_sorted_from_input(input: &str) -> Self {
        let mut hands = Hands {
            hands: input
                .lines()
                .map(|line| Hand::new_from_line(line))
                .collect(),
        };

        hands.hands.sort();

        hands
    }
    fn ranked_bid_sum(&self) -> usize {
        let mut ranked_bid_sum = 0;
        for (idx, hand) in self.hands.iter().enumerate() {
            ranked_bid_sum += (idx + 1) * hand.bid;
        }
        ranked_bid_sum
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

        let hands = Hands::new_sorted_from_input(input);

        // part 1
        assert_eq!(hands.ranked_bid_sum(), 6440);

        // part 2
        assert_eq!(0, 0);
    }
}
