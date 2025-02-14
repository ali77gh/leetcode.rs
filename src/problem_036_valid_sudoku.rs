use std::collections::HashSet;

const SIZE: usize = 9;
struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut dd = DuplicationDetector::new(&board);

        // -- x axis
        for x in 0..SIZE {
            dd.reset();
            for y in 0..SIZE {
                if dd.new_item(Pos::new(x, y)) {
                    return false;
                }
            }
        }

        // -- y axis
        for y in 0..SIZE {
            dd.reset();
            for x in 0..SIZE {
                if dd.new_item(Pos::new(x, y)) {
                    return false;
                }
            }
        }

        // sub boxes

        // row 1
        let mut subbox = Self::get_subbox();

        // column 1
        dd.reset();
        if dd.new_items(&subbox) {
            return false;
        }

        // column 2
        dd.reset();
        for p in &mut subbox {
            p.add_x(3);
        }
        if dd.new_items(&subbox) {
            return false;
        }

        // column 3
        dd.reset();
        for p in &mut subbox {
            p.add_x(3);
        }
        if dd.new_items(&subbox) {
            return false;
        }

        // row 2
        subbox = Self::get_subbox();
        for p in &mut subbox {
            p.add_y(3);
        }

        // column 1
        dd.reset();
        if dd.new_items(&subbox) {
            return false;
        }

        // column 2
        dd.reset();
        for p in &mut subbox {
            p.add_x(3);
        }
        if dd.new_items(&subbox) {
            return false;
        }

        // column 3
        dd.reset();
        for p in &mut subbox {
            p.add_x(3);
        }
        if dd.new_items(&subbox) {
            return false;
        }

        // row 3
        subbox = Self::get_subbox();
        for p in &mut subbox {
            p.add_y(6);
        }

        // column 1
        dd.reset();
        if dd.new_items(&subbox) {
            return false;
        }

        // column 2
        dd.reset();
        for p in &mut subbox {
            p.add_x(3);
        }
        if dd.new_items(&subbox) {
            return false;
        }

        // column 3
        dd.reset();
        for p in &mut subbox {
            p.add_x(3);
        }
        if dd.new_items(&subbox) {
            return false;
        }

        true
    }

    #[inline]
    const fn get_subbox() -> [Pos; 9] {
        [
            Pos::new(0, 0),
            Pos::new(0, 1),
            Pos::new(0, 2),
            Pos::new(1, 0),
            Pos::new(1, 1),
            Pos::new(1, 2),
            Pos::new(2, 0),
            Pos::new(2, 1),
            Pos::new(2, 2),
        ]
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    #[inline]
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    #[inline]
    pub fn add_x(&mut self, value: usize) {
        self.x += value;
    }
    #[inline]
    pub fn add_y(&mut self, value: usize) {
        self.y += value;
    }
}

struct DuplicationDetector<'a> {
    hash_set: HashSet<char>,
    board: &'a Vec<Vec<char>>,
}

impl<'a> DuplicationDetector<'a> {
    #[inline]
    pub fn new(board: &'a Vec<Vec<char>>) -> Self {
        Self {
            board,
            hash_set: HashSet::new(),
        }
    }

    #[inline]
    pub fn new_items(&mut self, poss: &[Pos]) -> bool {
        poss.iter().any(|pos| self.new_item(*pos))
    }

    /// returns true if is duplicate
    #[inline]
    pub fn new_item(&mut self, pos: Pos) -> bool {
        let element = self.board[pos.x][pos.y];
        if element == '.' {
            return false;
        }
        !self.hash_set.insert(element)
    }

    #[inline]
    pub fn reset(&mut self) {
        self.hash_set.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(board), true);
    }

    #[test]
    fn case_2() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn case_3() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['1', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(board), false);
    }

    #[test]
    fn case_4() {
        let board = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '.', '.', '.', '.', '.', '6', '.'],
            vec!['1', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '5', '7', '9'],
        ];

        assert_eq!(Solution::is_valid_sudoku(board), false);
    }
}
