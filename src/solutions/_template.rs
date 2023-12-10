use super::super::read_file;

const DAY: usize = 0; // change

pub fn run() {
    // read file to string
    let input = read_file(DAY).expect("Couldn't read file");
    
    let result_pt1 = "not yet implemented";
    let result_pt2 = "not yet implemented";
    println!("Day {}, part 1: {}", DAY, result_pt1);
    println!("Day {}, part 2: {}", DAY, result_pt2);
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_0() {
        let input = "";

        
        // part 1
        assert_eq!(0, 0);

        // part 2
        assert_eq!(0, 0);
    }
}

