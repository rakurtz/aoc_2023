use super::super::read_file;

const DAY: usize = 9; // change

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");
    let all_lines = AllLines::new_from_input(&input);

    let (result_pt1, result_pt2) = all_lines.sum_of_extrapolites();
    
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}

#[derive(Debug)]
struct Line {
    values: Vec<isize>,
    forward: Option<isize>,
    backward: Option<isize>,
}

impl Line {
    fn new_from_line(line: &str) -> Self {
        let mut values = vec![];

        for v in line.split(' ') {
            values.push(v.parse::<isize>().unwrap());
        }

        Line { values, forward: None, backward: None}
    }

    fn calculate_next_row(vector: &Vec<isize>) -> Option<Vec<isize>> {
        let mut next = vec![];
        for (a, b) in vector[..vector.len() - 1].iter().zip(vector[1..].iter()) {
            next.push(b - a);
        }

        if !next.iter().all(|x| *x == 0) {
            Some(next)
        } else {
            None
        }
    }

    fn extrapolite(&mut self) {
        let mut rows = vec![];
        rows.push(self.values.clone());

        while let Some(next) = Line::calculate_next_row(rows.last().unwrap()) {
            rows.push(next);
        }
        
        self.forward = Some(rows.iter().rev().fold(0, |akk, x| akk + x.last().unwrap()));
        self.backward = Some(rows.iter().rev().fold(0, |akk, x| x.first().unwrap() - akk));

    }
}
#[derive(Debug)]
struct AllLines {
    lines: Vec<Line>,
}

impl AllLines {
    fn new_from_input(input: &str) -> Self {
        let mut lines = vec![];

        for line in input.lines() {
            lines.push(Line::new_from_line(line));
        }

        let mut lines = AllLines { lines };
        lines.extrapolite_all();

        lines
    }

    fn extrapolite_all(&mut self) {
        self.lines
            .iter_mut()
            .for_each(|line| line.extrapolite());
    }

    fn sum_of_extrapolites(&self) -> (isize, isize) {
        (self.lines.iter().fold(0, |akk, x| akk + x.forward.unwrap()),
        self.lines.iter().fold(0, |akk, x| akk + x.backward.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_9() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let all_lines = AllLines::new_from_input(input);

        // part 1
        assert_eq!(114, all_lines.sum_of_extrapolites().0);

        // part 2
        assert_eq!(2, all_lines.sum_of_extrapolites().1);
    }
}
