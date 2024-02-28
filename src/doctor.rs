use crate::patient::Patient;
#[derive(Clone)]
pub struct Doctor {
    pub name: String,
    pub age: i32,
    pub specialization: String,
    pub experience: String,
    pub patients_under_care: Vec<Patient>,
}
impl Doctor {
    pub fn new(name: String, age: i32, specialization: String, experience: String) -> Doctor {
        Doctor {
            name,
            age,
            specialization,
            experience,
            patients_under_care: Vec::new(),
        }
    }
    pub fn assign_doctor_to_patient(&mut self, patient: &Patient) {
        self.patients_under_care.push(patient.clone());
    }
    pub fn show_doctor_information(&self) {
        println!("The name of the doctor is: {}", self.name);
        println!("The age of the doctor is: {}", self.age);
        println!(
            "The specialization of the doctor is: {}",
            self.specialization
        );
        println!("The experience of the doctor is: {}", self.experience);
    }
    pub fn show_patients_under_care(&self) {
        for patient in self.patients_under_care.iter() {
            println!("{}", patient.name);
        }
    }
    pub fn search_for_patient_under_care(&self, name: String) -> Option<Patient> {
        for patient in self.patients_under_care.iter() {
            if patient.name.eq(&name) {
                println!("Patient found!");
                Some(patient.clone());
            }
        }
        println!("Patient not found!");
        None
    }
}
