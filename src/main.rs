mod department;
mod doctor;
mod patient;
use department::Department;
use doctor::Doctor;
use patient::Patient;
fn main() {
    let p1 = Patient::new("Sajeel".to_string(), 20, "Fever".to_string(), 83.7, 172.0);
    let p2 = Patient::new("Rajeel".to_string(), 20, "Fever".to_string(), 83.7, 172.0);
    let p3 = Patient::new("Bajeel".to_string(), 20, "Fever".to_string(), 83.7, 172.0);
    let mut d1 = Doctor::new(
        "Rabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let d2 = Doctor::new(
        "Rabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let d3 = Doctor::new(
        "Rabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let d4 = Doctor::new(
        "Sabeel".to_string(),
        25,
        "Oncology".to_string(),
        "3 years".to_string(),
    );
    let mut dep = Department::new("Oncology", 200);
    dep.assign_doctor_to_department(&d1);
    dep.assign_doctor_to_department(&d2);
    dep.assign_doctor_to_department(&d4);
    dep.show_doctors_in_department();
    d1.assign_doctor_to_patient(p1);
    d1.assign_doctor_to_patient(p2);
    d1.show_patients_under_care();
    let pat = d1
        .search_for_patient_under_care("Sajeel".to_string())
        .unwrap_or(Patient::new(
            "Patient not found".to_string(),
            0,
            "".to_string(),
            0.0,
            0.0,
        ));
    println!("{}", pat.name);
}
