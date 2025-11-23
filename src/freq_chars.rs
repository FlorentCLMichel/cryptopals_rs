use std::collections::HashMap;
use std::hash::Hash;
use lazy_static::lazy_static;


const FRAC_SPACE: f64 = 0.2;


lazy_static! {
    pub static ref FREQ_ENGLISH: HashMap<u8,f64> = HashMap::<u8,f64>::from([
        (' ' as u8, FRAC_SPACE),
        ('a' as u8, (1. - FRAC_SPACE) * 0.082),
        ('b' as u8, (1. - FRAC_SPACE) * 0.015),
        ('c' as u8, (1. - FRAC_SPACE) * 0.028),
        ('d' as u8, (1. - FRAC_SPACE) * 0.043),
        ('e' as u8, (1. - FRAC_SPACE) * 0.127),
        ('f' as u8, (1. - FRAC_SPACE) * 0.022),
        ('g' as u8, (1. - FRAC_SPACE) * 0.020),
        ('h' as u8, (1. - FRAC_SPACE) * 0.061),
        ('i' as u8, (1. - FRAC_SPACE) * 0.070),
        ('j' as u8, (1. - FRAC_SPACE) * 0.0015),
        ('k' as u8, (1. - FRAC_SPACE) * 0.0077),
        ('l' as u8, (1. - FRAC_SPACE) * 0.04),
        ('m' as u8, (1. - FRAC_SPACE) * 0.024),
        ('n' as u8, (1. - FRAC_SPACE) * 0.067),
        ('o' as u8, (1. - FRAC_SPACE) * 0.075),
        ('p' as u8, (1. - FRAC_SPACE) * 0.019),
        ('q' as u8, (1. - FRAC_SPACE) * 0.00085),
        ('r' as u8, (1. - FRAC_SPACE) * 0.06),
        ('s' as u8, (1. - FRAC_SPACE) * 0.063),
        ('t' as u8, (1. - FRAC_SPACE) * 0.091),
        ('u' as u8, (1. - FRAC_SPACE) * 0.028),
        ('v' as u8, (1. - FRAC_SPACE) * 0.0098),
        ('w' as u8, (1. - FRAC_SPACE) * 0.024),
        ('x' as u8, (1. - FRAC_SPACE) * 0.0015),
        ('y' as u8, (1. - FRAC_SPACE) * 0.02),
        ('z' as u8, (1. - FRAC_SPACE) * 0.00074)
    ]);
}


/// Count the number of occurences of each element
pub(crate) fn get_counts<T: Eq + Hash + Copy>(x: &[T]) -> HashMap<T,usize>
{
    let mut result = HashMap::<T,usize>::new();
    for &e in x.iter() {
        let val = result.entry(e).or_insert(0);
        *val += 1;
    }
    result
}


/// Count the frequencies of each element
pub(crate) fn get_frequencies<T: Eq + Hash + Copy>(x: &[T]) -> HashMap<T,f64>
{
    let counts = get_counts(x);
    let mut result = HashMap::<T,f64>::new();
    for (&key, &val) in counts.iter() {
        result.insert(key, (val as f64) / (x.len() as f64));
    }
    result
}



/// Sum of squared differences between two frequency distributions
///
/// # Arguments
///
/// `freqs`: Hash map of frequencies to be tested
/// `reference`: Reference list of frequencies
///
/// The keys present in `freqs`  but not in `reference` are not taken into account. 
/// A missing key in `freqs` is interpreted as having a value 0.
pub(crate) fn squared_diff_freqs<T: Eq + Hash + Copy>(freqs: &HashMap<T,f64>, reference: &HashMap<T,f64>) -> f64 
{
    let mut res: f64 = 0.;
    for (key, &val_ref) in reference.iter() {
        match freqs.get(key) {
            Some(&val) => {
                let diff = val - val_ref;
                res += diff * diff;
            }
            None => res += val_ref * val_ref
        };
    }
    res
}


/// Compute the sum of squared differences between the frequencies of elements in a list and a
/// table of reference frequencies.
///
/// # Arguments
///
/// `x`: The list to be tested
/// `reference`: Reference list of frequencies
pub fn squared_diff_freqs_els<T: Eq + Hash + Copy>(x: &[T], reference: &HashMap<T,f64>) -> f64
{
    squared_diff_freqs(&get_frequencies(x), reference)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_counts_1() {
        let x: Vec<u8> = vec![3, 2, 1, 2, 3];
        let hm = get_counts(&x);

        assert_eq!(None, hm.get(&0));
        assert_eq!(Some(&1), hm.get(&1));
        assert_eq!(Some(&2), hm.get(&2));
        assert_eq!(Some(&2), hm.get(&3));
    }

    #[test]
    fn get_frequencies_1() {
        let x: Vec<u8> = vec![3, 2, 1, 2, 3];
        let hm = get_frequencies(&x);

        assert_eq!(None, hm.get(&0));
        assert_eq!(Some(&0.2), hm.get(&1));
        assert_eq!(Some(&0.4), hm.get(&2));
        assert_eq!(Some(&0.4), hm.get(&3));
    }
    
    #[test]
    fn squared_diff_freqs_1() {
        let uniform_freqs = HashMap::<u8,f64>::from([
            (0, 0.2),
            (1, 0.2),
            (2, 0.2),
            (3, 0.2),
            (4, 0.2),
        ]);
        
        let x: Vec<u8> = vec![3, 2, 1, 2, 3];
        let freqs = get_frequencies(&x);

        assert!((0.16 - squared_diff_freqs(&freqs, &uniform_freqs)).abs() < 1e-15);
    }
    
    #[test]
    fn squared_diff_freqs_els_1() {
        let uniform_freqs = HashMap::<u8,f64>::from([
            (0, 0.2),
            (1, 0.2),
            (2, 0.2),
            (3, 0.2),
            (4, 0.2),
        ]);
        
        let x: Vec<u8> = vec![3, 2, 1, 2, 3];

        assert!((0.16 - squared_diff_freqs_els(&x, &uniform_freqs)).abs() < 1e-15);
    }
}
