trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

struct MITSoftware;
impl Licensed for MITSoftware {
    fn licensing_info(&self) -> String {
        "MIT License".to_string()
    }
}

// TODO: Fix the compiler error by only changing the signature of this function.
fn compare_license_types(software1: impl Licensed, software2: impl Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
    let cmp = match compare_license_types(SomeSoftware, OtherSoftware) {
        true => "yes",
        false => "no",
    };
    println!("Do both softwares have the same license? {cmp}");

    let cmp = match compare_license_types(SomeSoftware, MITSoftware) {
        true => "yes",
        false => "no",
    };
    println!("Do both softwares have the same license? {cmp}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(SomeSoftware, OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(OtherSoftware, SomeSoftware));
    }
}
