extern crate csv;
extern crate sys_info;

use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;
use std::time::Instant;
use sys_info::mem_info;

pub fn calculate_median(values: &Vec<f64>) -> f64 {
    let mut sorted_values = values.clone();
    sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = sorted_values.len();
    if len % 2 == 1 {
        sorted_values[len / 2]
    } else {
        (sorted_values[len / 2 - 1] + sorted_values[len / 2]) / 2.0
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    // Record the start time
    let start_time = Instant::now();
    // Load the CSV file
    let csv_file = "../grades.csv";
    let file = File::open(csv_file)?;
    let mut rdr = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_reader(file);

    let mut values: Vec<f64> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let shape_leng: f64 = record[0].parse()?;
        values.push(shape_leng);
    }

    // Calculate and print the medians
    let median = calculate_median(&values);

    println!("Median: {}", median);

    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time);
    let mem_info = mem_info().unwrap();

    println!("Memory Usage: {}%", mem_info.total.saturating_sub(mem_info.avail) as f32 / mem_info.total as f32 * 100.0);
    println!("Time Usage: {:?}", elapsed_time);

    Ok(())
}
