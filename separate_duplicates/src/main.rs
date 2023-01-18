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
    fn test_remove_duplicates() {
        let input  = vec![2, 3, 3, 3, 6, 9, 9];
        assert_eq!(remove_duplicates(input), 5);
    }

}