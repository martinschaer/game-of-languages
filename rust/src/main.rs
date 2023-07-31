use std::io::Write;

macro_rules! actions {
    ($($name:ident = $value:expr => ($label:expr,$desc:expr)),*) => {
        enum ActionKey {
            $($name = $value,)*
        }
        struct Action;
        impl Action {
            $(
                pub const $name: (ActionKey, &str) = (ActionKey::$name, $desc);
            )*
        }
        pub const MENU: &'static str = concat![$(stringify!($value), ": ", $label, ", "),*];
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
            Ok(1) => println!("{}", Action::WALK.1),
            Ok(2) => println!("{}", Action::RUN.1),
            Ok(0) => {
                println!("{}", Action::END.1);
                break;
            }
            _ => {
                println!("Invalid input");
                continue;
            }
        };
    }
}
