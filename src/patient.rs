use std::io;
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
    pub fn change_patient_information(&mut self) {
        println!("What do you want to change? (name, age, diagnosis, weight, height)");
        let mut change = String::new();
        io::stdin().read_line(&mut change).unwrap();
        change = change.trim().to_string();
        if change.eq("Name") || change.eq("name") {
            self.name = Self::change_name();
        } else if change.eq("Age") || change.eq("age") {
            self.age = Self::change_age();
        } else if change.eq("Diagnosis") || change.eq("diagnosis") {
            self.diagnosis = Self::change_diagnosis();
        } else if change.eq("Weight") || change.eq("weight") {
            self.weight = Self::change_weight();
        } else if change.eq("Height") || change.eq("height") {
            self.height = Self::change_height();
        }
    }
    fn change_name() -> String {
        let mut name = String::new();
        println!("Please enter the new name of the patient: ");
        io::stdin().read_line(&mut name).unwrap();
        name.trim().to_string()
    }

    fn change_age() -> i32 {
        let mut age = String::new();
        println!("Please enter the new age of the patient: ");
        io::stdin().read_line(&mut age).unwrap();
        age.trim().parse().unwrap()
    }

    fn change_diagnosis() -> String {
        let mut diagnosis = String::new();
        println!("Please enter the new diagnosis of the patient: ");
        io::stdin().read_line(&mut diagnosis).unwrap();
        diagnosis.trim().to_string()
    }

    fn change_weight() -> f32 {
        let mut weight = String::new();
        println!("Please enter the new weight of the patient: ");
        io::stdin().read_line(&mut weight).unwrap();
        weight.trim().parse().unwrap()
    }

    fn change_height() -> f32 {
        let mut height = String::new();
        println!("Please enter the new height of the patient: ");
        io::stdin().read_line(&mut height).unwrap();
        height.trim().parse().unwrap()
    }
}
