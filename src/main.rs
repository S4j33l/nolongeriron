mod department;
mod doctor;
mod patient;
use department::Department;
use doctor::Doctor;
use patient::Patient;
fn main() {
    let mut p1 = Patient::new(
        "Sajeel".to_string(),
        20,
        "Fever".to_string(),
        83.7,
        172.0,
        false,
    );
    let mut p2 = Patient::new(
        "Rajeel".to_string(),
        20,
        "Fever".to_string(),
        83.7,
        172.0,
        false,
    );
    let mut p3 = Patient::new(
        "Bajeel".to_string(),
        20,
        "Fever".to_string(),
        83.7,
        172.0,
        false,
    );
    let mut d1 = Doctor::new(
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
        "Sabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let mut dep = Department::new("Oncology".to_string(), 200);
    dep.assign_doctor_to_department(&d1);
    dep.assign_doctor_to_department(&d2);
    dep.assign_doctor_to_department(&d4);
    dep.show_doctors_in_department();
    d1.assign_doctor_to_patient(&mut p1);
    d1.assign_doctor_to_patient(&mut p2);
    d1.show_patients_under_care();
    d1.search_for_patient_under_care("Bajeel".to_string());
}
