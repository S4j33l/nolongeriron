mod department;
mod doctor;
mod patient;
use department::Department;
use doctor::Doctor;
use patient::Patient;
fn main() {
    let p: Patient;
    p = Patient::new("Sajeel".to_string(), 20, "Fever".to_string(), 83.7, 172.0);
    println!("{}", p.diagnosis);
    let mut d1: Doctor;
    d1 = Doctor::new(
        "Rabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let mut d2 = Doctor::new(
        "Rabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let mut d3 = Doctor::new(
        "Rabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let mut d4 = Doctor::new(
        "Rabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    d1.assign_doctor_to_patient(p);
    let mut dep = Department::new("Oncology".to_string(), 200);
    dep.assign_doctor_to_wing(d1);
    dep.assign_doctor_to_wing(d2);
    dep.assign_doctor_to_wing(d4);
    dep.show_doctors_in_department();
}
