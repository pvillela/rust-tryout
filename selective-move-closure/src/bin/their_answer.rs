use std::thread;

struct Settings {
    // ... many fields
}

const MAX_FEASIBLE_SCORE: u8 = 10;

fn example(settings: Settings) {
    // Shadow `settings` with its reference so that the move closure below just borrows the original `settings`.
    let settings = &settings;

    thread::scope(|scope| {
        for score in 0..MAX_FEASIBLE_SCORE {
            scope.spawn(move || {
                let work_result = do_cool_computation(&settings, score);
                println!("{:?}", work_result);
            });
        }
    });
}

fn do_cool_computation(_: &Settings, _: u8) {}

fn main() {
    let settings = Settings {};
    example(settings);
}
