fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: Multiply each element in the `input` slice by 2 and push it to
        // the `output` vector.
        output.push(element * 2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // An example of collecting a vector after mapping.
    // We map each element of the `input` slice to its value plus 1.
    // If the input is `[1, 2, 3]`, the output is `[2, 3, 4]`.
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: Here, we also want to multiply each element in the `input` slice
    // by 2, but with iterator mapping instead of manually pushing into an empty
    // vector.
    // See the example in the function `vec_map_example` above.
    input.iter().map(|element| element * 2).collect()
}

fn main() {
    // You can optionally experiment here.
    let input_loop = [2, 4, 6, 8, 10];
    let output_loop = vec_loop(&input_loop);

    let input_map_example = [1, 2, 3];
    let output_map_example = vec_map_example(&input_map_example);

    let input_map = [2, 4, 6, 8, 10];
    let output_map = vec_map(&input_map);

    println!("vec_loop(&{:?}) -> {:?}", input_loop, output_loop);
    println!(
        "vec_map_example(&{:?}) -> {:?}",
        input_map_example, output_map_example
    );
    println!("vec_map(&{:?}) -> {:?}", input_map, output_map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
