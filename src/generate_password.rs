use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const SPECIAL_CHARS: [char; 14] = [
    '!', '#', '$', '%', '&', '*', ']', '[', '(', ')', '{', '}', '+', '-',
];

pub fn generate_password(length: u32, numbers: bool, symbols: bool, uppercase: bool) -> String {
    let mut chars: Vec<char> = Vec::with_capacity(86);

    for i in b'a'..b'z' + 1 {
        chars.push(i as char);
    }

    if numbers {
        for i in b'0'..b'9' + 1 {
            chars.push(i as char);
            chars.push(i as char);
        }
    }

    if symbols {
        chars.append(&mut SPECIAL_CHARS.to_vec());
    }

    if uppercase {
        for i in b'A'..b'Z' + 1 {
            chars.push(i as char);
        }
    }

    let seed = [0u8; 32];
    let range = Uniform::new(0, chars.len());

    let mut password = String::new();
    let mut rng = StdRng::from_seed(seed);

    for _ in 0..length {
        password.push(chars[rng.sample(range)]);
    }

    password
}
