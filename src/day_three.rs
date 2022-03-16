pub mod first {

    fn compute_number(buffer: &Vec<i32>, positive: usize, negative: usize) -> usize {
        let mut sum: usize = 0;
        for n in buffer {
            if *n > 0 {
                sum = sum * 2 + positive
            } else {
                sum = sum * 2 + negative
            }
        }

        sum
    }

    fn compute_gamma(buffer: &Vec<i32>) -> usize {
        compute_number(&buffer, 1, 0)
    }

    fn compute_epsilon(buffer: &Vec<i32>) -> usize {
        compute_number(&buffer, 0, 1)
    }

    #[allow(dead_code)]
    pub fn compute_power_consumption(input: &str) -> usize {
        let number_of_bits: usize = input.lines().next().expect("Could not read lines").len();

        // column entry < 0 => more zeroes than ones
        // column entry > 0 => less zeroes than ones
        let mut bits_accumulate: Vec<i32> = vec![0; number_of_bits];

        for line in input.lines() {
            for (index, char) in line.chars().enumerate() {
                let value = bits_accumulate[index];

                let modifier: i32 = if char == '1' { 1 } else { -1 };

                let _drop_me = std::mem::replace(&mut bits_accumulate[index], value + modifier);
            }
        }

        let gamma = compute_gamma(&bits_accumulate);
        let epsilon = compute_epsilon(&bits_accumulate);

        gamma * epsilon
    }
}

pub mod second {

    /**
     * accepts a list of strings of same length
     * every char is either '1' or '0'
     *
     * This finds the dominant bit for a given column
     */
    fn find_dominant_bit(input: &Vec<String>, column: usize, flip: bool) -> i32 {
        let buffer: i32 =
            input
                .iter()
                .map(|line| line.chars().nth(column))
                .fold(0, |acc, chat_at_index| match chat_at_index {
                    Some('0') => acc - 1,
                    Some('1') => acc + 1,
                    _ => acc,
                });

        if flip {
            if buffer >= 0 {
                return 0;
            }

            return 1;
        }

        if buffer >= 0 {
            return 1;
        }

        0
    }

    fn compute_oxygen(lines: &Vec<String>, index: usize) -> usize {
        compute_and_filter(lines, index, false)
    }

    fn compute_co2_scrubber(lines: &Vec<String>, index: usize) -> usize {
        compute_and_filter(lines, index, true)
    }

    /**
     * Count bits in column <index> and filter lines according to flip
     *
     * flip = false removes lines with the less occuring bit
     * flip = true removes lines with the more occuring bit
     *
     * recurses until only one line is left
     */
    fn compute_and_filter(lines: &Vec<String>, index: usize, flip: bool) -> usize {
        let dominant_bit = find_dominant_bit(lines, index, flip);

        // list for next recursion step
        let filtered_lines: Vec<String> = lines
            .iter()
            .filter(|line| match line.chars().nth(index) {
                Some('0') => 0 == dominant_bit,
                Some('1') => 1 == dominant_bit,
                _ => false,
            })
            .cloned()
            .collect();

        if filtered_lines.len() > 1 {
            return compute_and_filter(&filtered_lines, index + 1, flip);
        }
        let last_line = filtered_lines.get(0).expect("No line found");

        last_line.chars().fold(0, |acc, c| match c {
            '1' => acc * 2 + 1,
            '0' => acc * 2,
            _ => acc,
        })
    }

    #[allow(dead_code)]
    pub fn compute_life_support_rating(input: &str) -> usize {
        let lines: Vec<String> = input.lines().map(str::to_string).collect();

        let co2_scrubber = compute_co2_scrubber(&lines, 0);
        let oxygen = compute_oxygen(&lines, 0);

        oxygen * co2_scrubber
    }
}
