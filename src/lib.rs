mod day_four;
mod day_three;

#[cfg(test)]
mod tests {
    use super::day_four;
    use super::day_three;

    #[test]
    fn day_four_first() {
        let example =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";

        assert_eq!(day_four::first::compute_winning_board(example, false), 4512);

        let input = std::fs::read_to_string("./src/day_four.txt").expect("Failed to read file");
        assert_eq!(day_four::first::compute_winning_board(&input, false), 8442);
    }

    #[test]
    fn day_four_second() {
        let example =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n8  2 23  4 24\n21  9 14 16  7\n6 10  3 18  5\n1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7";

        assert_eq!(day_four::first::compute_winning_board(example, true), 1924);

        let input = std::fs::read_to_string("./src/day_four.txt").expect("Failed to read file");
        assert_eq!(day_four::first::compute_winning_board(&input, true), 4590);
    }

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
