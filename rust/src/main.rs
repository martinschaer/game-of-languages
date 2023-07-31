use std::io::Write;

enum ActionKey {
    WALK = 1,
    RUN = 2,
    JUMP = 3,
    END = 0,
}

struct Action;

impl Action {
    pub const WALK: (ActionKey, &str) = (ActionKey::WALK, "Walking");
    pub const RUN: (ActionKey, &str) = (ActionKey::RUN, "Running");
    pub const JUMP: (ActionKey, &str) = (ActionKey::JUMP, "Jumping");
    pub const END: (ActionKey, &str) = (ActionKey::END, "Ending");
}

fn main() {
    loop {
        println!("1: Walk, 2: Run, 0: End");
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let action = match input.trim().parse::<i32>() {
            Ok(1) => Action::WALK,
            Ok(2) => Action::RUN,
            Ok(3) => Action::JUMP,
            Ok(0) => Action::END,
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        match action.0 {
            ActionKey::WALK => println!("{}", action.1),
            ActionKey::RUN => println!("{}", action.1),
            ActionKey::JUMP => println!("{}", action.1),
            ActionKey::END => break,
        };
    }
}
