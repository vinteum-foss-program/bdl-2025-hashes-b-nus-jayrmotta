use rand::Rng;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const ASCII_PRINTABLE_MIN: u8 = 32;
const ASCII_PRINTABLE_MAX: u8 = 126;
const COMMA_BYTE: u8 = 44;
const STRING_LENGTH: usize = 8;

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

fn check_for_collision(
    hash_map: &mut HashMap<u32, String>,
    collision: &Arc<Mutex<Option<(String, String)>>>,
    random_str: String,
    hash_val: u32,
) -> bool {
    match hash_map.get(&hash_val) {
        Some(existing_str) if existing_str != &random_str => {
            let mut collision_guard = collision.lock().unwrap();
            if collision_guard.is_none() {
                *collision_guard = Some((existing_str.clone(), random_str));
            }
            true
        }
        Some(_) => false,
        None => {
            hash_map.insert(hash_val, random_str);
            false
        }
    }
}

fn worker_thread(
    hash_map: Arc<Mutex<HashMap<u32, String>>>,
    collision: Arc<Mutex<Option<(String, String)>>>,
) {
    let mut rng = rand::thread_rng();

    loop {
        if collision.lock().unwrap().is_some() {
            break;
        }

        let random_str = generate_random_string(&mut rng, STRING_LENGTH);
        let hash_val = hash_string(&random_str);

        let found_collision = {
            let mut hash_map_guard = hash_map.lock().unwrap();
            check_for_collision(&mut hash_map_guard, &collision, random_str, hash_val)
        };

        if found_collision {
            break;
        }
    }
}

fn main() {
    let num_threads = num_cpus::get();

    let hash_map = Arc::new(Mutex::new(HashMap::new()));
    let collision = Arc::new(Mutex::new(None));

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            std::thread::spawn({
                let hash_map = Arc::clone(&hash_map);
                let collision = Arc::clone(&collision);
                move || {
                    worker_thread(hash_map, collision);
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let collision_result = collision.lock().unwrap().take();
    if let Some((string1, string2)) = collision_result {
        println!("{},{}", string1, string2);
    }
}
