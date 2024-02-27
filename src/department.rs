use crate::doctor::Doctor;

pub struct Department {
    pub name: String,
    pub capacity: u32,
    pub doctors_in_department: Vec<Doctor>,
}
impl Department {
    pub fn new(name: String, capacity: u32) -> Department {
        Department {
            name,
            capacity,
            doctors_in_department: Vec::new(),
        }
    }
    pub fn show_department_capacity(&self) {
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
}
