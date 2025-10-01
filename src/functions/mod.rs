use rand::Rng;

// Generates a string of random uppercase letters A-Z of a given length with simulated load.
pub fn generate_long_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(length);
    for _ in 0..length {
        let char_code = rng.gen_range(65..91); // Letters A-Z
        result.push((char_code as u8 as char).to_ascii_uppercase());
        for _ in 0..1000 {
            let _ = rng.gen::<u8>(); // Empty operation for load
        }
    }
    result
}
