use clap::Parser;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashMap;
//use std::thread;

use week2::print_stats_analysis;

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
    let message = String::from("I am the message to encode and decode");
    let encrypted_msg = ex_encryption(&message);
    ex_cipher(&message);
    ex_decoding(&encrypted_msg);
    ex_threads_passing_msg();
    ex_mutex2();
    ex_arc();
    ex_rayon();
    week2::wikipedia_process_stats();
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
        data[i] += 1;
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

fn homophonic_cipher(plaintext: &str) -> (String, HashMap<char, Vec<char>>) {
    // Create a hashmap to store the mapping
    // Create a vector of all possible characters
    let mut rng: ThreadRng = rand::thread_rng();
    let mut mapping: HashMap<char, Vec<char>> = HashMap::new();
    let alphabet: Vec<char> = ('a'..='z').collect();
    // Cipher
    let mut ciphertext = String::new();

    for c in &alphabet {
        let homophones: Vec<char> = (0..rng.gen_range(2..4))
            .map(|_| rng.gen_range('a'..='z'))
            .collect();
        mapping.insert(*c, homophones);
    }

    for c in plaintext.chars() {
        if let Some(c) = c.to_lowercase().next() {
            if let Some(homophones) = mapping.get(&c) {
                if let Some(&homophones) = homophones.get(rng.gen_range(0..homophones.len())) {
                    ciphertext.push(homophones);
                } else {
                    eprintln!("Error: No homophones for character {}", c);
                }
            }
        } else {
            ciphertext.push(c);
        }
    }
    (ciphertext, mapping)
}

fn ex_encryption(text: &str) -> String {
    let encrypted = ceasar_encrypt(text, 5);
    println!("Original tex: {}, encrypted (ceasar) : {}", text, encrypted);
    encrypted
}

fn ex_cipher(message: &str) -> String {
    let (ciphertext, _mapping) = homophonic_cipher(message);
    println!(
        "Original text: {}, encrypted (homophonic): {}",
        message, ciphertext
    );
    ciphertext
}

fn ex_decoding(message: &str) {
    print_stats_analysis(message);
    let (depth, best_shift, decrypted, max_score) = week2::guessshift(&message, 26);
    println!(
        "Best shift: {} (out of {}), score: {}",
        best_shift, depth, max_score
    );
    println!("Decrypted message : {}", decrypted);
}

fn ex_threads_passing_msg() {
    use std::sync::mpsc;
    use std::thread;
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = String::from("Polo");
        tx.send(msg).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Marco: {}", received);
}

fn ex_mutex2() {
    use std::sync::Mutex;
    let data = Mutex::new(99);
    {
        let mut data_access = data.lock().unwrap();
        *data_access += 1;
    }
    println!("data: {:?}", *data.lock().unwrap());
}

fn ex_arc() {
    // Arc is used to safely share ownership of data across thread
    // It wraps types and allows them to be immutabled shared between threads.

    use std::sync::Arc;
    use std::thread;
    let data = Arc::new(5);
    for _ in 0..3 {
        let data_shared = data.clone();
        thread::spawn(move || {
            println!("{:?}", data_shared);
        });
    }
}

fn ex_rayon() {
    // Rayon is a parallelization library
    // Used to speed up operations like maps, filters, reduces ..
    use rayon::prelude::*;
    let data = vec![1, 2, 3];
    let parallel_sum: i32 = data.par_iter().map(|x| x * x).sum();

    println!("Parallel sum: {}", parallel_sum);
}
