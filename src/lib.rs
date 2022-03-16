mod day_three;

#[cfg(test)]
mod tests {
    use super::day_three;

    #[test]
    fn day_three_first() {
        let input = std::fs::read_to_string("./src/day_three.txt").expect("Failed to read file");

        assert_eq!(day_three::first::run(&input), Some(2972336));
    }

    // #[test]
    // fn day_three_second() {
    //     let input = std::fs::read_to_string("./src/day_three.txt").expect("Failed to read file");

    //     assert_eq!(day_three::second::run(&input), 2972336);
    // }
}
