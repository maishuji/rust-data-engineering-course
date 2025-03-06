use std::{error::Error, io, process};

#[allow(dead_code)]
fn ex_read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn ex_write_csv() -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    wtr.write_record(&["a", "b", "c"])?;
    wtr.write_record(&["1", "2", "3"])?;
    wtr.write_record(&["2", "6", "1"])?;
    wtr.write_record(&["5", "4", "2"])?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    //if let Err(e) = ex_read_csv() {
    //    println!("error running example: {}", e);
    //    process::exit(1);
    //}
    if let Err(e) = ex_write_csv() {
        println!("error running example: {}", e);
        process::exit(1);
    }

}