fn removing_values(input: Vec<i32>, key: i32) -> i32 {

    for (k,v) in input.iter().enumerate() {
        // println!("k: {k}, v: {v}");
    }
}


fn main() {

    let input = vec![3, 2, 3, 6, 3, 10, 9, 3];
    // let key = 3;
    //
    // let output = removing_values(input, key);
    //
    // println!("Output: {output}");

    // for (k,v) in input.iter().enumerate() {
    //     println!("k: {k}, v: {v}");
    // }
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