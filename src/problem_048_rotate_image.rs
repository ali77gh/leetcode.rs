struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let li = n - 1; // li means last index
        let mid = n / 2;
        let is_odd = n % 2 == 1;

        for i in 0..mid {
            for j in 0..mid {
                // top-left to top-right
                let temp1 = matrix[j][li - i];
                matrix[j][li - i] = matrix[i][j];

                // top-right to bottom-right
                let temp2 = matrix[li - i][li - j];
                matrix[li - i][li - j] = temp1;

                // bottom-right to bottom-left
                let temp3 = matrix[li - j][i];
                matrix[li - j][i] = temp2;

                // bottom-left to top-left
                matrix[i][j] = temp3;
            }
            if is_odd {
                // top to right
                let temp1 = matrix[mid][li - i];
                matrix[mid][li - i] = matrix[i][mid];

                // right to bottom
                let temp2 = matrix[li - i][mid];
                matrix[li - i][mid] = temp1;

                // bottom to left
                let temp3 = matrix[mid][i];
                matrix[mid][i] = temp2;

                // left to top
                matrix[i][mid] = temp3;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn case_2() {
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11]
            ]
        );
    }
}
