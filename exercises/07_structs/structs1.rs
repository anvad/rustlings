struct ColorRegularStruct {
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    red: u8,
    green: u8,
    blue: u8,
}

// struct ColorTupleStruct(/* TODO: Add the fields that the test `tuple_structs` expects */);
struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: i32,
}

fn main() {
    // You can optionally experiment here.
    let user1 = User {
        name: String::from("a b"),
        email: String::from("a.b@mail.org"),
        age: 45,
    };
    println!("User1={:?}", user1);
    // Note: here, user1.email got moved but user1.name did not
    let user2 = User {
        name: String::from("a b2"),
        ..user1
    };
    // so here, i can still access user1.name, but can't access user1.email
    // so looks like instead of the whole struct getting moved, parts of it got moved.
    // works
    println!("User2={:?}, User1 name={}", user2, user1.name);
    // does not work
    // println!(
    //     "User2={:?}, User1 name={}, User1 email={}",
    //     user2, user1.name, user1.email
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
