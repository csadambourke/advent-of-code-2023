use std::collections::HashMap;

fn main() {
    let contents = aoc::read_and_split_file::<String>("../data/day2.txt");
    println!("part 1: {}", sum_wins(contents.unwrap()));
    let contents = aoc::read_and_split_file::<String>("../data/day2.txt");
    println!("part 2: {}", multiply_colours(contents.unwrap()));
}

fn split_line(s: &str) -> Vec<&str> {
    s.split(&[':', ';', ',']).collect()
}

fn win_game(colours: HashMap<&str, usize>) -> bool {
    if (colours["red"] > 12)
        || (colours["green"] > 13)
        || (colours["blue"] > 14) {
        return false; 
    }
    true 
}

fn get_game_number(line: Vec<&str>) -> usize {
    line[0].split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap()
}

fn sum_wins(v: Vec<String>) -> usize {
    v.iter().filter(|x| x.len() > 0 && win_game(count_cubes(split_line(x))))
        .map(|x| get_game_number(split_line(x))).sum()        
}

fn count_cubes(line: Vec<&str>) -> HashMap<&str, usize> {
    let mut cubes = HashMap::from([
                              ("red", 0),
                              ("green", 0),
                              ("blue", 0)]);
    for val in &line[1..] {
        let v: Vec<&str> = val.split_whitespace().collect();
        let colour = v[1];
        let num = v[0].parse().unwrap();
        if cubes[colour] < num {
            cubes.insert(colour, num);
        }
    }
    cubes
}

fn multiply_colours(v: Vec<String>) -> usize {
    v.iter().filter(|x| x.len() > 0).map(|x| {
        let map = count_cubes(split_line(x));
        return map["red"] * map["green"] * map["blue"];
    }).sum()
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::count_cubes;
    use crate::split_line;
    use crate::win_game;
    use crate::sum_wins;
    use crate::get_game_number;
    use crate::multiply_colours;

    #[test]
    fn split_string() {
        assert_eq!(vec!["Game 1", " 3 blue", " 4 red", " 1 red", " 2 green", " 6 blue", " 2 green"],
                   split_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"));
    }

    #[test]
    fn number_of_cubes_should_be_true() {
        assert_eq!(true, win_game(count_cubes(split_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"))));
    }

    #[test]
    fn number_of_cubes_should_be_false() {
        assert_eq!(false, win_game(count_cubes(split_line("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"))));
    }

    #[test]
    fn sum_correct_lines() {
        assert_eq!(8, sum_wins(vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
                                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
                                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
                                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
                                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()]))
    }

    #[test]
    fn highest_num_per_colour() {
        assert_eq!(HashMap::from([("red", 4), ("blue", 6), ("green", 2)]),
                                count_cubes(split_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")))
    }

    #[test]
    fn zero_of_one_colour() {
        assert_eq!(HashMap::from([("red", 4), ("blue", 0), ("green", 2)]),
                                count_cubes(split_line("Game 1: 0 blue, 4 red; 1 red, 2 green, 0 blue; 2 green")))
    }

    #[test]
    fn get_number_of_game() {
        assert_eq!(1, get_game_number(split_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green")))
    }

    #[test]
    fn multiply_one_line() {
        assert_eq!(48, 
                   multiply_colours(vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()]))
    }

    #[test]
    fn multiply_multiple_lines() {
        assert_eq!(2286, multiply_colours(vec!["Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string(),
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string(),
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string(),
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()]))
    }
}
