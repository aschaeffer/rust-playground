use std::time::Instant;
use rand::distributions::{Distribution};

pub fn roll_dices (how_many_sixes_roll: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let die = rand::distributions::Uniform::from(1..7);

    let mut found = 0;
    let mut tries = 0;
    loop {
        tries += 1;
        let throw = die.sample(&mut rng);
        if throw == 6 {
            found += 1;
        }
        if found > how_many_sixes_roll {
            break;
        }
    }
    tries
}

pub fn time_duration_tests() {
    let start = Instant::now();
    let how_many_sixes_roll = 100000;
    let tries = roll_dices(how_many_sixes_roll);
    let duration = start.elapsed();

    println!("In order to roll {} sixes you needed {} tries. Time elapsed in expensive_function() is: {:?}", how_many_sixes_roll, tries, duration);
}
