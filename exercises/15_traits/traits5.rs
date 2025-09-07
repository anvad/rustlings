trait SomeTrait {
    fn some_function(&self) -> bool {
        true
    }
}

trait OtherTrait {
    fn other_function(&self) -> bool {
        true
    }
}

struct SomeStruct;
impl SomeTrait for SomeStruct {}
impl OtherTrait for SomeStruct {}

struct OtherStruct;
impl SomeTrait for OtherStruct {}
impl OtherTrait for OtherStruct {}

// TODO: Fix the compiler error by only changing the signature of this function.
// fn some_func<T: SomeTrait + OtherTrait>(item: T) -> bool {
//     item.some_function() && item.other_function()
// }

// fn some_func<T>(item: T) -> bool
// where
//     T: SomeTrait + OtherTrait,
// {
//     item.some_function() && item.other_function()
// }

fn some_func(item: impl SomeTrait + OtherTrait) -> bool {
    item.some_function() && item.other_function()
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // You can optionally experiment here.
    fn main() {
        // experimenting with reference lifetimes
        let a = 1;
        let mut r = &a;
        println!("r initial: {r}");
        let b = 2;

        {
            let x = 5;
            println!("x: {x}");
            // `x` does not live long enough
            // borrowed value does not live long enough
            // r = &x;
        }

        r = &b;

        println!("r: {r}");

        let x = String::from("abcd");
        let y = String::from("xyz");
        let r = longest(&x, &y);

        // cannot move out of `y` because it is borrowed
        // move out of `y` occurs on line 72 but earlier borrow occurs on line 68 and is later used in line 73
        // let z = y; // this will not work
        println!("The longest string is {}", r);
        let z = y; // this will be fine, because `r` is no longer used after line 73, so the borrow ends there.
        println!("z: {}", z);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_func() {
        assert!(some_func(SomeStruct));
        assert!(some_func(OtherStruct));
    }
}
