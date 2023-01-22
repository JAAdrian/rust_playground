use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &str = "data/noisy_input_fs1000Hz.txt";

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


}
