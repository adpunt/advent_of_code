use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;
use std::collections::HashSet;
use rayon::prelude::*;

fn main() -> Result<(), std::io::Error> {
    let file = File::open("input_day5.txt")?;
    let reader = BufReader::new(file);

    // let result = calculate_sum_written_numbers(reader)?;
    // let result = find_viable_games(reader)?;
    // let results = find_adjacent_numbers(reader)?;
    // let results = scratchcard_score(reader)?;
    // let results = scratchcard_recursive(reader)?;
    let results = almanac(reader)?;

    println!("{}", results);

    Ok(())
}

fn calculate_sum_written_numbers<R>(reader: R) -> Result<i32, std::io::Error>
where
    R: BufRead,
{
    let mut mapping = HashMap::new();
    mapping.insert("one", "1");
    mapping.insert("two", "2");
    mapping.insert("three", "3");
    mapping.insert("four", "4");
    mapping.insert("five", "5");
    mapping.insert("six", "6");
    mapping.insert("seven", "7");
    mapping.insert("eight", "8");
    mapping.insert("nine", "9");

    let mut s: i32 = 0;
    let mut v: Vec<i32> = Vec::new();
    for result_line in reader.lines() {
        let line = result_line?;

        let mut modified_line = line.clone();
        let mut modified_line2 = line.clone();

        if let Some((first_index, first_num)) = find_written_num(&line, &mapping, true) {
            let replacement = mapping.get(first_num).unwrap();
            println!("{}", replacement);
            modified_line = format!("{}{}{}{}", &line[..first_index], replacement, first_num, &line[first_index + first_num.len()..]);
            println!("{}", modified_line);

            if let Some((last_index, last_num)) = find_written_num(&modified_line, &mapping, false) {
                let replacement = mapping.get(last_num).unwrap();
                let modified_part = format!("{}{}{}", &modified_line[..last_index], replacement, &modified_line[last_index + last_num.len()..]);
                modified_line2 = format!("{}{}", modified_part, last_num);

                println!("first num: {}", first_num);
                if first_num == last_num {
                    modified_line2 = format!("{}{}{}", last_num, &modified_line, last_num);
                }
                else {
                    modified_line2 = format!("{}{}", modified_part, last_num);
                }
                println!("{}", modified_line2);
            }
        }

        let mut first_num_c: char = '0';
        let mut second_num_c: char = '0';
        for c in modified_line2.chars() {
            if c.is_numeric() {
                first_num_c = c;
                break;
            }
        }
        for c in modified_line2.chars().rev() {
            if c.is_numeric() {
                second_num_c = c;
                break;
            }
        }
        let result = format!("{}{}", first_num_c, second_num_c);
        let num_to_add = result.parse::<i32>().unwrap();
        v.push(num_to_add);
        s += num_to_add;
    }

    println!("{:?}", v);

    Ok(s)
}

fn find_written_num<'a>(line: &'a str, mapping: &'a HashMap<&str, &str>, first: bool) -> Option<(usize, &'a str)> {
    let mut indices: Vec<_> = mapping.keys().filter_map(|&written_num| {
        line.match_indices(written_num).last().map(|(index, _)| (index, written_num))
    }).collect();

    indices.sort_by_key(|&(index, _)| index);

    if indices.is_empty() {
        None
    } else if first {
        Some(indices[0])
    } else {
        Some(indices[indices.len() - 1])
    }
}

fn extract_first_number(input: &str) -> Option<i32> {
    let mut number_str = String::new();
    let mut found_digit = false;

    for c in input.chars() {
        if c.is_digit(10) {
            found_digit = true;
            number_str.push(c);
        } else if found_digit {
            break; // Stop after encountering the first non-digit character after finding a digit
        }
    }

    number_str.parse::<i32>().ok()
}

fn find_viable_games<R>(reader: R) -> Result<i32, std::io::Error> 
where
    R: BufRead,
{
    let mut s: i32 = 0;
    let mut s_power: i32 = 0;
    let num_red = 12;
    let num_green = 13;
    let num_blue = 14;

    for result_line in reader.lines() {
        let input = result_line?;

        let colon_index = input.find(':').unwrap();
        let before_colon = &input[0..colon_index].trim();
        let after_colon = &input[colon_index + 1..].trim();
        let substrings: Vec<&str> = after_colon.split(';').map(str::trim).collect();

        let pattern = r"(\d+)\s*([a-zA-Z]+)";
        let regex = Regex::new(pattern).unwrap();

        let mut possible = true;

        let mut max_red = 0;
        let mut max_blue = 0;
        let mut max_green = 0;

        for substring in substrings {

            for captures in regex.captures_iter(substring) {
                let number_str = captures.get(1).map_or("", |m| m.as_str());
                let number = number_str.parse::<i32>().unwrap();
                let word = captures.get(2).map_or("", |m| m.as_str());

                if word.contains("red") && number > max_red {
                    max_red = number;
                }
                else if word.contains("blue") && number > max_blue {
                    max_blue = number;
                }
                else if word.contains("green") && number > max_green {
                    max_green = number;
                }

                if (word.contains("red") && number > num_red) || (word.contains("blue") && number > num_blue) || (word.contains("green") && number > num_green) {
                    possible = false;
                    // uncomment this line for part 1, it won't work for part 2
                    // break;
                }
            }
        }
            
        if possible {
            let first_number = extract_first_number(&input);
            s += first_number.unwrap();
        }
        let power = max_red * max_blue * max_green;
        println!("line: {}", input);
        println!("power: {}", power);
        s_power += power;
    }
    // return s instead of s_power for pt 1
    Ok(s_power)
}

fn extract_number(line: &str, index: usize) -> Option<(i32, usize)> {
    let mut start = index;
    let mut end = index;

    while start > 0 && line.chars().nth(start - 1).unwrap().is_digit(10) {
        start -= 1;
    }

    while end < line.len() - 1 && line.chars().nth(end + 1).unwrap().is_digit(10) {
        end += 1;
    }

    let number_str: String = line.chars().skip(start).take(end - start + 1).collect();
    number_str.parse().ok().map(|num| (num, start))
}

fn engine_numbers<R>(reader: R) -> Result<i32, std::io::Error> where
    R: BufRead,
{
    let mut s: i32 = 0;
    let mut s_ratio: i32 = 0;
    let contents: Vec<_> = reader.lines().filter_map(Result::ok).collect();

    // Unwrap the Result to get the Regex instance
    let symbols_pattern = Regex::new(r"[^0-9.]").unwrap();
    let numbers_pattern = Regex::new(r"\d+").unwrap();
    let digits_pattern = Regex::new(r"\d").unwrap();
    // let gear_pattern = Regex::new(r"\*").unwrap();

    for i in 0..contents.len() {
        let mut line_above = String::new();
        let mut line_below = String::new();
        let mut symbols_above = Vec::<usize>::new();
        let mut symbols_below = Vec::<usize>::new();

        if i > 0 {
            symbols_above = symbols_pattern
                .find_iter(&line_above)
                .map(|match_| match_.start())
                .collect();
        }
        if i < contents.len()-1 {
            symbols_below = symbols_pattern
                .find_iter(&line_below)
                .map(|match_| match_.start())
                .collect();
        }
        let symbols_current: Vec<usize> = symbols_pattern
                .find_iter(&contents[i])
                .map(|match_| match_.start())
                .collect();
        let numbers: Vec<i32> = numbers_pattern
                .find_iter(&contents[i])
                .map(|match_| match_.as_str().parse::<i32>())
                .filter_map(|parsed| parsed.ok())
                .collect();
        let numbers_indices: Vec<_>= numbers_pattern
                .find_iter(&contents[i])
                .map(|match_| match_.start())
                .collect();

        if numbers.len() > 0 {
            for k in 0..numbers.len() {
                let num = numbers[k];
                let n_index = numbers_indices[k];
                let n_str = num.to_string();
                let n_length = n_str.len();
                let mut found_symbol = false;

                for j in n_index..=n_index + n_length -1  {

                    if j > 0 {
                        if symbols_above.contains(&(j-1)) || symbols_below.contains(&(j-1)) || symbols_current.contains(&(j-1)) {
                            found_symbol = true;
                        }
                    }
                    if j < contents[i].len()-1 && !found_symbol {
                        if symbols_above.contains(&(j+1)) || symbols_below.contains(&(j+1)) || symbols_current.contains(&(j+1)) {
                            found_symbol = true;
                        }
                    }
                    if !found_symbol {
                        if symbols_above.contains(&j) || symbols_below.contains(&j) {
                            found_symbol = true;
                        }
                    }
                    // println!("found symbol: {}", found_symbol);
                }
                if found_symbol {
                    println!("number: {}", num);
                    s += num;
                }
            }
        }
    }
    Ok(s)
}
fn find_adjacent_numbers<R>(reader: R) -> Result<i32, std::io::Error>
where
    R: BufRead,
{
    let contents: Vec<_> = reader.lines().filter_map(Result::ok).collect();
    let mut s: i32 = 0;

    for i in 0..contents.len() {
        let line = &contents[i];
        for (j, c) in line.chars().enumerate() {
            if c == '*' {
                // Identify adjacent positions
                let adjacent_positions = [
                    (i.wrapping_sub(1), j),     // above
                    (i, j.wrapping_sub(1)),     // left
                    (i, j + 1),                 // right
                    (i + 1, j),                 // below
                    (i.wrapping_sub(1), j.wrapping_sub(1)), // diagonal above-left
                    (i.wrapping_sub(1), j + 1), // diagonal above-right
                    (i + 1, j.wrapping_sub(1)), // diagonal below-left
                    (i + 1, j + 1),             // diagonal below-right
                ];

                let mut unique_numbers = HashSet::<(i32, usize)>::new();

                for (x, y) in adjacent_positions.iter() {
                    if let Some(line) = contents.get(*x) {
                        if let Some((number, start)) = extract_number(line, *y) {
                            unique_numbers.insert((number, start));
                        }
                    }
                }

                println!("unique_numbers: {:?}", unique_numbers);
                if unique_numbers.len() == 2 {
                    let mut product: i32 = 1;
                    for n in unique_numbers {
                        product = product * n.0;
                    }
                    s += product;
                }
            }
        }
    }

    Ok(s)
}

// Recursion
// Helper function scratchcard_score_individual(int i) which takes in an int, calculates the score (ie 4), and makes that many recursive calls scratchcard_score_individual(5), ..., scratchcard_score_individual(8)
// if no game exists with that number, return 0
// Call helper function on first one


fn scratchcard_recursive<R>(reader: R) -> Result<i32, std::io::Error>
where
    R: BufRead,
{
    let contents: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut memo: HashMap<i32, i32> = HashMap::new();
    let mut total_score = 0;

    for i in 0..contents.len() {
        let contents_clone = &contents;
        let result = scratchcard_score_individual(i as i32, contents_clone, &mut memo);
        total_score += result;
    }

    Ok(total_score)
}

fn scratchcard_score_individual(game_no: i32, contents: &Vec<String>, memo: &mut HashMap<i32, i32>) -> i32 {
    let contents_length = contents.len() as i32;
    if game_no < contents_length {
        if let Some(&result) = memo.get(&game_no) {
            return result;
        }

        let game_no_usize = game_no as usize;
        let line = &contents[game_no_usize];
        let numbers: Vec<&str> = line.split(":").collect();
        let number_groups: Vec<&str> = numbers[1].split("|").collect();
        let winning_numbers_str = number_groups[0];
        let your_numbers_str = number_groups[1];
        let winning: Vec<&str> = winning_numbers_str.split_whitespace().collect();
        let your_numbers: Vec<&str> = your_numbers_str.split_whitespace().collect();

        let score = your_numbers.iter().filter(|&n| winning.contains(n)).count() as i32;

        let mut total_score = 1;

        for j in 1..=score {
            let next_game_no = game_no + j;
            if next_game_no < contents_length {
                let result = scratchcard_score_individual(next_game_no, contents, memo);
                total_score += result;
            }
        }

        memo.insert(game_no, total_score);
        return total_score;
    }

    0
}

fn scratchcard_score<R>(reader: R) -> Result<i32, std::io::Error>
where
    R: BufRead,
{ 
    let mut s: i32 = 0;
    let contents: Vec<_> = reader.lines().filter_map(Result::ok).collect();
    for i in 0..contents.len() {
        let line = &contents[i];
        println!("line: {:?}; ", line);
        let mut count: i32 = 0;
        let mut score: i32 = 0;
        let numbers: Vec<&str> = line.split(":").collect();
        let number_groups: Vec<&str> = numbers[1].split("|").collect();
        let winning_numbers_str = number_groups[0];
        let your_numbers_str = number_groups[1];
        let winning: Vec<&str> = winning_numbers_str.split_whitespace().collect();
        let your_numbers: Vec<&str> = your_numbers_str.split_whitespace().collect();
        for n in winning {
            if your_numbers.contains(&n) {
                count += 1;
                if count > 1 {
                    score = score * 2;
                } else {
                    score = 1;
                }
            }
        }
        s += score;
    }
    Ok(s)
}

fn process_lines(contents: Vec<String>) -> Result<(Vec<HashMap::<u64, (u64, u64)>>, Vec<u64>), std::io::Error>
{
    let mut master_mapping: Vec<HashMap::<u64, (u64, u64)>> = Vec::new();
    let mut mapping: HashMap<u64, (u64, u64)> = HashMap::new();
    let numbers_pattern = Regex::new(r"\d+").unwrap();
    let seeds: Vec<u64> = numbers_pattern
                .find_iter(&contents[0])
                .map(|match_| match_.as_str().parse::<u64>())
                .filter_map(|parsed| parsed.ok())
                .collect();
    let mut line_no = 3;
    while line_no < contents.len() {
        let line = &contents[line_no];
        println!("line: {:?}", line);
        let numbers: Vec<u64> = numbers_pattern
                .find_iter(&contents[line_no])
                .map(|match_| match_.as_str().parse::<u64>())
                .filter_map(|parsed| parsed.ok())
                .collect();
        if numbers.len() > 0 {
            println!("numbers: {:?}", numbers);
            mapping.insert(numbers[1], (numbers[0], numbers[2]));
            line_no += 1
        }
        else {
            master_mapping.push(mapping);
            mapping = HashMap::new();
            line_no += 2
        }
    }
    master_mapping.push(mapping);
    Ok((master_mapping, seeds))
}

fn lookup_mapping(source: u64, mapping: &HashMap<u64, (u64, u64)>) -> Result<u64, std::io::Error>
{
    for (key, value) in mapping.iter() {
        if key <= &source && &source < &(key + value.1) {
            // println!("source: {}; start_range: {}. end_range: {:?}", source, key, key + value.1);
            // println!("value.0: {}; key: {}", value.0, key);
            return Ok(source + value.0 - key);
        }
    }
    return Ok(source);
}

fn almanac<R>(reader: R) -> Result<u64, std::io::Error>
where
    R: BufRead,
{ 
    let mut min_location: u64 = std::u64::MAX;
    let contents: Vec<_> = reader.lines().filter_map(Result::ok).collect();
    let (master_mapping, seeds): (Vec<HashMap<u64, (u64, u64)>>, Vec<u64>) = process_lines(contents)?;
    println!("master_mapping length: {}", master_mapping.len());
    let mut num_seeds: u64 = 0;
    let mut i: usize = 1;
    while i < seeds.len() {
        num_seeds += seeds[i];
        i += 2;
    }
    println!("number seeds total: {}", num_seeds);
    println!("seeds: {:?}", seeds);

    let mut i: usize = 0;
    let mut count: usize = 0;
    let mut range_val = false;
    let mut saved_seed: u64 = 0;
    let mut count: u64 = 0;
    for s in &seeds {
        if range_val {
            println!("saved seed: {}", saved_seed);
            for j in 0..*s {
                let mut source = saved_seed + j;
                // println!("new seed: {}", source);
                for mapping in &master_mapping {
                    source = lookup_mapping(source, mapping).unwrap();
                }
                // println!("new seed location: {}", source);
                if source < min_location {
                    min_location = source;
                }
                count += 1;
                if count % 1000000 == 0 {
                    println!("Percent done: {:.2}%", 100.0 * (count as f64 / num_seeds as f64));
                }
            }
            range_val = false;
        }
        else {
            saved_seed = *s;
            range_val = true;
        }
    }

    // TODO: uncomment for part 1
    // for s in seeds {
    //     let mut source = s;
    //     println!("source: {}", source);
    //     for mapping in &master_mapping {
    //         source = lookup_mapping(source, mapping.clone()).unwrap();
    //         println!("location: {}", source);
    //     }
    //     if source < min_location {
    //         min_location = source;
    //     }
    // }
    Ok(min_location)
}

// fn next_function<R>(reader: R) -> Result<i32, std::io::Error>
// where
//     R: BufRead,
// { 
//     let mut s: i32 = 0;
//     let contents: Vec<_> = reader.lines().filter_map(Result::ok).collect();
//     for i in 0..contents.len() {
//         let line = &contents[i];
//     }
//     Ok(s)
// }

// Can construct as a graph potentially 
// can re-construct hashmap to make it sorted (if alamancs too big)
