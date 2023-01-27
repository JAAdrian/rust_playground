use std::fs::File;
use std::io::{BufRead, BufReader};

mod smoothing;

const FILE_PATH: &str = "data/noisy_input_fs1000Hz.txt";
const SAMPLE_RATE: i32 = 1000;

const TIME_CONSTANT: f64 = 1.0;


/// Read a TXT file's context into a float64 vector.
fn read_signal(file_path: &str) -> Vec<f64> {
    let file = File::open(file_path).expect("File was not found.");
    let reader = BufReader::new(file);

    let content: Vec<f64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<f64>().unwrap())
        .collect();

    return content;
}

fn main() {
    let signal = read_signal(FILE_PATH);
    println!("{:?}", &signal[0..10]);

    let mut smoother = smoothing::ExponentialSmoother::new();

    smoother.set_initial_state(signal[0]);
    smoother.set_time_contant(TIME_CONSTANT);
    smoother.set_sample_rate(SAMPLE_RATE);
    smoother.setup();

    let mut smoothed_signal: Vec<f64> = Vec::with_capacity(signal.len());
    for sample in signal {
        smoothed_signal.push(smoother.step(sample));
    }

    println!("\n");
    println!("{:?}", &smoothed_signal[0..10]);
}
