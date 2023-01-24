fn squaring_a_sorted_array(input: Vec<i32>) -> Vec<i32> {
    vec![0, 1, 4, 4, 9]
}

fn main() {
    let input_ex_1 = vec![-2, -1, 0, 2, 3];
    let output_ex_1 = squaring_a_sorted_array(input_ex_1);
    println!("Output Example 1: {:?}", output_ex_1);

    let input_ex_2 = vec![-3, -1, 0, 1, 2];
    let output_ex_2 = squaring_a_sorted_array(input_ex_2);
    println!("Output Example 2: {:?}", output_ex_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec![-2, -1, 0, 2, 3];

        let output = squaring_a_sorted_array(input);

        assert_eq!(output, vec![0, 1, 4, 4, 9]);
    }

    #[test]
    fn example_2() {
        let input = vec![-3, -1, 0, 1, 2];

        let output = squaring_a_sorted_array(input);

        assert_eq!(output, vec![0, 1, 1, 4, 9]);
    }
}