use std::collections::HashMap;
use regex::Regex;
use super::super::read_file;


const DAY: usize = 8; // change

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");
    
    let nodes = Nodes::new_from_input(&input);
    let result_pt1 = nodes.navigate();

    let result_pt2 = "not yet implemented";
    
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Node {
    id: String,
}

#[derive(Debug)]
struct Nodes {
    directions: Vec<Direction>,
    nodes: HashMap<Node, (Node, Node)>,
}

// custom iterator:

#[derive(Debug)]
struct NodesIterator<'a> {
    nodes: &'a Nodes,
    current_index: usize,
}

impl Nodes {
    fn iter(&self) -> NodesIterator {
        NodesIterator {
            nodes: self,
            current_index: 0,
        }
    }
}

impl<'a> Iterator for NodesIterator<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.nodes.directions.is_empty() {
            // Handle the case when the vector is empty
            None
        } else {
            let next_direction = &self.nodes.directions[self.current_index];

            // endlessly yielding idx 0, 1, 0, 1 for len() = 2 
            // or 0, 1, 2, 3, 4 for len() = 5
            self.current_index = (self.current_index + 1) % self.nodes.directions.len();
            Some(*next_direction)
        }
    }
}


impl Nodes {
    fn new_from_input(input: &str) -> Self {
        let re = Regex::new(r"(\w+)").unwrap();

        let mut input_lines = input.lines();
        let mut directions = vec![];

        for c in input_lines.next().unwrap().chars() {
            directions.push(match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Could not parse direction!"),
            }
            );
        }
        input_lines.next();

        let mut nodes = HashMap::new();
        

        for line in input_lines {
            let mut captured_nodes = vec![];
            for captures in re.captures_iter(line) {
                if let Some(Some(capture)) = captures.iter().next() {
                    captured_nodes.push(Node{ id: capture.as_str().to_string()});
                }
            } 
         
            nodes.insert(captured_nodes[0].clone(), (captured_nodes[1].clone(), captured_nodes[2].clone()));
        }
        
        Nodes { 
                nodes,
                directions, 
            }

    }

    fn navigate(&self) -> usize {
        let mut counter = 1usize;
        let mut direction_iterator = self.iter();

        let mut next_node = Node {id: "AAA".to_string()};
        while let Some(node) = self.next_node(&next_node, &direction_iterator.next().unwrap()) {
            next_node = node;
            counter += 1;
        }
        counter
    }

    fn next_node(&self, node: &Node, direction: &Direction) -> Option<Node> {
        
        // using the endless direction iterator via self.iter().next()
        let next_node = match direction {
            Direction::Left => self.nodes.get(node).unwrap().0.clone(),
            Direction::Right => self.nodes.get(node).unwrap().1.clone(),
        };

        if next_node.id != "ZZZ" {
            Some(next_node)
        } else {
            None
        }
    }


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_8() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let input2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let nodes = Nodes::new_from_input(input);
        
        
        // part 1
        assert_eq!(2, nodes.navigate());

        let nodes = Nodes::new_from_input(input2);
        // part 2
        assert_eq!(6, nodes.navigate());
    }
}

