#[derive(Clone)]
pub struct Patient {
    pub name: String,
    pub age: i32,
    pub diagnosis: String,
    pub weight: f32,
    pub height: f32,
}
impl Patient {
    pub fn new(name: String, age: i32, diagnosis: String, weight: f32, height: f32) -> Patient {
        Patient {
            name,
            age,
            diagnosis,
            weight,
            height,
        }
    }
    pub fn show_patient_information(&self) {
        println!("The name of the patient is: {}", self.name);
        println!("The age of the patient is: {}", self.age);
        println!("The diagnosis of the patient is: {}", self.diagnosis);
        println!("The weight of the patient is: {}", self.weight);
        println!("The height of the patient is: {}", self.height);
    }
}
