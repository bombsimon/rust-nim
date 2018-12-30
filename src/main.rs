use rand::{thread_rng, Rng};
use std::env;
use std::fmt;
use std::io::{self, Write};

pub struct Board {
    stack: i32,
}

impl Board {
    pub fn new(s: i32) -> Board {
        Board { stack: s }
    }

    pub fn draw(&mut self, c: i32) -> Result<(), &'static str> {
        match c {
            _ if c < 1 => return Err("Must take at least one item"),
            _ if c > self.stack / 2 => return Err("Cannot remove more than half of the remainders"),
            _ => {
                self.stack -= c;
                return Ok(());
            }
        }
    }

    pub fn run(mut self) {
        while self.stack > 1 {
            println!("There are {} objects left in the stack", self);

            let to_draw = read_move();

            match self.draw(to_draw) {
                Ok(_) => println!("User removed {} from stack.", to_draw),
                Err(e) => {
                    println!("Could not remove from stack: {}", e);
                    continue;
                }
            }

            match self.stack {
                3 => {
                    self.draw(1).ok();
                    continue;
                }
                2 => {
                    self.draw(1).ok();
                    println!("Computer wins!");
                    break;
                }
                1 => {
                    println!("Player wins!");
                    break;
                }
                _ => (),
            }

            let computer_draw = generate_move(self.stack);
            self.draw(computer_draw).ok();
            println!("Computer removed {} from stack.", computer_draw);
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stack)
    }
}

fn generate_move(s: i32) -> i32 {
    let mut rng = thread_rng();
    let x: i32 = rng.gen_range(1, s / 2);

    return x;
}

fn read_move() -> i32 {
    print!("Draw some: ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();

    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");

    let trimmed = buffer.trim();

    match trimmed.parse::<i32>() {
        Ok(i) => return i,
        Err(msg) => {
            println!("{}", msg);
            return read_move();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument")
    }

    let s = &args[1];
    let stack = s.parse::<i32>().unwrap_or(0);
    let board = Board::new(stack);

    board.run()
}
