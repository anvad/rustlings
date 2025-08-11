fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let a = [0; 100];

    const SIZE: usize = 100;
    let mut iter = 1..=SIZE as i32;
    let arr: [i32; SIZE] = std::array::from_fn(|_| iter.next().unwrap());
    // let arr = get_numarr(10, None);
    println!("Array: {:?}", arr);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        let b = a.len();
        println!("Meh, I eat arrays like that for breakfast. {b}");
        panic!("Array not big enough, more elements needed");
    }
}

// fn get_numarr(size: usize, start: Option<i32>) -> [i32; 10] {
//     let start = start.unwrap_or(1);
//     let mut iter = start..=start + size as i32 - 1;
//     std::array::from_fn(|_| iter.next().unwrap())
// }
