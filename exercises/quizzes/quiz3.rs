// This quiz tests:
// - Generics
// - Traits
//
// An imaginary magical school has a new report card generation system written
// in Rust! Currently, the system only supports creating report cards where the
// student's grade is represented numerically (e.g. 1.0 -> 5.5). However, the
// school also issues alphabetical grades (A+ -> F-) and needs to be able to
// print both types of report card!
//
// Make the necessary code changes in the struct `ReportCard` and the impl
// block to support alphabetical report cards in addition to numerical ones.

use std::fmt::Display;

// TODO: Adjust the struct as described above.
// here, i've constrained the ReportCard struct to only accept grade type that implements Display
// in the quiz3.rs solution, they left this generic and only constrained the implementation
// so, their solution allows ReportCards that don't have a print function!
// struct ReportCard<T: Display> {
//     grade: T,
//     student_name: String,
//     student_age: u8,
// }

struct ReportCard<T> {
    grade: T,
    student_name: String,
    student_age: u8,
}

// TODO: Adjust the impl block as described above.
impl<T: Display> ReportCard<T> {
    fn print(&self) -> String {
        format!(
            "{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade,
        )
    }
}

struct MagicalGrade;

fn main() {
    // You can optionally experiment here.
    let report_card = ReportCard {
        grade: MagicalGrade,
        student_name: "Dumbledore".to_string(),
        student_age: 77,
    };
    println!(
        "{}'s grade is {}",
        report_card.student_name, /*report_card.grade*/ report_card.student_age,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1",
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+",
        );
    }
}
