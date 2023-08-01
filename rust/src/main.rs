use std::io::Write;

pub struct Action {
    key: ActionKeys,
    // label: &'static str,
    desc: &'static str,
}

macro_rules! actions {
    ($($name:ident = $value:expr => ($label:expr,$desc:expr)),*) => {
        pub enum ActionKeys {
            $($name = $value,)*
        }
        struct Actions;
        impl Actions {
            $(
                pub const $name: Action = Action {
                    key: ActionKeys::$name,
                    // label: $label,
                    desc: $desc,
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

actions! {
    WALK = 1 => ("Walk", "Walking"),
    RUN = 2 => ("Run", "Running"),
    END = 0 => ("End", "Ending")
}

fn main() {
    loop {
        println!("{}", MENU);
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<i32>() {
            Ok(x) => {
                match get_action(x) {
                    Some(a) => match a.key {
                        ActionKeys::END => {
                            break;
                        }
                        _ => {
                            println!("{}", a.desc);
                        }
                    },
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
    }
}
