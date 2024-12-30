use rayon::prelude::*;
use std::collections::HashMap;
use wikipedia::http::default::Client;
use wikipedia::Page;
use wikipedia::Wikipedia;

fn gen_counts() -> HashMap<char, f32> {
    // Reference letter frequencies
    let mut letter_freq: HashMap<char, f32> = HashMap::new();
    letter_freq.insert('e', 12.7);
    letter_freq.insert('t', 9.1);
    letter_freq.insert('a', 8.2);
    letter_freq.insert('o', 7.5);
    letter_freq.insert('i', 7.0);
    letter_freq.insert('n', 6.7);
    letter_freq.insert('s', 6.3);
    letter_freq.insert('h', 6.1);
    letter_freq.insert('r', 6.0);
    letter_freq.insert('d', 4.3);
    letter_freq
}

pub fn stats_analysis(text: &str) -> Vec<(char, u32, f32, Option<f32>, f32)> {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let total: u32 = counts.values().sum();

    let letter_freq_map = gen_counts();
    let letter_freq_map: HashMap<char, f32> =
        letter_freq_map.iter().map(|(k, v)| (*k, *v)).collect();

    let mut results = Vec::new();
    for (letter, count) in &counts {
        let freq = (*count as f32 / total as f32) * 100.0;
        let letter_freq = letter_freq_map.get(&letter.to_ascii_lowercase()).cloned();

        let letter_freq_diff = letter_freq.map_or(0.0, |f| (freq - f).abs());
        results.push((*letter, *count, freq, letter_freq, letter_freq_diff));
    }
    results
}

pub fn print_stats_analysis(text: &str) {
    let stats = stats_analysis(text);
    for (letter, count, freq, letter_freq, letter_freq_diff) in stats {
        println!(
            "{}: {} ({}%), Letter Frequency: {} ({}%)",
            letter,
            count,
            freq,
            letter_freq.unwrap_or(0.0),
            letter_freq_diff
        );
    }
}

pub fn decrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let offset = (c as u8 - base + shift) % 26;
            result.push((base + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

pub fn guessshift(text: &str, depth: u8) -> (u8, u8, String, f32) {
    let mut max_score = 0.0;
    let mut best_shift = 0;
    let mut decrypted = String::new();

    for shift in 0..depth {
        let decrypted_text = decrypt(text, shift);
        let stats = stats_analysis(&decrypted_text);

        let mut score = 0.0;
        for (_, _, freq, letter_freq, letter_freq_diff) in stats {
            if let Some(letter_freq) = letter_freq {
                score += (1.0 - letter_freq_diff / letter_freq) * freq;
            }
        }
        println!("Shift: {}, Score: {}", shift, score);
        if score > max_score {
            max_score = score;
            best_shift = shift;
            decrypted = decrypted_text;
        }
    }
    (depth, best_shift, decrypted, max_score)
}

const PAGES: [&str; 5] = [
    "Mae Poen district",
    "Bitcoin",
    "Taiwan",
    "The Matrix",
    "Wall (play)",
];

struct ProcessedPage {
    title: String,
    data: String,
}

fn process_page(page: &Page<Client>) -> ProcessedPage {
    let title = page.get_title().unwrap();
    let content = page.get_content().unwrap();
    ProcessedPage {
        title,
        data: content,
    }
}

pub fn wikipedia_process_stats() {
    // Here, we check how long it takes to process pages
    let start = std::time::Instant::now();
    let wikipedia = Wikipedia::<Client>::default();
    let pages: Vec<_> = PAGES
        .par_iter()
        .map(|&p| wikipedia.page_from_title(p.to_string()))
        .collect();
    let processed_pages: Vec<ProcessedPage> = pages.par_iter().map(process_page).collect();
    for page in processed_pages {
        let start_page = std::time::Instant::now();

        println!("Title: {}", page.title.as_str());
        let first_sentence = page.data.split('.').next().unwrap();
        println!("First sentence: {}", first_sentence);
        let word_count = page.data.split_whitespace().count();
        println!("Word count: {}", word_count);
        println!("Page time: {:?}", start_page.elapsed());
    }

    // Statistics
    println!("Total time {:?}", start.elapsed());
    println!(
        "Avg time per page: {:?}",
        start.elapsed() / PAGES.len() as u32
    );
    println!("Total number of pages: {}", PAGES.len());
    println!("Number of threads: {}", rayon::current_num_threads());
}
