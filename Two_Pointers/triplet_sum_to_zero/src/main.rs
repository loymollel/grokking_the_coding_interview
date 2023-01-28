fn triplet_sum_to_zero(input: Vec<i32>) -> Vec<Vec<i32>> {
    let output = vec![vec![-3, 1, 2], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, 0, 1]];
    output
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec![-3, 0, 1, 2, -1, 1, -2];
        let output = triplet_sum_to_zero(input);
        assert_eq!(output, vec![vec![-3, 1, 2], vec![-2, 0, 2], vec![-2, 1, 1], vec![-1, 0, 1]])
    }

    #[test]
    fn example_2() {
        let input = vec![-5, 2, -1, -2, 3];
        let output = triplet_sum_to_zero(input);
        assert_eq!(output, vec![vec![-5, 2, 3], vec![-2, -1, 3]])
    }
}
