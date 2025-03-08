use rand::Rng;
use std::collections::HashMap;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    #[inline]
    fn new() -> Self {
        RandomizedSet {
            map: HashMap::<i32, usize>::new(),
            vec: Vec::<i32>::new(),
        }
    }

    #[inline]
    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            // already exist
            false
        } else {
            let index = self.vec.len();
            self.map.insert(val, index);
            self.vec.push(val);
            true
        }
    }

    #[inline]
    fn remove(&mut self, val: i32) -> bool {
        unsafe {
            if let Some(index) = self.map.remove(&val) {
                let last = self.vec.pop().unwrap_unchecked();
                if last != val {
                    self.vec[index] = last;
                    *self.map.get_mut(&last).unwrap_unchecked() = index;
                }
                true
            } else {
                false
            }
        }
    }

    #[inline]
    fn get_random(&self) -> i32 {
        self.vec[rand::thread_rng().gen_range(0..self.vec.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.insert(1), true);
        assert_eq!(rs.remove(2), false);
        assert_eq!(rs.insert(2), true);
        {
            let r = rs.get_random();
            assert!(r == 1 || r == 2);
        }
        assert_eq!(rs.remove(1), true);
        assert_eq!(rs.insert(2), false);
        assert_eq!(rs.get_random(), 2);
    }

    #[test]
    fn case_2() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.remove(0), false);
        assert_eq!(rs.remove(0), false);
        assert_eq!(rs.insert(0), true);
        assert_eq!(rs.get_random(), 0);
        assert_eq!(rs.remove(0), true);
        assert_eq!(rs.insert(0), true);
    }

    #[test]
    fn case_3() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.insert(0), true);
        assert_eq!(rs.remove(0), true);
        assert_eq!(rs.insert(-1), true);
        assert_eq!(rs.remove(0), false);
        assert_eq!(rs.get_random(), -1);
        assert_eq!(rs.get_random(), -1);
        assert_eq!(rs.get_random(), -1);
        assert_eq!(rs.get_random(), -1);
    }

    #[test]
    fn case_4() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.insert(3), true);
        assert_eq!(rs.insert(3), false);
        assert_eq!(rs.get_random(), 3);
        assert_eq!(rs.get_random(), 3);
        assert_eq!(rs.insert(1), true);
        assert_eq!(rs.remove(3), true);
        assert_eq!(rs.get_random(), 1);
        assert_eq!(rs.get_random(), 1);
        assert_eq!(rs.insert(0), true);
        assert_eq!(rs.remove(0), true);
    }

    #[test]
    fn case_5() {
        let mut rs = RandomizedSet::new();
        assert_eq!(rs.insert(1), true);
        assert_eq!(rs.insert(10), true);
        assert_eq!(rs.insert(20), true);
        assert_eq!(rs.insert(30), true);
    }
}
