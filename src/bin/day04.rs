use std::fs::File;
use std::io::{
    prelude::*,
    BufReader,
};

use retain_mut::RetainMut;

const BOARD_WIDTH: usize  = 5;
const BOARD_HEIGHT: usize = 5;

#[derive(Debug, Clone, Copy)]
struct Board {
    nums: [[i32; BOARD_HEIGHT]; BOARD_WIDTH],
    marks: [[bool; BOARD_HEIGHT]; BOARD_WIDTH],
}

struct Point {
    x: usize,
    y: usize,
}

impl Board {
    fn blank() -> Board {
        Board{
            nums: [[0; BOARD_HEIGHT]; BOARD_WIDTH],
            marks: [[false; BOARD_HEIGHT]; BOARD_WIDTH],
        }
    }

    fn reset(&mut self) {
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                self.marks[x][y] = false;
            }
        }
    }

    // Mark number, return true if win
    fn mark_num(&mut self, num: i32) -> bool {
        let mut found_position = None;

        'outer:
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if self.nums[x][y] == num {
                    self.marks[x][y] = true;
                    found_position = Some(Point{x, y});
                    break 'outer;
                }
            }
        }

        if let Some(pos) = found_position {
            // Check for win
            for x in 0..BOARD_WIDTH {
                if !self.marks[x][pos.y] {
                    break
                }
                if x == BOARD_WIDTH-1 {
                    return true;
                }
            }
            for y in 0..BOARD_HEIGHT {
                if !self.marks[pos.x][y] {
                    break
                }
                if y == BOARD_HEIGHT-1 {
                    return true;
                }
            }
        }
        return false;
    }

    fn calc_score(&self, winning_num: i32) -> i32 {
        let mut score = 0;
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                if !self.marks[x][y] {
                    score += self.nums[x][y]
                }
            }
        }

        score *= winning_num;
        return score;
    }
}

fn main() {
    let file = File::open("input/04.txt").unwrap();
    let mut reader = BufReader::new(file);

    let callouts: Vec<i32> = reader.by_ref().lines().next().unwrap().unwrap().split(",")
                                   .map(|s| s.parse::<i32>().unwrap()).collect();

    let mut boards: Vec<Board> = Vec::new();

    let mut cur_row = BOARD_HEIGHT;
    let mut cur_board = Board::blank();

    for line in reader.lines() {
        let line = line.unwrap();
        if cur_row == BOARD_HEIGHT {
            cur_row = 0;
            continue;
        }

        for (idx, num) in line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).enumerate() {
            cur_board.nums[idx][cur_row] = num;
        }

        cur_row += 1;
        if cur_row == BOARD_HEIGHT {
            boards.push(cur_board);
        }
    }

    let mut winning_score = 0;

    'outer:
    for num in &callouts {
        for board in &mut boards {
            let won = board.mark_num(*num);
            if won {
                winning_score = board.calc_score(*num);
                break 'outer;
            }
        }
    }
    println!("Part 1: {}", winning_score);

    for board in &mut boards {
        board.reset();
    }

    let mut losing_score = 0;
    for num in &callouts {
        if boards.len() > 1 {
            // Only keep boards that don't win
            boards.retain_mut(|b| !b.mark_num(*num));
        } else {
            // Last board, play until win
            if boards[0].mark_num(*num) {
                losing_score = boards[0].calc_score(*num);
                break;
            }
        }
    }
    println!("Part 2: {}", losing_score)
}
