// The goal is to determine which ciphertexts in the file `8.txt` has beeen encoded with AES-128 
// in ECB mode. 
// A well-known weakness of AES in ECB mode is that a given block (here, 16 bytes) is encrypted 
// deterministically without any additional input. Repeated plaintext blocks thus produce similarly
// repeated ciphertext blocks.

use std::fs::File;
use std::io::Read;

use cryptopals::bitwise_ops::count_n_repeated_blocks;
use cryptopals::hex::decode_hex;

const BLOCK_SIZE_BYTES: usize = 16;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // Read the data
    let mut file = File::open("data/8.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    // Split the data into ciphertexts (one per line)
    let ciphertexts = data.split('\n')
                          .map(decode_hex)
                          .collect::<Result<Vec<_>,_>>()?;
   
    // For each ciphertext, we count the maximum number of times a given block of 16 bytes appears. 
    // The ciphertext with the highest maximum is likely the one which was encrypted with
    // QES-128-ECB.
    
    let mut max_repeated_blocks = 0;
    let mut index_max_repeated_blocks = 0;
    for (i, ciphertext) in ciphertexts.iter().enumerate() {
        let n_repeated_blocks = count_n_repeated_blocks(ciphertext, BLOCK_SIZE_BYTES);
        if n_repeated_blocks > max_repeated_blocks {
            index_max_repeated_blocks = i;
            max_repeated_blocks = n_repeated_blocks;
        }
    }

    println!("The most likely ciphertext to be encrypted using AES-128-ECB is the ciphertext with index {}", 
             index_max_repeated_blocks);
    println!("Maximum number of occurrences of a block: {}", max_repeated_blocks);

    Ok(())
}
