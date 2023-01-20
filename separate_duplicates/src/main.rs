fn remove_duplicates(input: Vec<i32>) -> i32 {
    4
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
        assert_eq!(remove_duplicates(input), 5);
    }

    #[test]
    fn test_example_2() {
        let input  = vec![2, 2, 2, 11];
        assert_eq!(remove_duplicates(input), 2);
    }

}