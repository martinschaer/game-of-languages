use std::io::Write;

enum Action {
    Walk,
    Run,
    End,
}

fn main() {
    loop {
        println!("1: Walk, 2: Run, 0: End");
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let action = match input.trim() {
            "1" => Action::Walk,
            "2" => Action::Run,
            "0" => Action::End,
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        match action {
            Action::Walk => println!("Walking"),
            Action::Run => println!("Running"),
            Action::End => break,
        }
    }
}
