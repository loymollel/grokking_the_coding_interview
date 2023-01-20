fn remove_duplicates(input: Vec<i32>) -> i32 {

    let mut output = 1;

    for (i, x) in input.iter().enumerate() {

        if i < input.len() - 1 {
            if *x != input[i + 1] {
                output += 1;
            }
        }

    }

    output

}

fn main() {

    let input  = vec![2, 3, 3, 3, 6, 9, 9];

    let output = remove_duplicates(input);

    println!("Output: {output}");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input  = vec![2, 3, 3, 3, 6, 9, 9];
        assert_eq!(remove_duplicates(input), 4);
    }

    #[test]
    fn test_example_2() {
        let input  = vec![2, 2, 2, 11];
        assert_eq!(remove_duplicates(input), 2);
    }

}