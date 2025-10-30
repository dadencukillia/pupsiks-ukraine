use rand::prelude::*;
use rand::rngs::OsRng;

pub const EMAIL_CODE_LETTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const EMAIL_CODE_NUMBERS: &[u8] = b"0123456789";
pub const EMAIL_TOKEN_SYMBOLS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn generate_email_code() -> String {
    let mut rng = OsRng;

    let mut gen_from_dict = |dict: &[u8], length: usize| -> String {
        (0..length).map(|_| {
            let random_index = rng.gen_range(0..dict.len());
            dict[random_index] as char
        }).collect()
    };

    format!("{}{}{}", 
        gen_from_dict(EMAIL_CODE_LETTERS, 3),
        gen_from_dict(EMAIL_CODE_NUMBERS, 3),
        gen_from_dict(EMAIL_CODE_LETTERS, 3),
    )
}

pub fn generate_code_token() -> String {
    let mut rng = OsRng;

    (0..32).map(|_| {
        let random_index = rng.gen_range(0..EMAIL_TOKEN_SYMBOLS.len());
        EMAIL_TOKEN_SYMBOLS[random_index] as char
    }).collect()
}
