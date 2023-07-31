use std::io::Write;


macro_rules! actions {
    ($($name:ident = $value:expr => ($label:expr,$desc:expr)),*) => {
        enum ActionKey {
            $($name = $value,)*
        }
        struct Action;
        #[allow(dead_code)]
        impl Action {
            $(
                pub const $name: (ActionKey, &str) = (ActionKey::$name, $desc);
            )*
        }
        pub const MENU: &'static str = concat![$(stringify!($value), ": ", $label, ", "),*];
        pub fn get_action_desc(key: i32) -> &'static str {
            match key {
                $($value => $desc,)*
                _ => unreachable!()
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
            Ok(0) => {
                break;
            }
            Ok(x @ 1..=2) => {
                println!("{}", get_action_desc(x));
            }
            _ => {
                println!("Invalid input");
                continue;
            }
        };
    }
}
