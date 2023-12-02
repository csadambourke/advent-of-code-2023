use std::collections::HashMap;

fn main() {
    let contents = aoc::read_and_split_file::<String>("../data/day1.txt");
    println!("part 1 solution: {}", sum_lines(contents.unwrap(), "int only"));
    let contents = aoc::read_and_split_file::<String>("../data/day1.txt");
    println!("part 2 solution: {}", sum_lines(contents.unwrap(), "all"));
}

fn filter_line(s: String, param: &str) -> usize {
    let mut updated_s = s;
    if param == "all" {
        updated_s = replace_str_with_int(updated_s);
    }
    let num: Vec<char> = updated_s.chars().filter(|x| x.is_numeric()).collect();
    if num.len() == 0 {
        return 0;
    }
    format!("{}{}", num[0], num[num.len()-1]).parse().unwrap()
}

fn sum_lines(v: Vec<String>, param: &str) -> usize {
    v.iter().map(|s| filter_line(s.to_string(), param)).sum()
}
fn replace_str_with_int(s: String) -> String {   
    let numbers = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut result = s.clone();
    for (num, val) in numbers {
        result = result.replace(&num.to_string(), &(num.to_string() + &val + num));
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    use crate::filter_line;
    use crate::sum_lines;

    #[test]
    fn filter_single_line() {
        assert_eq!(12, filter_line("1abc2".to_string()));
    }

    #[test]
    fn no_numbers_in_line() {
        assert_eq!(0, filter_line("abcde".to_string()));
    }

    #[test]
    fn single_number_in_line() {
        assert_eq!(88, filter_line("abv8".to_string()));
    }

    #[test]
    fn more_than_two_numbers_in_line() {
        assert_eq!(19, filter_line("1ab8cde9hf".to_string()));
    }

    #[test]
    fn filter_multiple_lines() {
        assert_eq!(142, sum_lines(vec!["1abc2".to_string(),
                                 "pqr3stu8vwx".to_string(),
                                 "a1b2c3d4e5f".to_string(),
                                 "treb7uchet".to_string()]));
    }

    #[test]
    fn single_line_with_text() {
        assert_eq!(29, filter_line("two1nine".to_string()));
    }

    #[test]
    fn single_line_with_overlapping_text_at_start() {
        assert_eq!(24, filter_line("xtwone3four".to_string()));
    }

    #[test]
    fn single_line_with_overlapping_text_at_end() {
        assert_eq!(41, filter_line("xfour3twone".to_string()));
    }

    #[test]
    fn multi_lines_with_text() {
        assert_eq!(281,sum_lines(vec!["two1nine".to_string(),
                                "eightwothree".to_string(),
                                "abcone2threexyz".to_string(),
                                "xtwone3four".to_string(),
                                "4nineeightseven2".to_string(),
                                "zoneight234".to_string(),
                                "7pqrstsixteen".to_string()]));
    }
}
