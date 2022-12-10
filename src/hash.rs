extern crate crypto;
extern crate rand;

use rand::Rng;

use self::crypto::digest::Digest;

pub type Sha1 = Vec<u8>;

const PEER_ID_PREFIX: &'static str = "-RC0001-";

pub fn calculate_sha1(input: &[u8]) -> Sha1 {
    let mut hasher = crypto::sha1::Sha1::new();
    hasher.input(input);

    let mut buf = vec![0; hasher.output_bytes()];
    hasher.result(&mut buf);
    buf
}

pub fn generate_peer_id() -> String {
    let mut rng = rand::thread_rng();
    let rand_chars: String = rng
        .gen_ascii_chars()
        .take(20 - PEER_ID_PREFIX.len())
        .collect();
    format!("{}{}", PEER_ID_PREFIX, rand_chars)
}
