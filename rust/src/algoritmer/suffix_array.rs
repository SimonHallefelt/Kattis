//suffix array
//lcp array = longest common prefix array

use std::cmp::Ordering;

fn main() {
    let word = "banana";
    let patarn = "nan";
    let suffix_array = suffix_array(word);
    let result = binary_search(word, patarn, suffix_array);
    println!("{}", if result.is_none() { "None".to_string() } else { result.unwrap().to_string() });
}

fn suffix_array(word: &str) -> Vec<usize> {
    let mut suffix_array: Vec<usize> = (0..word.len()).collect();
    suffix_array.sort_by(|a, b| word[*a..].cmp(&word[*b..]));
    suffix_array
}

fn binary_search(word: &str, patarn: &str, suffix_array: Vec<usize>) -> Option<usize> {
    let mut left = 0;
    let mut right = word.len();
    while left < right {
        let mid = (left + right) / 2;
        if word[suffix_array[mid]..].starts_with(patarn) {
            return Some(suffix_array[mid]);
        } else if word[suffix_array[mid]..].cmp(patarn) == Ordering::Less {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

//funkar inte, kolla testerna
fn _lcp_array(word: &str, suffix_array: Vec<usize>) -> Vec<usize> {
    let mut lcp_array = vec![0; word.len()];
    let mut h = 0;
    for i in 0..word.len() {
        if suffix_array[i] == word.len() - 1 {
            h = 0;
            continue;
        }
        let j = suffix_array[i] + 1;
        while i + h < word.len() && j + h < word.len() && word.chars().nth(i + h) == word.chars().nth(j + h) {
            h += 1;
        }
        lcp_array[i] = h;
        if h > 0 {
            h -= 1;
        }
    }
    lcp_array
}

//tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_array() {
        let word = "banana";
        let suffix_array = suffix_array(word);
        assert_eq!(suffix_array, vec![5, 3, 1, 0, 4, 2]);
    }

    #[test]
    fn test_binary_search() {
        let word = "banana";
        let patarn = "na";
        let suffix_array = suffix_array(word);
        let result = binary_search(word, patarn, suffix_array);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_binary_search2() {
        let word = "banana";
        let patarn = "nan";
        let suffix_array = suffix_array(word);
        let result = binary_search(word, patarn, suffix_array);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_binary_search3() {
        let word = "bananass";
        let patarn = "nas";
        let suffix_array = suffix_array(word);
        let result = binary_search(word, patarn, suffix_array);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_lcp_array() {
        let word = "banana";
        let suffix_array = suffix_array(word);
        let lcp_array = _lcp_array(word, suffix_array);
        assert_eq!(lcp_array, vec![1, 3, 0, 0, 2, 0]);
    }
}