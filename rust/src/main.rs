use std::io::Write;

pub struct Action {
    key: ActionKeys,
    desc: &'static str,
    cb: Option<fn(state: &mut State)>,
}

struct State {
    distance: u32,
}

macro_rules! actions {
    ($($name:ident => ($value:expr, $label:expr, $desc:expr, $fn:expr)),*) => {
        pub enum ActionKeys {
            $($name = $value,)*
        }
        struct Actions;
        impl Actions {
            $(
                pub const $name: Action = Action {
                    key: ActionKeys::$name,
                    desc: $desc,
                    cb: $fn,
                };
            )*
        }
        pub const MENU: &'static str = concat![$(stringify!($value), ": ", $label, ", "),*];
        pub fn get_action(key: i32) -> Option<Action> {
            match key {
                $($value => Some(Actions::$name),)*
                _ => None
            }
        }
    };
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
            Ok(x) => {
                match get_action(x) {
                    Some(a) => {
                        match a.key {
                            ActionKeys::END => {
                                break;
                            }
                            _ => match a.cb {
                                Some(cb) => cb(&mut state),
                                None => {}
                            },
                        }
                        println!("{}", a.desc);
                    }
                    _ => {
                        println!("Invalid input");
                        continue;
                    }
                };
            }
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        }

        println!("Distance: {}", state.distance);
    }
}
