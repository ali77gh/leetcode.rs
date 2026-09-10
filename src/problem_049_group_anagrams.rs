use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

pub struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<u64, Vec<String>> = HashMap::new();

        for item in strs {
            let hash = Self::hash(&item);
            if let Some(x) = map.get_mut(&hash) {
                x.push(item);
            } else {
                map.insert(hash, vec![item]);
            }
        }

        let mut ans = Vec::new();
        for inner in map.into_values() {
            ans.push(inner);
        }
        ans
    }

    // hash function of string that don't care about ordering and only care about inclusion
    #[inline]
    fn hash(data: &str) -> u64 {
        let mut inclusion: [u8; 26] = [0; 26];

        data.chars()
            .map(|x| (x as usize) - ('a' as usize)) // move range to 0-26
            .for_each(|char_index| inclusion[char_index] += 1);

        let mut s = DefaultHasher::new();
        inclusion.hash(&mut s);
        s.finish()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn case1() {
        let expected: Vec<Vec<String>> = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];

        let calculated = Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ]);

        assert_eq!(calculated, expected);
    }
}
