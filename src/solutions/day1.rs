use super::super::read_file;


pub fn run() {
    // read file to string
    let input = read_file(1).expect("Couldn't read file");
  

    println!("Day 1, part 1 - {}", sum_digits(input.clone()));
    // println!("Day 1, part 2 - {}", );
}

fn sum_digits(input: String) -> u32 {
    let mut sum = 0;

    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut first = 0;
        let mut last = 0;

        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                first = digit;
                break;
            }
        }

        for c in line.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                last = digit;
                break;
            }
        }

        let two_digit_number = 10*first + last;

        // warum funktioniert der iterator ansatz nicht?
        // while let Some(c) = line.chars().next() {
        //     println!("first inner while {}", c);
        //     if let Some(digit) = c.to_digit(10) {
        //         println!("first inner if, {}", digit);
        //         first = digit;
        //         break;
        //     }
        // }

        // while let Some(c) = line.chars().into_iter().rev().next() {
        //     if let Some(digit) = c.to_digit(10) {
        //         last = digit;
        //         break;
        //     }
        // }

        sum += two_digit_number;
    }

    sum
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_pt1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            .to_string();
       
        assert_eq!(142, sum_digits(input));
    }

   
}
