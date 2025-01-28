use crate::engine::{actions, Action};
use std::{io::Write, time::Instant};

mod engine;

pub struct State {
    distance: u32,
    stamina: i32,
    idle_time: Instant,
}

fn move_player(state: &mut State, distance: u32, stamina: i32) {
    if state.stamina < stamina {
        println!("You are too tired to move");
        return;
    }
    state.distance += distance;
    state.stamina -= stamina;
    state.idle_time = Instant::now();
}

actions! {
    WALK => (1, "Walk", "Walking", Some(|state| move_player(state, 1, 1))),
    RUN => (2, "Run", "Running", Some(|state| move_player(state, 3, 2))),
    JUMP => (3, "Jump", "Jumping", Some(|state| move_player(state, 2, 3))),
    END => (0, "End", "Ending", None)
}

fn main() {
    let mut state = State {
        distance: 0,
        stamina: 10,
        idle_time: Instant::now(), // now
    };

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
                        let elapsed = state.idle_time.elapsed();
                        state.stamina += elapsed.as_secs() as i32;

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
        println!("Stamina: {}", state.stamina);
    }
}
