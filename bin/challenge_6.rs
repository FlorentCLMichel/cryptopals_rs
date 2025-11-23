use std::fs::File;
use std::str;
use std::io::Read;
use cryptopals::base64::*;
use cryptopals::bitwise_ops::*;
use cryptopals::freq_chars::*;
use cryptopals::break_xor_cipher::*;

const MIN_KEY_SIZE: usize = 1;
const MAX_KEY_SIZE: usize = 40;
const N_BLOCKS: usize = 4;  // Number of blocks to consider to estimate the best key size

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // Read the data
    let mut file = File::open("data/6.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    data.retain(|c| !c.is_whitespace());

    // Base64 decoding
    let data_decoded = hex_from_base64(&data[..data.len()-1].as_bytes());
     
    // Determine the most likely key length
    let mut best_key_size = MIN_KEY_SIZE;
    let mut best_avg_distance = avg_hamming_distance(&data_decoded, MIN_KEY_SIZE, N_BLOCKS);
    for key_size in MIN_KEY_SIZE ..= MAX_KEY_SIZE {
        let avg_distance = avg_hamming_distance(&data_decoded, key_size, (N_BLOCKS * MAX_KEY_SIZE + key_size - 1) / key_size);
        if avg_distance < best_avg_distance {
            best_key_size = key_size;
            best_avg_distance = avg_distance;
        }
    }
    
    // Buffer to store the decrypted data
    let mut data_decrypted = vec![0; data.len()];

    for key_index in 0 .. best_key_size {
        // Get one data row
        let data_row = data_decoded.iter()
                                   .skip(key_index)
                                   .step_by(best_key_size)
                                   .map(|&x| x)
                                   .collect::<Vec<u8>>();

        // Break single-byte XOR encryption
        let data_row_decrypted = break_single_byte_xor(&data_row, &FREQ_ENGLISH);

        // Add to the result
        for i in 0 .. data_row_decrypted.len() {
            data_decrypted[i*best_key_size+key_index] = data_row_decrypted[i];
        }
    }

    // Print the decrypted data
    println!("{}", str::from_utf8(&data_decrypted)?);

    Ok(())
}
