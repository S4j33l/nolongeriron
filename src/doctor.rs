use crate::patient::Patient;
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
    pub fn assign_doctor_to_patient(&mut self, patient: Patient) {
        self.patients_under_care.push(patient);
    }
    pub fn search_for_patient_under_care(&self) {}
}
