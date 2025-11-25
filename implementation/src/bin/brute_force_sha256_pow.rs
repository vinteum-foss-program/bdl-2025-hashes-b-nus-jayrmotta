use rand::Rng;
use sha2::{Sha256, Digest};
use std::sync::{Arc, Mutex};

const ASCII_PRINTABLE_MIN: u8 = 32;
const ASCII_PRINTABLE_MAX: u8 = 126;
const COMMA_BYTE: u8 = 44;
const PREFIX: &str = "bitcoin";
const SUFFIX_LENGTH: usize = 8;

const TARGET1: &str = "cafe";
const TARGET2: &str = "faded";
const TARGET3: &str = "decade";

fn compute_sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    hex::encode(hasher.finalize())
}

fn random_printable_char_excluding_comma(rng: &mut rand::rngs::ThreadRng) -> char {
    loop {
        let byte = rng.gen_range(ASCII_PRINTABLE_MIN..=ASCII_PRINTABLE_MAX);
        if byte != COMMA_BYTE {
            return byte as char;
        }
    }
}

fn generate_random_suffix(rng: &mut rand::rngs::ThreadRng) -> String {
    (0..SUFFIX_LENGTH)
        .map(|_| random_printable_char_excluding_comma(rng))
        .collect()
}

fn worker_thread(
    target_prefix: &'static str,
    found_string: Arc<Mutex<Option<String>>>,
) {
    let mut rng = rand::thread_rng();

    loop {
        if found_string.lock().unwrap().is_some() {
            break;
        }

        let suffix = generate_random_suffix(&mut rng);
        let candidate = format!("{}{}", PREFIX, suffix);
        let hash = compute_sha256(&candidate);

        if hash.starts_with(target_prefix) {
            let mut found_guard = found_string.lock().unwrap();
            if found_guard.is_none() {
                *found_guard = Some(candidate);
            }
            break;
        }
    }
}

fn find_string_with_prefix(target_prefix: &'static str) -> String {
    let num_threads = num_cpus::get();
    let found_string = Arc::new(Mutex::new(None));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            std::thread::spawn({
                let found_string = Arc::clone(&found_string);
                move || {
                    worker_thread(target_prefix, found_string);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let result = found_string.lock().unwrap().take().unwrap();
    result
}

fn main() {
    let string1 = find_string_with_prefix(TARGET1);
    let string2 = find_string_with_prefix(TARGET2);
    let string3 = find_string_with_prefix(TARGET3);

    println!("{},{},{}", string1, string2, string3);
}

