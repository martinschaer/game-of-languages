use std::fmt::Display;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

// -----------------------------------------------------------------------------
// Data

enum Action {
    Walk,
    Run,
    End,
}

trait Wearable: Display {
    fn calculate_damage(&mut self, hit: f32) -> f32;
    fn resistance(&self) -> f32;
}

struct Armor {
    resistance: f32,
}

impl Wearable for Armor {
    fn resistance(&self) -> f32 {
        self.resistance
    }
    fn calculate_damage(&mut self, hit: f32) -> f32 {
        let damage = if hit > self.resistance {
            hit - self.resistance
        } else {
            0.
        };
        self.resistance -= (hit - self.resistance) * 0.1;
        self.resistance = self.resistance.max(0.);
        damage
    }
}

impl Display for Armor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Resistance: {}", self.resistance)
    }
}

struct WearableVec<'a>(&'a Vec<Box<dyn Wearable + Send + Sync>>);

impl<'a> Display for WearableVec<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut arr = Vec::new();
        for bw in self.0 {
            arr.push(format!("Resistance: {}", bw.resistance()));
        }
        write!(f, "{}", arr.join(", "))
    }
}

struct Player {
    hp: u32,
    distance: i32,
    // used to signal the enemy thread to stop
    ending: bool,
    wearables: Vec<Box<dyn Wearable + Send + Sync>>,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "HP: {} | Distance: {} | Wearables: {}",
            self.hp,
            self.distance,
            WearableVec(&self.wearables)
        )
    }
}

// -----------------------------------------------------------------------------
// Enemy thread

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
        let x = pseudo_rand(now.elapsed().as_secs_f32());
        if x > 0.5 {
            let mut damage = f32::MAX;
            for wearable in &mut player.wearables {
                let d = wearable.calculate_damage(5.);
                if d < damage {
                    damage = d;
                }
            }
            let damage = damage.floor() as u32;
            player.hp = player.hp.saturating_sub(damage);
            println!("Enemy attacked! Damage taken: {}", damage);
            print_prompt(&player);
            std::thread::sleep(std::time::Duration::from_secs_f32(x));
        }
    }
}

// -----------------------------------------------------------------------------
// Main

fn main() {
    let player = Arc::new(Mutex::new(Player {
        hp: 100,
        distance: 0,
        ending: false,
        wearables: vec![
            Box::new(Armor { resistance: 3. }),
            Box::new(Armor { resistance: 4. }),
        ],
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

// -----------------------------------------------------------------------------
// Utils

fn print_prompt(player: &Player) {
    println!("{}", player);
    println!("1: Walk, 2: Run, 0: End");
    print!("> ");
    std::io::stdout().flush().unwrap();
}

fn pseudo_rand(seed: f32) -> f32 {
    let seed = (seed * 69069.) + 1.;
    seed.cos() / 2. + 0.5
}

// -----------------------------------------------------------------------------
// Tests

#[test]
fn pseudo_rand_test() {
    for i in 0..20 {
        let x = pseudo_rand(i as f32);
        println!("{}: {}", i, x);
        assert!(x >= 0. && x < 1.);
    }
}
