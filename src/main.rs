mod doctor;
mod patient;
use patient::Patient;
fn main() {
    let p: Patient;
    p = Patient {
        name: "John".to_string(),
        age: 32,
        weight: 80.0,
        height: 1.80,
    };
}
