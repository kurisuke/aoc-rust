use common::day::Day;
use std::collections::HashMap;

pub struct Day17 {}

const ROW_WIDTH: usize = 7;
type Row = [bool; ROW_WIDTH];
const CYCLE_HEIGHT: usize = 30;

struct Rock {
    shape: Shape,
    top_row: usize,
    left_col: usize,
}

#[derive(Copy, Clone, Debug)]
enum Shape {
    Bar,
    Cross,
    L,
    I,
    O,
}

const SHAPES_ORDER: [Shape; 5] = [Shape::Bar, Shape::Cross, Shape::L, Shape::I, Shape::O];

impl Rock {
    fn insert(shape: Shape, stack: &mut Vec<Row>) -> Rock {
        let rows_for_rock = match shape {
            Shape::Bar => 4,
            Shape::Cross => 6,
            Shape::L => 6,
            Shape::I => 7,
            Shape::O => 5,
        };

        let empty_on_top = empty_on_top(stack);
        let (blank_rows_to_insert, insert_offset) = if empty_on_top >= rows_for_rock {
            (0, empty_on_top - rows_for_rock)
        } else {
            (rows_for_rock - empty_on_top, 0)
        };

        for _ in 0..blank_rows_to_insert {
            stack.push([false; ROW_WIDTH]);
        }

        Rock {
            shape,
            top_row: stack.len() - 1 - insert_offset,
            left_col: 2,
        }
    }

    fn settle(self, stack: &mut [Row]) {
        match self.shape {
            Shape::Bar => {
                stack[self.top_row][self.left_col] = true;
                stack[self.top_row][self.left_col + 1] = true;
                stack[self.top_row][self.left_col + 2] = true;
                stack[self.top_row][self.left_col + 3] = true;
            }
            Shape::Cross => {
                stack[self.top_row][self.left_col + 1] = true;
                stack[self.top_row - 1][self.left_col] = true;
                stack[self.top_row - 1][self.left_col + 1] = true;
                stack[self.top_row - 1][self.left_col + 2] = true;
                stack[self.top_row - 2][self.left_col + 1] = true;
            }
            Shape::L => {
                stack[self.top_row][self.left_col + 2] = true;
                stack[self.top_row - 1][self.left_col + 2] = true;
                stack[self.top_row - 2][self.left_col] = true;
                stack[self.top_row - 2][self.left_col + 1] = true;
                stack[self.top_row - 2][self.left_col + 2] = true;
            }
            Shape::I => {
                stack[self.top_row][self.left_col] = true;
                stack[self.top_row - 1][self.left_col] = true;
                stack[self.top_row - 2][self.left_col] = true;
                stack[self.top_row - 3][self.left_col] = true;
            }
            Shape::O => {
                stack[self.top_row][self.left_col] = true;
                stack[self.top_row][self.left_col + 1] = true;
                stack[self.top_row - 1][self.left_col] = true;
                stack[self.top_row - 1][self.left_col + 1] = true;
            }
        }
    }

    fn move_down(&mut self, stack: &[Row]) -> bool {
        let cannot_move = match self.shape {
            Shape::Bar => {
                self.top_row == 0
                    || stack[self.top_row - 1][self.left_col]
                    || stack[self.top_row - 1][self.left_col + 1]
                    || stack[self.top_row - 1][self.left_col + 2]
                    || stack[self.top_row - 1][self.left_col + 3]
            }
            Shape::Cross => {
                self.top_row == 2
                    || stack[self.top_row - 3][self.left_col + 1]
                    || stack[self.top_row - 2][self.left_col]
                    || stack[self.top_row - 2][self.left_col + 2]
            }
            Shape::L => {
                self.top_row == 2
                    || stack[self.top_row - 3][self.left_col]
                    || stack[self.top_row - 3][self.left_col + 1]
                    || stack[self.top_row - 3][self.left_col + 2]
            }
            Shape::I => self.top_row == 3 || stack[self.top_row - 4][self.left_col],
            Shape::O => {
                self.top_row == 1
                    || stack[self.top_row - 2][self.left_col]
                    || stack[self.top_row - 2][self.left_col + 1]
            }
        };

        if cannot_move {
            false
        } else {
            self.top_row -= 1;
            true
        }
    }

    fn move_left(&mut self, stack: &[Row]) -> bool {
        let cannot_move = self.left_col == 0
            || match self.shape {
                Shape::Bar => stack[self.top_row][self.left_col - 1],
                Shape::Cross => {
                    stack[self.top_row][self.left_col]
                        || stack[self.top_row - 1][self.left_col - 1]
                        || stack[self.top_row - 2][self.left_col]
                }
                Shape::L => {
                    stack[self.top_row][self.left_col + 1]
                        || stack[self.top_row - 1][self.left_col + 1]
                        || stack[self.top_row - 2][self.left_col - 1]
                }
                Shape::I => {
                    stack[self.top_row][self.left_col - 1]
                        || stack[self.top_row - 1][self.left_col - 1]
                        || stack[self.top_row - 2][self.left_col - 1]
                        || stack[self.top_row - 3][self.left_col - 1]
                }
                Shape::O => {
                    stack[self.top_row][self.left_col - 1]
                        || stack[self.top_row - 1][self.left_col - 1]
                }
            };

        if cannot_move {
            false
        } else {
            self.left_col -= 1;
            true
        }
    }

    fn move_right(&mut self, stack: &[Row]) -> bool {
        let cannot_move = match self.shape {
            Shape::Bar => {
                if self.left_col > (ROW_WIDTH - 5) {
                    true
                } else {
                    stack[self.top_row][self.left_col + 4]
                }
            }
            Shape::Cross => {
                if self.left_col > (ROW_WIDTH - 4) {
                    true
                } else {
                    stack[self.top_row][self.left_col + 2]
                        || stack[self.top_row - 1][self.left_col + 3]
                        || stack[self.top_row - 2][self.left_col + 2]
                }
            }
            Shape::L => {
                if self.left_col > (ROW_WIDTH - 4) {
                    true
                } else {
                    stack[self.top_row][self.left_col + 3]
                        || stack[self.top_row - 1][self.left_col + 3]
                        || stack[self.top_row - 2][self.left_col + 3]
                }
            }
            Shape::I => {
                if self.left_col > (ROW_WIDTH - 2) {
                    true
                } else {
                    stack[self.top_row][self.left_col + 1]
                        || stack[self.top_row - 1][self.left_col + 1]
                        || stack[self.top_row - 2][self.left_col + 1]
                        || stack[self.top_row - 3][self.left_col + 1]
                }
            }
            Shape::O => {
                if self.left_col > (ROW_WIDTH - 3) {
                    true
                } else {
                    stack[self.top_row][self.left_col + 2]
                        || stack[self.top_row - 1][self.left_col + 2]
                }
            }
        };

        if cannot_move {
            false
        } else {
            self.left_col += 1;
            true
        }
    }
}

fn row_empty(row: &Row) -> bool {
    row.iter().all(|field| !(*field))
}

fn empty_on_top(stack: &[Row]) -> usize {
    if stack.is_empty() {
        0
    } else {
        let mut empty_on_top = 0;
        let mut row_idx = stack.len() - 1;

        while row_empty(&stack[row_idx]) {
            empty_on_top += 1;
            if row_idx > 0 {
                row_idx -= 1;
            } else {
                break;
            }
        }

        empty_on_top
    }
}

#[allow(dead_code)]
fn print_stack(stack: &[Row]) {
    for row in stack.iter().rev() {
        let s: String = row
            .iter()
            .map(|field| match field {
                true => '#',
                false => '.',
            })
            .collect();
        println!("{}", s);
    }
    println!();
}

fn simulate(jet_pattern: &[char], target_rocks: usize, check_cycle: bool) -> usize {
    let mut stack = vec![];
    let mut rocks_fallen = 0;

    let mut jet_it = jet_pattern.iter().cycle();
    let mut shape_it = SHAPES_ORDER.iter().cycle();

    let mut seen_states = HashMap::new();

    while rocks_fallen < target_rocks {
        let mut rock = Rock::insert(*shape_it.next().unwrap(), &mut stack);

        loop {
            match *jet_it.next().unwrap() {
                '>' => {
                    rock.move_right(&stack);
                }
                '<' => {
                    rock.move_left(&stack);
                }
                _ => unreachable!(),
            }

            if !rock.move_down(&stack) {
                rock.settle(&mut stack);
                rocks_fallen += 1;
                // print_stack(&stack);
                break;
            }
        }

        if check_cycle && stack.len() >= CYCLE_HEIGHT {
            let pattern: Vec<_> = stack[stack.len() - CYCLE_HEIGHT..stack.len() - 1].iter().flatten().cloned().collect();
            if let Some((first_cycle, first_height)) = seen_states.get(&pattern) {                
                let height_map: HashMap<usize, usize> = seen_states.iter().filter_map(|(_, (n, height))| {
                    if n >= first_cycle {
                        Some((*n - first_cycle, *height - first_height))
                    } else {
                        None
                    }
                }).collect();

                let cycle_len = rocks_fallen - first_cycle;
                let offset = first_cycle;
                let height_diff_per_cycle = stack.len() - empty_on_top(&stack) - first_height;
                
                let num_cycles = (target_rocks - offset) / cycle_len;
                let offset_in_cycle = (target_rocks - offset) % cycle_len;

                return first_height + num_cycles * height_diff_per_cycle + height_map.get(&offset_in_cycle).unwrap()
            } else {
                seen_states.insert(pattern, (rocks_fallen, stack.len() - empty_on_top(&stack)));
            }
        }
    }

    stack.len() - empty_on_top(&stack)
}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let jet_pattern: Vec<_> = input.trim().chars().collect();
        format!("{}", simulate(&jet_pattern, 2022, false))
    }

    fn star2(&self, input: &str) -> String {
        let jet_pattern: Vec<_> = input.trim().chars().collect();
        format!("{}", simulate(&jet_pattern, 1000000000000, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

        let d = Day17 {};
        assert_eq!(d.star1(input), "3068");
        assert_eq!(d.star2(input), "1514285714288");
    }
}
