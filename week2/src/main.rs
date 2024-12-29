use clap::Parser;
use std::thread;

#[derive(Parser)]
#[clap(
    name = "Week2",
    version = "1.0",
    author = "maishuji",
    about = "Demo for week2"
)]

struct Args {
    #[clap(short, long)]
    fruits: Option<String>,
    csvfile: Option<String>,
}

fn main() {
    ex_run_threads();
    ex_mutex();
    ex_csv();
    ex_data_race();
    ex_encryption();
}

fn ex_run_threads() {
    use std::thread;

    // Define thread task
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    // Join spawned thread
    handle.join().unwrap();
}

fn ex_mutex() {
    use std::sync::Mutex;

    // Create mutex resource
    let m = Mutex::new(5);

    {
        // Lock mutex
        let mut num = m.lock().unwrap();
        // Modify
        *num = 1000;
    }

    println!("m = {:?}", m);
}

fn csv_to_vec(csv: &str) -> Vec<String> {
    csv.split(",").map(|s| s.to_string()).collect()
}

fn display_fruits(fruits: Vec<String>) {
    for fruit in fruits {
        println!("{}", fruit);
    }
}

fn ex_csv() {
    let args: Args = Args::parse();
    let fruits = match args.csvfile {
        Some(file) => {
            let content = std::fs::read_to_string(file).expect("Cannot read file");
            csv_to_vec(&content)
        }
        None => args
            .fruits
            .unwrap_or_default()
            .split(",")
            .map(|s| s.to_string())
            .collect(),
    };
    display_fruits(fruits);
}

fn ex_data_race() {
    let mut data = vec![1, 2, 3, 4, 5];

    for i in 0..5 {
        // THis won't compile as no data race can occurs
        //thread::spawn(move || {
        //    data[i] += 1;
        //});
    }
}

fn ceasar_encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let base = if c.is_lowercase() { b'a' } else { b'A' };
                let offset = (c as u8).wrapping_sub(base) + shift;
                (base + offset % 26) as char
            } else {
                c
            }
        })
        .collect()
}

fn ex_encryption() {
    let text = "Hello, World!";
    let encrypted = ceasar_encrypt(text, 5);
    println!("Original tex: {}, encrypted (ceasar) : {}", text, encrypted);
}
