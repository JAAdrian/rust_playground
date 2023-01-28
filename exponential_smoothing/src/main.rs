use std::fs::File;
use std::io::{BufRead, BufReader, Write};

mod smoothing;
use crate::smoothing::SmootherMethods;

const INPUT_FILE_PATH: &str = "data/noisy_input_fs1000Hz.txt";
const NAIVLY_OUTPUT_FILE_PATH: &str = "data/smoothed_output_fs1000Hz.txt";
const ALPHA_BETA_OUTPUT_FILE_PATH: &str = "data/smoothed_output_fs1000Hz.txt";
const SAMPLE_RATE: i32 = 1000;

const TIME_CONSTANT: f64 = 10e-3;

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

/// Write a float64 vector to file.
fn write_signal(filepath: &str, data: &Vec<f64>) {
    let mut file = File::create(filepath).expect("The output file could not be opened/created.");

    for sample in data {
        write!(file, "{}\n", sample).expect("Error writing to file.");
    }
}

/// Smooth noisy signal with simple exponential smoothing.
fn smooth_naivly(signal: Vec<f64>) -> Vec<f64> {
    let mut smoother = smoothing::ExponentialSmoother::new();

    smoother.set_initial_state(signal[0]);
    smoother.set_time_contant(TIME_CONSTANT);
    smoother.set_sample_rate(SAMPLE_RATE);
    smoother.setup();

    let smoothed_signal = smooth_signal(signal, smoother);
    return smoothed_signal;
}

/// Smooth a signal using the smoother object.
fn smooth_signal(signal: Vec<f64>, mut smoother: smoothing::ExponentialSmoother) -> Vec<f64> {
    let mut smoothed_signal: Vec<f64> = Vec::with_capacity(signal.len());
    for sample in signal {
        smoothed_signal.push(smoother.step(&sample));
    }
    return smoothed_signal;
}

fn main() {
    let signal = read_signal(INPUT_FILE_PATH);

    let naivly_smoothed_signal = smooth_naivly(signal);

    write_signal(NAIVLY_OUTPUT_FILE_PATH, &naivly_smoothed_signal);
}
