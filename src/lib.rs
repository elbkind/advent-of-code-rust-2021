mod day_three;

#[cfg(test)]
mod tests {
    use super::day_three;

    #[test]
    fn day_three_first() {
        let example =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        let input = std::fs::read_to_string("./src/day_three.txt").expect("Failed to read file");

        assert_eq!(day_three::first::compute_power_consumption(example), 198);
        assert_eq!(day_three::first::compute_power_consumption(&input), 2972336);
    }

    #[test]
    fn day_three_second() {
        let example =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";
        assert_eq!(day_three::second::compute_life_support_rating(example), 230);

        let input = std::fs::read_to_string("./src/day_three.txt").expect("Failed to read file");
        assert_eq!(
            day_three::second::compute_life_support_rating(&input),
            3368358
        );
    }
}
