pub mod first {
    struct Board {
        rows: Vec<Vec<i32>>,
        is_complete: bool,
        solution: usize,
    }

    impl Board {
        pub fn mark_drawn_number(&mut self, draw: usize) -> bool {
            for row_index in 0..5 {
                for column_index in 0..5 {
                    if self.rows[row_index][column_index] == (draw as i32) {
                        self.rows[row_index][column_index] = -1;

                        if self.is_column_complete(column_index) || self.is_row_complete(row_index)
                        {
                            self.is_complete = true;
                            self.solution = self.compute_solution(draw);
                        }

                        return self.is_complete;
                    }
                }
            }

            false
        }

        fn is_row_complete(&self, row_index: usize) -> bool {
            self.rows[row_index].iter().all(|cell| *cell == -1)
        }

        fn is_column_complete(&self, column_index: usize) -> bool {
            self.rows.iter().all(|row| row[column_index] == -1)
        }

        pub fn compute_solution(&self, draw: usize) -> usize {
            let sum = self
                .rows
                .iter()
                .map(|row| {
                    row.iter()
                        .filter(|cell| **cell > -1)
                        .fold(0, |col_acc, column| {
                            return col_acc + (*column as usize);
                        })
                })
                .fold(0, |acc, sum| acc + sum);

            sum * draw
        }
    }

    fn string_to_cells(line: &str) -> Vec<i32> {
        line.split(' ')
            .filter(|slice| slice.len() > 0)
            .map(|slice| slice.trim().parse().expect("not a number"))
            .collect()
    }

    fn setup(input: &str) -> (Vec<usize>, Vec<Board>) {
        let mut boards: Vec<Board> = Vec::new();
        let mut cells: Vec<i32> = Vec::new();
        let lines: Vec<String> = input.lines().map(str::to_string).collect();

        let mut iterator = lines.iter();

        let draws: Vec<usize> = iterator
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

        for board_index in 0..cells.len() / 25 {
            let mut board: Vec<Vec<i32>> = Vec::new();
            let offset = board_index * 25;
            for row_index in 0..5 {
                let start = offset + row_index * 5;
                let end = offset + row_index * 5 + 5;
                board.push(Vec::from_iter(cells[start..end].iter().cloned()));
            }

            boards.push(Board {
                rows: board,
                is_complete: false,
                solution: 0,
            });
        }

        (draws, boards)
    }

    #[allow(dead_code)]
    pub fn compute_winning_board(input: &str, use_last_complete_board: bool) -> usize {
        let (draws, mut boards) = setup(input);

        let mut last_complete_board: i32 = -1;

        for draw in draws {
            for (index, board) in boards.iter_mut().enumerate() {
                if !board.is_complete && board.mark_drawn_number(draw) {
                    if !use_last_complete_board {
                        return board.solution;
                    }

                    last_complete_board = index as i32;
                }
            }
        }

        boards[last_complete_board as usize].solution
    }
}
