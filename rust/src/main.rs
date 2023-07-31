use std::io::Write;

macro_rules! actions {
    ($($name:ident = $value:expr => $desc:expr),*) => {
        enum ActionKey {
            $($name = $value,)*
        }
        struct Action;
        impl Action {
            $(
                pub const $name: (ActionKey, &str) = (ActionKey::$name, $desc);
            )*
        }
    };
}

actions! {
    WALK = 1 => "Walking",
    RUN = 2 => "Running",
    END = 0 => "Ending"
}

fn main() {
    loop {
        println!("1: Walk, 2: Run, 0: End");
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
