use std::fs::File;
use std::str;
use std::io::Read;
use openssl::symm::{decrypt, Cipher};
use cryptopals::base64::*;

const KEY: &str = "YELLOW SUBMARINE";

fn main() -> Result<(), Box<dyn std::error::Error>>
{
    // Read the data
    let mut file = File::open("data/7.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    data.retain(|c| !c.is_whitespace());

    // Base64 decoding
    let data_decoded = hex_from_base64(&data.as_bytes()[..data.len()]);

    // Decrypt using AES-128-ECB
    let key = KEY.as_bytes();
    let data_decrypted = decrypt(Cipher::aes_128_ecb(), &key, None, &data_decoded)?;

    // Print the result
    println!("{}", str::from_utf8(&data_decrypted)?);
    
    Ok(())
}
