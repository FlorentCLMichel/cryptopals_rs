/// Base64 encoding
fn encode_base64(x: &[u8]) -> Vec<u8> 
{
    x.iter().map(|&e| {
        match e {
            f if f < 26 => b'A' + f,
            f if f < 52 => b'a' + (f - 26),
            f if f < 62 => b'0' + (f - 52),
            62 => b'+',
            63 => b'/',
            _ => b'='
        }
    }).collect()
}


/// Base64 decoding
fn decode_base64(x: &[u8]) -> Vec<u8> 
{
    let char1 = b'A';
    let char2 = b'a';
    let char3 = b'0';
    x.iter().map(|&e| {
        match e {
            f if f >= char1 && f - char1 < 26 => f - char1,
            f if f >= char2 && f - char2 < 26 => f - char2 + 26,
            f if f >= char3 && f - char3 < 10 => f - char3 + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 255
        }
    }).collect()
}


/// Convert an array of bytes to base 64
fn base_256_to_base_64(x: &[u8]) -> Vec<u8>
{
    let mut y = Vec::<u8>::with_capacity(4 * (x.len() / 3) + 2 * (x.len() % 3));

    // Encode a group of 3 bytes into 4 Base64 characters
    for i in 0 .. (x.len() / 3) {
        let b0 = x[3*i];
        let b1 = x[3*i+1];
        let b2 = x[3*i+2];

        // First character: take the highest 6 bits of b0
        y.push(b0 >> 2);

        // Second character: lowest 2 bits of b0 and highest 4 bits of b1
        y.push(((b0 & 3) << 4) | (b1 >> 4));
        
        // Third character: lowest 4 bits of b1 and highest 2 bits of b2
        y.push(((b1 & 15) << 2) | (b2 >> 6));

        // Fourth character: lowest 6 bits of b2
        y.push(b2 & 63);
    }

    // Last character if the length of x is 1 mod 3
    if x.len() % 3 == 1 {
        let b0 = x[x.len() - 1];
        y.push(b0 >> 2);
        y.push((b0 & 3) << 4);
    }

    // Last two characters if the length of x is 2 mod 3
    if x.len() % 3 == 2 {
        let b0 = x[x.len() - 2];
        let b1 = x[x.len() - 1];
        y.push(b0 >> 2);
        y.push(((b0 & 3) << 4) | (b1 >> 4));
        y.push((b1 & 15) << 2);
    }

    y
}


/// Convert a base-64 array to an array of bytes
/// This function assumes `x` has a multiple of 4 bytes.
fn base_256_from_base_64(x: &[u8]) -> Vec<u8>
{
    let mut y = Vec::<u8>::with_capacity(3 * (x.len() / 4));

    // Encode a group of 4 Base64 characters into 3 bytes
    for i in 0 .. (x.len() / 4) {
        let c0 = x[4*i];
        let c1 = x[4*i+1];
        let c2 = x[4*i+2];
        let c3 = x[4*i+3];

        // First byte: c0 and the highest two bits of c1
        y.push((c0 << 2) | (c1 >> 4));

        // Second byte: lowest four bits of c1 and highest four bits of c2
        y.push(((c1 & 15) << 4) | (c2 >> 2));
        
        // Third byte: lowest two bits of c3 and c3
        y.push(((c2 & 3) << 6) | c3);
    }

    y
}


/// Convert an array of bytes to Base64 encoding
pub fn hex_to_base64(x: &[u8]) -> Vec<u8> 
{
    encode_base64(&base_256_to_base_64(x))
}


/// Convert an array of bytes encoded as Base64 to a standard array of bytes
pub fn hex_from_base64(x: &[u8]) -> Vec<u8> 
{
    base_256_from_base_64(&decode_base64(x))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::*;

    #[test] 
    fn decode_base64_1() 
    {
        let x: Vec<u8> = (0..64).collect();
        let y = decode_base64(&encode_base64(&x));
        assert_eq!(y, x);
    }

    #[test]
    fn base_256_to_base_64_1() {
        let x: Vec<u8> = vec![77, 97, 110];
        let y = base_256_to_base_64(&x);
        let exp_y: Vec<u8> = vec![19, 22, 5, 46];
        assert_eq!(exp_y, y);
    }
    
    #[test]
    fn hex_to_base64_1() {
        let x: Vec<u8> = vec![77, 97, 110];
        let y = hex_to_base64(&x);
        let exp_y: Vec<u8> = vec![84, 87, 70, 117];
        assert_eq!(exp_y, y);
    }
    
    #[test]
    fn hex_to_base64_2() {
        let x: Vec<u8> = decode_hex(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
            ).expect("Input decoding failed");
        let y = hex_to_base64(&x);
        let y = std::str::from_utf8(&y).expect("Conversion to string failed");
        let exp_y = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(exp_y, y);
    }
}
