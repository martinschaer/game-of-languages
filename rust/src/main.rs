use std::io::Write;

macro_rules! actions {
    ($($name:ident = $value:expr => ($label:expr,$desc:expr)),*) => {
        pub enum ActionKey {
            $($name = $value,)*
        }
        struct Action;
        impl Action {
            $(
                pub const $name: (ActionKey, &str) = (ActionKey::$name, $desc);
            )*
        }
        pub const MENU: &'static str = concat![$(stringify!($value), ": ", $label, ", "),*];
        pub fn get_action(key: i32) -> Option<(ActionKey, &'static str)> {
            match key {
                $($value => Some(Action::$name),)*
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
                    Some((ActionKey::END, _)) => {
                        break;
                    }
                    Some((ActionKey::WALK | ActionKey::RUN, x)) => {
                        println!("{}", x);
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
    }
}
