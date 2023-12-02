use regex::Regex;

fn main() {
    let input_text = include_str!("./input1.txt");
    let result = calculate_calibration_sum(input_text);
    println!("Calibration sum: {}", result);
}

fn parse_number(word: &str) -> i64 {
    if let Ok(number) = word.parse::<i64>() {
        return number;
    }

    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

fn calculate_calibration_sum(lines: &str) -> i64 {
    let regex =
        Regex::new("(one|two|three|four|five|six|seven|eight|nine|ten|1|2|3|4|5|6|7|8|9)").unwrap();

    let mut total_sum = 0;

    for line in lines.lines() {
        let numbers: Vec<i64> = regex
            .captures_iter(line)
            .map(|capture| parse_number(capture.get(0).unwrap().as_str()))
            .collect();

        if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
            total_sum += first * 10 + last;
        }
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_calibration_sum() {
        let result: i64 = calculate_calibration_sum(
            "two1nine
             eightwothree
             abcone2threexyz
             xtwone3four
             4nineeightseven2
             zoneight234
             7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
