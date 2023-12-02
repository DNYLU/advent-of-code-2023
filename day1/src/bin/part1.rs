fn main() {
    let input_text = include_str!("./input1.txt");
    let result = calculate_calibration_sum(input_text);
    dbg!(result);
}

fn calculate_calibration_sum(input: &str) -> u32 {
    let mut total_sum = 0;

    for line in input.lines() {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

        if let (Some(first), Some(last)) = (numbers.first(), numbers.last()) {
            let calibration_value = first * 10 + last;
            total_sum += calibration_value;
        }
    }

    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_calibration_sum() {
        let result: u32 = calculate_calibration_sum(
            "1abc2
             pqr3stu8vwx
             a1b2c3d4e5f
             treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
