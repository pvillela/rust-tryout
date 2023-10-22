use std::thread;

struct Settings {
    // ... many fields
}

const MAX_FEASIBLE_SCORE: u8 = 10;

fn example(settings: Settings) {
    thread::scope(|scope| {
        // Create a helper closure that takes the move variables as arguments.
        let fn_spawn = |score: u8| {
            let work_result = do_cool_computation(&settings, score);
            println!("{:?}", work_result);
        };

        for score in 0..MAX_FEASIBLE_SCORE {
            scope.spawn(move || fn_spawn(score));
        }
    });
}

fn do_cool_computation(_: &Settings, _: u8) {}

fn main() {
    let settings = Settings {};
    example(settings);
}
