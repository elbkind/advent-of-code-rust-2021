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
    pub fn run(input: &str) -> Option<usize> {
        let number_of_bits: usize = match input.lines().next() {
            None => 0,
            Some(chars) => chars.len(),
        };

        if number_of_bits == 0 {
            return None;
        }

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

        Some(gamma * epsilon)
    }
}
