fn squaring_a_sorted_array(input: Vec<i32>) -> Vec<i32> {

    let mut output = vec![0; input.len()];

    let mut left_index = 0;
    let mut right_index =  input.len() - 1;
    let mut insertion_index = output.len() - 1;

    while left_index <= right_index {

        let left_square = input[left_index] * input[left_index];
        let right_square = input[right_index] * input[right_index];

        if left_square > right_square {

            output[insertion_index] = left_square;

            left_index += 1;

        } else {

            output[insertion_index] = right_square;

            right_index -= 1;

        }

        if insertion_index != 0 {
            insertion_index -= 1;
        }

    }

    output

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