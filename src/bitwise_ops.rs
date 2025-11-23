/// Xor two sequences of bytes. 
///
/// The output has the same length as the shortest input.
pub fn xor(x: &[u8], y: &[u8]) -> Vec<u8>
{
    x.iter().zip(y.iter()).map(
        |(e,f)| e ^ f
    ).collect()
}


/// Xor one byte sequence with a single byte
pub fn single_byte_xor(x: &[u8], y: u8) -> Vec<u8>
{
    x.iter().map(|e| e ^y).collect()
}


/// Xor a ‘message’ with a repeated ‘key’, encoded as sequences of bytes
fn repeated_xor(message: &[u8], key: &[u8]) -> Vec<u8> 
{
    message.iter()
           .zip(key.iter().cycle())
           .map(|(x,y)| x ^ y)
           .collect()
}


/// Xor a ‘message’ with a repeated ‘key’
pub fn repeated_xor_str(message: &str, key: &str) -> Vec<u8> 
{
    repeated_xor(message.as_bytes(), key.as_bytes())
}


/// Return the number of differing bits between two sequences of bytes. 
/// If their lengths are different, the longer sequence is truncated.
fn edit_distance(lhs: &[u8], rhs: &[u8]) -> u32
{
    lhs.iter().zip(rhs.iter())
       .fold(0, |sum, (x,y)| sum + (x ^ y).count_ones())
}


/// Return the number of differing bits between two sequences of characters. 
/// If their lengths are different, the longer sequence is truncated.
pub fn edit_distance_str(lhs: &str, rhs: &str) -> u32
{
    edit_distance(&lhs.as_bytes(), &rhs.as_bytes())
}


/// Return the average Hamming distance between consecutive blocks
pub fn avg_hamming_distance(data: &[u8], block_size: usize, n_blocks: usize) -> f32
{
    let mut sum_distances = 0;
    for i in 0 .. n_blocks-1 {
        sum_distances += edit_distance(
            &data[i*block_size .. (i+1)*block_size],
            &data[(i+1)*block_size .. (i+2)*block_size]);
    }
    (sum_distances as f32) / ((block_size * (n_blocks-1)) as f32)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::hex::*;

    #[test]
    fn xor_1() {
        let x: Vec<u8> = vec![0, 1, 2, 3];
        let y: Vec<u8> = vec![1, 2, 3, 4];
        let z = xor(&x, &y);
        let exp_z: Vec<u8> = vec![1, 3, 1, 7];
        assert_eq!(exp_z, z);
    }

    #[test]
    fn xor_2() {
        let x: Vec<u8> = decode_hex(
            "1c0111001f010100061a024b53535009181c"
            ).expect("Input decoding failed (x)");
        let y: Vec<u8> = decode_hex(
            "686974207468652062756c6c277320657965"
            ).expect("Input decoding failed (y)");
        let z = xor(&x, &y);
        let z = encode_hex(&z);
        assert_eq!(z, "746865206b696420646f6e277420706c6179");
    }

    #[test]
    fn repeated_xor_1() {
        let message: String = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal".to_string();
        let key: String = "ICE".to_string();
        let ciphertext = encode_hex(&repeated_xor_str(&message, &key));
        let expected_ciphertext = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f".to_string();
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn edit_distance_1() {
        let lhs = "this is a test".to_string();
        let rhs = "wokka wokka!!!".to_string();
        let res = edit_distance_str(&lhs, &rhs);
        let expected = 37;
        assert_eq!(res, expected);
    }
}
