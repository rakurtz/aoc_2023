use super::super::read_file;
use ndarray::Array2;

const DAY: usize = 10; // change

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");

    let grid = Grid::new_from_input(&input);
    
    let result_pt1 = grid.most_far_away_in_steps();
    let result_pt2 = "not yet implemented";
    
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug)]
struct Grid {
    grid: Array2<usize>,
    start: (usize, usize),
    connected: Vec<(usize, usize)>
}

impl Grid {
    fn new_from_input(input: &str) -> Self {
        let n_cols = input.lines().next().unwrap().chars().count();
        let n_rows = input.lines().count();

        let mut grid = Array2::zeros((n_rows, n_cols));
        let mut start = None;
        let mut connected = vec![];

        // encoding grid signs to numbers:
        //
        //  1 2     F 7
        //  3 4     L J
        //  
        // 10 vertical 20 horizontal
        // 0 ground

        //  |   10   is a vertical pipe connecting north and south.
        //  -   20   is a horizontal pipe connecting east and west.

        //  L   3   is a 90-degree bend connecting north and east.
        //  J   4   is a 90-degree bend connecting north and west.
        //  7   2    is a 90-degree bend connecting south and west.
        //  F   1   is a 90-degree bend connecting south and east.

        //  .   0   is ground; there is no pipe in this tile.
        
        for (row, line) in input.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c != 'S' {
                    grid[[row, col]] = match c {
                        '.' => 0,
                        '|' => 10,
                        '-' => 20,
                        'F' => 1,
                        '7' => 2,
                        'L' => 3,
                        'J' => 4,
                        _ => panic!("unknown character to parse")
                    }
                } else {
                    start = Some((row, col));
                    connected.push(start.unwrap());
                }
            }
        }
        let mut this_grid = Grid {
            grid, start: start.unwrap(), connected
        };

        this_grid.first_connected();
        while this_grid.next_connected() {
            ();
        }
        this_grid

    }

    fn first_connected(&mut self) {
        let (row, col) = self.start;

        
        if self.grid[[row -1, col -1]] == 1 {              // north east   1
            self.connected.push((row -1, col -1));

        } else if self.grid[[row -1, col]] == 10 {       // north        10
            self.connected.push((row -1, col));
        
        } else if self.grid[[row -1, col + 1]] == 2 {       // north west   2
            self.connected.push((row -1, col + 1));

        } else if self.grid[[row, col +1 ]] == 20 {       // west         20
            self.connected.push((row, col + 1));

        } else if self.grid[[row + 1, col + 1]] == 3 {       // south west   3
            self.connected.push((row + 1, col + 1));

        } else if self.grid[[row + 1, col]] == 10 {       // south        10
            self.connected.push((row + 1, col));

        } else if self.grid[[row + 1, col - 1]] == 4 {       // south east   4
            self.connected.push((row + 1, col - 1));

        } else if self.grid[[row, col -1 ]] == 20 {       // east         20
            self.connected.push((row, col - 1 ));

        }
    }

    fn next_connected(&mut self) -> bool {
        let (actual_row, actual_col) = *self.connected.iter().last().unwrap();
        let (coming_from_row, coming_from_col) = *self.connected.iter().rev().nth(1).unwrap();
        
        let next_position =  match self.grid[[actual_row, actual_col]] {
            10 =>  if coming_from_row < actual_row { (actual_row +1, actual_col) } else {(actual_row - 1, actual_col)},
            20 =>   if coming_from_col < actual_col { (actual_row, actual_col + 1) } else {(actual_row, actual_col - 1)},
            1 => if coming_from_col > actual_col { (actual_row + 1, actual_col) } else {(actual_row, actual_col + 1)},
            2 => if coming_from_col < actual_col { (actual_row + 1, actual_col) } else {(actual_row, actual_col - 1)},
            3 => if coming_from_row < actual_row { (actual_row, actual_col +1) } else {(actual_row -1, actual_col)},
            4 => if coming_from_row < actual_row { (actual_row, actual_col -1) } else {(actual_row -1, actual_col)},
            _ => {return false;}
        };

        // println!("actual {:?}, coming from {:?}, grid_sign {:?}", (actual_row, actual_col), (coming_from_row, coming_from_col), self.grid[[actual_row, actual_col]]);

        if next_position != *self.connected.first().unwrap() {
            self.connected.push(next_position);
            true
        } else {
            false
        }
    }

    fn most_far_away_in_steps(&self) -> usize {
        self.connected.len() / 2
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

        let grid = Grid::new_from_input(input);

        // part 1
        assert_eq!(4, grid.most_far_away_in_steps());

        // part 2
        assert_eq!(0, 0);
    }
}

