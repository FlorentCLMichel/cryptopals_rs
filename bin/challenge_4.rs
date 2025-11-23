use std::fs::File;
use std::str;
use std::io::Read;
use cryptopals::hex::*;
use cryptopals::freq_chars::*;
use cryptopals::break_xor_cipher::*;

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // Read the data
    let mut file = File::open("data/4.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let data = data.split('\n');
    
    // Try to break the encryption of each line assuming it has been XORed with a single byte
    // One of the rows decrypts as “Now that the party is jumping”
    for row in data {
        // Print a decrypted row only if it is valid UTF-8
        if let Ok(m) = str::from_utf8(&break_single_byte_xor(
            &decode_hex(row)
                .expect("Input decoding failed"),
            &FREQ_ENGLISH))
        { println!("{}", m); }
    }

    Ok(())
}
