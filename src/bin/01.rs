use core::num;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut line_num_vec: Vec<char> = vec![];
    let mut total = 0;

    for char in input.chars() {
        if char.is_numeric() {
            line_num_vec.push(char);
        }

        if char.is_control() {
            if line_num_vec.len() > 0 {
                let first_num = line_num_vec[0];
                let last_num = line_num_vec[line_num_vec.len() - 1];

                total += (first_num.to_digit(10).unwrap() * 10) + last_num.to_digit(10).unwrap();
            }

            line_num_vec.clear();
        }
    }

    Some(total)
}

// fn find_spelled_out_digits

pub fn part_two(input: &str) -> Option<u32> {
    let mut line_num_vec: Vec<char> = vec![];
    let mut total: u32 = 0;

    let step_one = input.replace("one", "one1one");
    let step_two = step_one.replace("two", "two2two");
    let step_three = step_two.replace("three", "three3three");
    let step_four = step_three.replace("four", "four4four");
    let step_five = step_four.replace("five", "five5five");
    let step_six = step_five.replace("six", "six6six");
    let step_seven = step_six.replace("seven", "seven7seven");
    let step_eight = step_seven.replace("eight", "eight8eight");
    let step_nine = step_eight.replace("nine", "nine9nine");

    for char in step_nine.chars() {
        if char.is_numeric() {
            line_num_vec.push(char);
        } else if char.is_control() {
            if line_num_vec.len() > 0 {
                let first_num = line_num_vec[0];
                let last_num = line_num_vec[line_num_vec.len() - 1];

                total += (first_num.to_digit(10).unwrap() * 10) + last_num.to_digit(10).unwrap();
            }

            line_num_vec.clear();
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
