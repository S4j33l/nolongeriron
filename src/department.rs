use crate::doctor::Doctor;
pub enum DepartmentName {
    Oncology,
    Opthalmology,
    Dermatology,
    Cardiology,
    ENT,
    Neurology,
    Rehabilitation,
    Gynaecology,
    Urology,
    ICU,
    BurnUnit,
    AccidentAndEmergency,
    Psychiatry,
}
pub struct Department {
    pub name: DepartmentName,
    pub capacity: u32,
    pub doctors_in_department: Vec<Doctor>,
}
impl Department {
    fn check_if_department_name_is_valid(name: &str) -> DepartmentName {
        match name {
            "Oncology" => DepartmentName::Oncology,
            "Opthalmology" => DepartmentName::Opthalmology,
            "Dermatology" => DepartmentName::Dermatology,
            "Cardiology" => DepartmentName::Cardiology,
            "ENT" => DepartmentName::ENT,
            "Neurology" => DepartmentName::Neurology,
            "Rehabilitation" => DepartmentName::Rehabilitation,
            "Gynaecology" => DepartmentName::Gynaecology,
            "Urology" => DepartmentName::Urology,
            "ICU" => DepartmentName::ICU,
            "Burn Unit" => DepartmentName::BurnUnit,
            "Accident And Emergency" => DepartmentName::AccidentAndEmergency,
            "Psychiatry" => DepartmentName::Psychiatry,
            _ => panic!("Invalid department name"),
        }
    }
    pub fn new(name: &str, capacity: u32) -> Department {
        Department {
            name: Department::check_if_department_name_is_valid(&name),
            capacity,
            doctors_in_department: Vec::new(),
        }
    }
    pub fn show_department_information(&self) {
        println!("The capacity of this department is {}", self.capacity);
    }
    pub fn assign_doctor_to_department(&mut self, doctor: &Doctor) {
        self.doctors_in_department.push(doctor.clone());
    }
    pub fn show_doctors_in_department(&self) {
        for doctor in self.doctors_in_department.iter() {
            println!("{}", doctor.name)
        }
    }
    pub fn search_for_doctor_in_department(&self, name: String) -> Option<Doctor> {
        for doctor in self.doctors_in_department.iter() {
            if doctor.name.eq(&name) {
                println!("Doctor found!");
                Some(doctor.clone());
            }
        }
        println!("Doctor not found!");
        None
    }
}
