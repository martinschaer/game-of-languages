use std::{
    io::Write,
    sync::{Arc, Mutex},
};

enum Action {
    Walk,
    Run,
    End,
}

struct Player {
    hp: u32,
    distance: i32,
    // used to signal the enemy thread to stop
    ending: bool,
}

fn print_prompt(player: &Player) {
    println!("HP: {} | Distance: {}", player.hp, player.distance);
    println!("1: Walk, 2: Run, 0: End");
    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn enemy_thread(player: Arc<Mutex<Player>>) {
    let now = std::time::Instant::now();
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        let mut player = player.lock().unwrap();

        // check if the player has ended the game
        if player.ending {
            break;
        }

        // pseudo random
        let x = (now.elapsed().as_secs() as f32 + player.distance as f32 + player.hp as f32).sin();
        if x > 0.75 {
            player.hp = player.hp.saturating_sub(1);
            println!("Enemy attacked!");
            print_prompt(&player);
            std::thread::sleep(std::time::Duration::from_secs_f32(x));
        }
    }
}

fn main() {
    let player = Arc::new(Mutex::new(Player {
        hp: 100,
        distance: 0,
        ending: false,
    }));

    // start the enemy thread
    let enemy_thread_handle = {
        let player = player.clone();
        std::thread::spawn(move || {
            enemy_thread(player);
        })
    };

    // render and input loop
    loop {
        print_prompt(&player.lock().unwrap());

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

        let mut player = player.lock().unwrap();
        if player.hp == 0 {
            println!("Game Over!");
            player.ending = true;
            break;
        }

        match action {
            Action::Walk => {
                player.distance += 1;
                println!("Walking");
            }
            Action::Run => {
                player.distance += 2;
                println!("Running");
            }
            Action::End => {
                // signal the enemy thread to stop
                player.ending = true;
                break;
            }
        }
    }

    // wait for the enemy thread to finish
    enemy_thread_handle.join().unwrap();
}
