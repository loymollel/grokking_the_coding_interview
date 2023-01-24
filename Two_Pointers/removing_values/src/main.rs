fn removing_values(input: Vec<i32>, key: i32) -> i32 {

    let mut count = 0;

    for v in input.into_iter() {

        if v != key {
            count += 1;
        }
    }

    count
}


fn main() {

    let input_ex_1 = vec![3, 2, 3, 6, 3, 10, 9, 3];
    let key_ex_1 = 3;

    let output_ex_1 = removing_values(input_ex_1, key_ex_1);
    println!("Output Example 1 : {output_ex_1}");


    let input_ex_2 = vec![2, 11, 2, 2, 1];
    let key_ex_2 = 2;

    let output_ex_2 = removing_values(input_ex_2, key_ex_2);
    println!("Output Example 2: {output_ex_2}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {

        let input = vec![3, 2, 3, 6, 3, 10, 9, 3];
        let key = 3;

        assert_eq!(removing_values(input, key), 4);
    }

    #[test]
    fn test_example_2() {

        let input = vec![2, 11, 2, 2, 1];
        let key = 2;

        assert_eq!(removing_values(input, key), 2);
    }
}