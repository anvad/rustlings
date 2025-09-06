#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<Self, CreationError> {
        // TODO: This function shouldn't always return an `Ok`.

        // if value > 0 {
        //     Ok(Self(value as u64))
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Err(CreationError::Negative)
        // }

        // match value {
        //     1.. => Ok(Self(value as u64)),
        //     0 => Err(CreationError::Zero),
        //     ..0 => Err(CreationError::Negative),
        // }

        match value {
            v if value > 0 => Ok(Self(v as u64)),
            _ if value == 0 => Err(CreationError::Zero),
            _ => Err(CreationError::Negative),
        }
    }
}

fn main() {
    // You can optionally experiment here.
    let num: i64 = 2;
    let a = num.cmp(&0);
    println!("-5/(5-1)={a:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}
