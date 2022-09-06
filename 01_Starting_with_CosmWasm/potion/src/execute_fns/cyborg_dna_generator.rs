use sha3::{Digest, Keccak256};

pub fn cyborg_dna_generator(value: &String, dna_length: usize, dna_modulus: u8) -> Vec<u8> {
    let mut hasher = Keccak256::new();
    hasher.update(value);

    let result = hasher.finalize();
    let slice = &result[0..dna_length];
    let mut truncated = Vec::with_capacity(dna_length);

    for item in slice {
        truncated.push(item % dna_modulus);
    }

    return truncated;
}
