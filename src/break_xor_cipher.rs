use crate::bitwise_ops::*;
use crate::freq_chars::*;
use std::collections::HashMap;


/// Try to break a single-byte XOR cipher
///
/// # Arguments
///
/// `cipher`: The cipher to break.
/// `freq_symbols`: The frequency of symbols in the message language.
pub fn break_single_byte_xor(cipher: &[u8], freq_symbols: &HashMap<u8, f64>) -> Vec<u8>
{
    // First try with the key 0;
    let mut best_key: u8 = 0;
    let mut best_score = squared_diff_freqs_els(&single_byte_xor(&cipher, best_key), freq_symbols);

    // Loop over the other possible keys to find the best one
    for key in 0 .. 255 {
        let score = squared_diff_freqs_els(&single_byte_xor(&cipher, key), freq_symbols);
        if score < best_score {
            best_score = score;
            best_key = key;
        }
    }
    single_byte_xor(&cipher, best_key)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::str;
    use crate::hex::*;

    #[test]
    fn break_single_byte_xor_1() {
        assert_eq!(
            "Cooking MC's like a pound of bacon",
            str::from_utf8(&break_single_byte_xor(
                &decode_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")
                    .expect("Input decoding failed (x)"),
                &FREQ_ENGLISH)).unwrap()
        );
    }
}
