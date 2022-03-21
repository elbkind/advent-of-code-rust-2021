pub mod first {

    #[allow(dead_code)]
    fn print_state(cells: &Vec<i32>) {
        for index in 0..cells.len() {
            let cell = cells[index];
            if index % 5 == 0 {
                if cell != -1 {
                    println!("{:>3}", cell);
                } else {
                    println!("   ");
                }

                if index != 0 && index % 25 == 0 {
                    println!("##########")
                }
            } else {
                if cell != -1 {
                    print!("{:>3}", cell);
                } else {
                    print!("   ");
                }
            }
        }

        println!();
    }

    pub fn mark_drawn_number(cells: &mut Vec<i32>, draw: u32) -> Option<Vec<usize>> {
        let mut result: Vec<usize> = Vec::new();
        for index in 0..cells.len() {
            if cells[index] == draw.try_into().unwrap() {
                cells[index] = -1;
                result.push(index.try_into().unwrap());
            }
        }

        if result.len() > 0 {
            return Some(result);
        }

        None
    }

    // 27, board 1, row 0, cell 2
    // 46 board 1, row 4 cell 1
    pub fn check_completion(cells: &Vec<i32>, index: usize) -> bool {
        let board_index: usize = index / 25;
        let position_in_board = index % 25;

        let row: usize = position_in_board / 5;
        let column = position_in_board % 5;

        let board_offset = board_index * 25;

        if [0, 1, 2, 3, 4]
            .iter()
            .map(|column| cells[board_offset + row * 5 + column])
            .all(|cell| cell == -1)
        {
            return true;
        }

        [0, 1, 2, 3, 4]
            .iter()
            .map(|row| cells[board_offset + row * 5 + column])
            .all(|cell| cell == -1)
    }

    fn string_to_cells(line: &str) -> Vec<i32> {
        line.split(' ')
            .filter(|slice| slice.len() > 0)
            .map(|slice| slice.trim().parse().expect("not a number"))
            .collect()
    }

    fn setup(input: &str) -> (Vec<u32>, Vec<i32>) {
        let mut cells: Vec<i32> = Vec::new();
        let lines: Vec<String> = input.lines().map(str::to_string).collect();

        let mut iterator = lines.iter();

        let draws: Vec<u32> = iterator
            .next()
            .expect("expected line with draws")
            .split(',')
            .map(|x| x.trim().parse().expect("not a number"))
            .collect();
        let _drop_empty_line = iterator.next();

        iterator.for_each(|line| match line.trim().len() {
            0 => (),
            _ => {
                for number in string_to_cells(line) {
                    cells.push(number);
                }
            }
        });

        (draws, cells)
    }

    fn find_complete_board(cells: &mut Vec<i32>, draws: Vec<u32>, last: bool) -> (usize, usize) {
        let mut win_board_counter = 0;

        for draw_index in 0..draws.len() {
            if draw_index < 4 {
                let _drop_result = mark_drawn_number(cells, draws[draw_index]);
            } else {
                let draw = draws[draw_index];

                match mark_drawn_number(cells, draw) {
                    None => (),
                    Some(matches) => {
                        for match_index in 0..matches.len() {
                            let position = matches[match_index];

                            if check_completion(&cells, position) {
                                if !last {
                                    return (draw as usize, position);
                                }
                                win_board_counter = win_board_counter + 1;
                                let is_last_board = win_board_counter == cells.len() / 25;

                                if is_last_board {
                                    return (draw as usize, position);
                                }
                            }
                        }
                    }
                }
            }
        }

        (0, 0) // wont happen
    }

    #[allow(dead_code)]
    pub fn compute_winning_board(input: &str, last: bool) -> usize {
        let (draws, mut cells) = setup(input);

        let (draw, hit) = find_complete_board(&mut cells, draws, last);

        let board_index = hit / 25;
        let offset = board_index * 25;

        let sum_unmarked = &cells[offset..offset + 25]
            .iter()
            .filter(|entry| **entry >= 0)
            .fold(0, |acc, item| acc + (*item as usize));

        println!("{} {} {} {}", draw, sum_unmarked, draw, hit);
        draw * sum_unmarked
    }
}
