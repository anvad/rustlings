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

fn double_ref(a: &i32) -> i32 {
    a * 2
}

fn double(a: i32) -> i32 {
    a * 2
}

fn main() {
    // You can optionally experiment here.
    let a = vec![1, 2, 3];
    // i can pass an inline closure to map, or pass a function with the correct signature
    // here, i had to modify the signature of double_ref from i32 -> i32 to &i32 -> i32
    let b: Vec<i32> = a.iter().map(double_ref).collect();

    // if i don't want to modify the signature, then i have to have a closure that first de-references
    let c: Vec<i32> = a.iter().map(|elem| double(*elem)).collect();
    println!("a={:?}, b={:?}, c={:?}", a, b, c)
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
