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
    let var: i32 = 5;
    assert_eq!(&var * 8, 40);
    // This works, because &var refers to 5, not to the address of var. Note that in C, the & is an operator. In Rust, the & is acting as part of the type of the variable. Hence, the type is &i32.
    // Please see the [book](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html) and carefully follow the diagrams.

    // https://stackoverflow.com/questions/57739755/how-could-rust-multiply-i32-with-i32
    // Did the compiler implement the operation between &i32 and i32?
    // Yes. Well, not the compiler, but rather the standard library. You can see the impl in the documentation.
    a * 2
}

fn double_ref_deref(a: &i32) -> i32 {
    // here i am doing an explicit dereference
    // i don't need to since the multiplication is implemented for &i32 in the standard library
    *a * 2
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
    // another option would be to use traits (just like the stdlib does)
    let c: Vec<i32> = a.iter().map(|elem| double(*elem)).collect();

    let d: Vec<i32> = a.iter().map(double_ref_deref).collect();

    println!("a={:?}, b={:?}, c={:?}, d={:?}", a, b, c, d)
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
