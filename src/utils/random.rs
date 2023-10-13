use cosmwasm_std::Env;
use secret_toolkit::crypto::Prng;

const SEED: &[u8] = b"my_constant_seed"; // Replace with your constant seed


pub fn get_random_number(env: &Env, min: u8, max: u8) -> u8 {
    let entropy = env.block.height.to_le_bytes(); // Convert u64 to byte array
    let mut prng = Prng::new(SEED, &entropy);
    let random_bytes: [u8; 32] = prng.rand_bytes();
    let random_byte = random_bytes[0]; // Take the first byte
    let range = max - min;
    (random_byte % (range + 1)) + min
}

pub fn get_random_number_up_to(env: &Env, max: u8) -> u8 {
    let entropy = env.block.height.to_le_bytes(); // Convert u64 to byte array
    let mut prng = Prng::new(SEED, &entropy);
    let random_bytes: [u8; 32] = prng.rand_bytes();
    let random_byte = random_bytes[0]; // Take the first byte
    random_byte % (max + 1)
}




