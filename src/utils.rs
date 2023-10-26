use rand::Rng;
use rand::SeedableRng;

pub fn generate_random_input(length: usize) -> String 
{
    let mut rng = rand::thread_rng();
    let mut input = String::new();

    for _ in 0..length 
    {
        // Generate random ASCII characters
        let random_char: char = rng.gen_range(32..127) as u8 as char;
        input.push(random_char);
    }

    input
}

pub fn generate_random_input_with_seed(length: usize, seed: u64) -> String 
{
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut input = String::new();

    for _ in 0..length 
    {
        // Generate random ASCII characters
        let random_char: char = rng.gen_range(32..127) as u8 as char;
        input.push(random_char);
    }

    input
}

pub fn generate_random_input_with_seed_and_range(length: usize, seed: u64, min: u8, max: u8) -> String 
{
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut input = String::new();

    for _ in 0..length 
    {
        // Generate random ASCII characters
        let random_char: char = rng.gen_range(min..max) as u8 as char;
        input.push(random_char);
    }

    input
}

pub fn generate_random_input_with_range(length: usize, min: u8, max: u8) -> String 
{
    let mut rng = rand::thread_rng();
    let mut input = String::new();

    for _ in 0..length 
    {
        // Generate random ASCII characters
        let random_char: char = rng.gen_range(min..max) as u8 as char;
        input.push(random_char);
    }

    input
}

pub fn generate_random_input_with_seed_and_range_and_chars(length: usize, seed: u64, min: u8, max: u8, chars: &str) -> String 
{
    let _ = chars;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut input = String::new();

    for _ in 0..length 
    {
        // Generate random ASCII characters
        let random_char: char = rng.gen_range(min..max) as u8 as char;
        input.push(random_char);
    }

    input
}

pub fn generate_random_input_with_range_and_chars(length: usize, min: u8, max: u8, chars: &str) -> String 
{
    let _ = chars;
    let mut rng = rand::thread_rng();
    let mut input = String::new();

    for _ in 0..length 
    {
        // Generate random ASCII characters
        let random_char: char = rng.gen_range(min..max) as u8 as char;
        input.push(random_char);
    }

    input
}