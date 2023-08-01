use crate::engine::{actions, Action};
use std::io::Write;

mod engine;

pub struct State {
    distance: u32,
}

fn walk(state: &mut State) {
    state.distance += 1;
}

actions! {
    WALK => (1, "Walk", "Walking", Some(walk)),
    RUN => (2, "Run", "Running", Some(|state| state.distance += 2)),
    END => (0, "End", "Ending", None)
}

fn main() {
    let mut state = State { distance: 0 };

    loop {
        println!("{}", MENU);
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(num) => {
                match get_action(num) {
                    Some(action) => {
                        match action.key {
                            ActionKeys::END => {
                                break;
                            }
                            _ => match action.cb {
                                Some(cb) => cb(&mut state),
                                None => {}
                            },
                        }
                        println!("{}", action.desc);
                    }
                    None => {
                        println!("Invalid input, {} is not an option", num);
                        continue;
                    }
                };
            }
            Err(_) => {
                println!("Invalid input, enter a number.");
                continue;
            }
        }

        println!("Distance: {}", state.distance);
    }
}
