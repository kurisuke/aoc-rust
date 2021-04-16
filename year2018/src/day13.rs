use common::day::Day;
use std::collections::BTreeMap;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day13 {}

struct State {
    pub grid: Grid2D<char>,
    pub carts: BTreeMap<Coords, Cart>,
}

#[derive(Copy, Clone)]
struct Cart {
    pub direction: Direction,
    pub next_turn: NextTurn,
}

#[derive(Copy, Clone)]
enum NextTurn {
    Left,
    Straight,
    Right,
}

impl State {
    fn tick(&mut self, remove: bool) -> Option<Coords> {
        let mut new_carts = BTreeMap::new();
        loop {
            let cart_pos = match self.carts.keys().next() {
                None => break,
                Some(cart_pos) => *cart_pos,
            };
            let cart = self.carts.remove(&cart_pos).unwrap();
            let (new_cart_pos, new_cart) = self.move_cart(&cart_pos, &cart);
            if remove {
                #[allow(clippy::map_entry)]
                {
                    if self.carts.contains_key(&new_cart_pos) {
                        self.carts.remove(&new_cart_pos);
                    } else if new_carts.contains_key(&new_cart_pos) {
                        new_carts.remove(&new_cart_pos);
                    } else {
                        new_carts.insert(new_cart_pos, new_cart);
                    }
                }
            } else if self.carts.contains_key(&new_cart_pos)
                || new_carts.contains_key(&new_cart_pos)
            {
                return Some(new_cart_pos);
            } else {
                new_carts.insert(new_cart_pos, new_cart);
            }
        }
        self.carts = new_carts;
        None
    }

    fn move_cart(&self, cart_pos: &Coords, cart: &Cart) -> (Coords, Cart) {
        let track = self.grid.at(cart_pos).unwrap();
        match track {
            '|' => match cart.direction {
                Direction::N | Direction::S => (cart_pos.mov(cart.direction), *cart),
                _ => {
                    panic!("Invalid direction!");
                }
            },
            '-' => match cart.direction {
                Direction::E | Direction::W => (cart_pos.mov(cart.direction), *cart),
                _ => {
                    panic!("Invalid direction!");
                }
            },
            '/' => match cart.direction {
                Direction::N => (
                    cart_pos.mov(Direction::E),
                    Cart {
                        direction: Direction::E,
                        next_turn: cart.next_turn,
                    },
                ),
                Direction::S => (
                    cart_pos.mov(Direction::W),
                    Cart {
                        direction: Direction::W,
                        next_turn: cart.next_turn,
                    },
                ),
                Direction::E => (
                    cart_pos.mov(Direction::N),
                    Cart {
                        direction: Direction::N,
                        next_turn: cart.next_turn,
                    },
                ),
                Direction::W => (
                    cart_pos.mov(Direction::S),
                    Cart {
                        direction: Direction::S,
                        next_turn: cart.next_turn,
                    },
                ),
                _ => {
                    panic!("Invalid direction!");
                }
            },
            '\\' => match cart.direction {
                Direction::N => (
                    cart_pos.mov(Direction::W),
                    Cart {
                        direction: Direction::W,
                        next_turn: cart.next_turn,
                    },
                ),
                Direction::S => (
                    cart_pos.mov(Direction::E),
                    Cart {
                        direction: Direction::E,
                        next_turn: cart.next_turn,
                    },
                ),
                Direction::E => (
                    cart_pos.mov(Direction::S),
                    Cart {
                        direction: Direction::S,
                        next_turn: cart.next_turn,
                    },
                ),
                Direction::W => (
                    cart_pos.mov(Direction::N),
                    Cart {
                        direction: Direction::N,
                        next_turn: cart.next_turn,
                    },
                ),
                _ => {
                    panic!("Invalid direction!");
                }
            },
            '+' => match cart.direction {
                Direction::N => match cart.next_turn {
                    NextTurn::Left => (
                        cart_pos.mov(Direction::W),
                        Cart {
                            direction: Direction::W,
                            next_turn: NextTurn::Straight,
                        },
                    ),
                    NextTurn::Straight => (
                        cart_pos.mov(Direction::N),
                        Cart {
                            direction: Direction::N,
                            next_turn: NextTurn::Right,
                        },
                    ),
                    NextTurn::Right => (
                        cart_pos.mov(Direction::E),
                        Cart {
                            direction: Direction::E,
                            next_turn: NextTurn::Left,
                        },
                    ),
                },
                Direction::S => match cart.next_turn {
                    NextTurn::Left => (
                        cart_pos.mov(Direction::E),
                        Cart {
                            direction: Direction::E,
                            next_turn: NextTurn::Straight,
                        },
                    ),
                    NextTurn::Straight => (
                        cart_pos.mov(Direction::S),
                        Cart {
                            direction: Direction::S,
                            next_turn: NextTurn::Right,
                        },
                    ),
                    NextTurn::Right => (
                        cart_pos.mov(Direction::W),
                        Cart {
                            direction: Direction::W,
                            next_turn: NextTurn::Left,
                        },
                    ),
                },
                Direction::E => match cart.next_turn {
                    NextTurn::Left => (
                        cart_pos.mov(Direction::N),
                        Cart {
                            direction: Direction::N,
                            next_turn: NextTurn::Straight,
                        },
                    ),
                    NextTurn::Straight => (
                        cart_pos.mov(Direction::E),
                        Cart {
                            direction: Direction::E,
                            next_turn: NextTurn::Right,
                        },
                    ),
                    NextTurn::Right => (
                        cart_pos.mov(Direction::S),
                        Cart {
                            direction: Direction::S,
                            next_turn: NextTurn::Left,
                        },
                    ),
                },
                Direction::W => match cart.next_turn {
                    NextTurn::Left => (
                        cart_pos.mov(Direction::S),
                        Cart {
                            direction: Direction::S,
                            next_turn: NextTurn::Straight,
                        },
                    ),
                    NextTurn::Straight => (
                        cart_pos.mov(Direction::W),
                        Cart {
                            direction: Direction::W,
                            next_turn: NextTurn::Right,
                        },
                    ),
                    NextTurn::Right => (
                        cart_pos.mov(Direction::N),
                        Cart {
                            direction: Direction::N,
                            next_turn: NextTurn::Left,
                        },
                    ),
                },
                _ => {
                    panic!("Invalid direction!");
                }
            },
            _ => {
                panic!("Invalid track at {}: {}", cart_pos, track);
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut print_grid = self.grid.clone();
        for (cart_pos, cart) in self.carts.iter() {
            let dir_char = match cart.direction {
                Direction::N => '^',
                Direction::S => 'v',
                Direction::E => '>',
                Direction::W => '<',
                _ => {
                    panic!("Invalid direction!");
                }
            };
            print_grid.set(cart_pos, dir_char);
        }
        println!("{}\n", print_grid);
    }
}

fn parse_input(input: &str) -> State {
    let mut grid = Grid2D::new(input).unwrap();
    let mut carts = BTreeMap::new();
    for cart_pos in grid.filter(&['^', 'v', '<', '>']) {
        let cart_char = grid.at(&cart_pos).unwrap();
        match cart_char {
            '^' => {
                grid.set(&cart_pos, '|');
                carts.insert(
                    cart_pos,
                    Cart {
                        direction: Direction::N,
                        next_turn: NextTurn::Left,
                    },
                );
            }
            'v' => {
                grid.set(&cart_pos, '|');
                carts.insert(
                    cart_pos,
                    Cart {
                        direction: Direction::S,
                        next_turn: NextTurn::Left,
                    },
                );
            }
            '<' => {
                grid.set(&cart_pos, '-');
                carts.insert(
                    cart_pos,
                    Cart {
                        direction: Direction::W,
                        next_turn: NextTurn::Left,
                    },
                );
            }
            '>' => {
                grid.set(&cart_pos, '-');
                carts.insert(
                    cart_pos,
                    Cart {
                        direction: Direction::E,
                        next_turn: NextTurn::Left,
                    },
                );
            }
            _ => {
                panic!("Invalid cart char");
            }
        }
    }

    State { grid, carts }
}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let mut state = parse_input(input);
        loop {
            match state.tick(false) {
                None => {}
                Some(crash_coords) => return format!("{}", crash_coords),
            }
        }
    }

    fn star2(&self, input: &str) -> String {
        let mut state = parse_input(input);
        while state.carts.len() > 1 {
            state.tick(true);
        }
        format!("{}", state.carts.keys().next().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let input = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   "#;
        let d = Day13 {};
        assert_eq!(d.star1(input), "7,3");
    }

    #[test]
    fn star2() {
        let input = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;
        let d = Day13 {};
        assert_eq!(d.star2(input), "6,4");
    }
}
