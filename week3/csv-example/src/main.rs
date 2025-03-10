use std::{error::Error, io, process};
use std::fs::File;
use csv;

#[allow(dead_code)]
fn read_csv(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn write_csv() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["a", "b", "c"])?;
    wtr.write_record(&["1", "2", "3"])?;
    wtr.write_record(&["2", "6", "1"])?;
    wtr.write_record(&["5", "4", "2"])?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(e) = read_csv("data/data_test.csv") {
        println!("error running example: {}", e);
        process::exit(1);
    }
    if let Err(e) = write_csv() {
        println!("error running example: {}", e);
        process::exit(1);
    }

}