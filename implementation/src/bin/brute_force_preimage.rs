use rand::Rng;
use std::sync::{Arc, Mutex};

const ASCII_PRINTABLE_MIN: u8 = 32;
const ASCII_PRINTABLE_MAX: u8 = 126;
const COMMA_BYTE: u8 = 44;
const STRING_LENGTH: usize = 8;
const TARGET_STRING: &str = "jayr";

fn hash_string(s: &str) -> u32 {
    s.bytes()
        .fold(0u32, |hash, byte| {
            ((hash << 5).wrapping_sub(hash).wrapping_add(byte as u32)) & 0xFFFFFFFF
        })
}

fn random_printable_char_excluding_comma(rng: &mut rand::rngs::ThreadRng) -> char {
    loop {
        let byte = rng.gen_range(ASCII_PRINTABLE_MIN..=ASCII_PRINTABLE_MAX);
        if byte != COMMA_BYTE {
            return byte as char;
        }
    }
}

fn generate_random_string(rng: &mut rand::rngs::ThreadRng, length: usize) -> String {
    (0..length)
        .map(|_| random_printable_char_excluding_comma(rng))
        .collect()
}

fn worker_thread(
    target_hash: u32,
    found_string: Arc<Mutex<Option<String>>>,
) {
    let mut rng = rand::thread_rng();

    loop {
        if found_string.lock().unwrap().is_some() {
            break;
        }

        let random_str = generate_random_string(&mut rng, STRING_LENGTH);
        let hash_val = hash_string(&random_str);

        if hash_val == target_hash && random_str != TARGET_STRING {
            let mut found_guard = found_string.lock().unwrap();
            if found_guard.is_none() {
                *found_guard = Some(random_str);
            }
            break;
        }
    }
}

fn main() {
    let target_hash = hash_string(TARGET_STRING);
    let num_threads = num_cpus::get();
    let found_string = Arc::new(Mutex::new(None));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            std::thread::spawn({
                let found_string = Arc::clone(&found_string);
                move || {
                    worker_thread(target_hash, found_string);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let result = found_string.lock().unwrap().take();
    if let Some(found_str) = result {
        println!("{},{}", TARGET_STRING, found_str);
    } else {
        std::process::exit(1);
    }
}

