use core::panic;
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
        let change_for_match = &change[..];
        match change_for_match {
            "Name" | "name" => Self::change_name(change_for_match),
            "Age" | "age" => Self::change_age(change_for_match).to_string(),
            "Diagnosis" | "diagnosis" => Self::change_diagnosis(change_for_match),
            "Weight" | "weight" => Self::change_weight(change_for_match).to_string(),
            "Height" | "height" => Self::change_height(change_for_match).to_string(),
            _ => panic!(),
        };
    }
    fn change_name(mut name: &str) -> String {
        println!("Please enter the new name of the patient: ");
        let mut new_name = String::new();
        io::stdin().read_line(&mut new_name).unwrap();
        new_name = new_name.trim().to_string();
        name = &new_name;
        name.to_string()
    }
    fn change_age(age: &str) -> u32 {
        age.to_string
    }
    fn change_diagnosis(diagnosis: &str) -> String {
        diagnosis.to_string()
    }
    fn change_weight(weight: &str) -> f32 {
        weight.to_string()
    }
    fn change_height(height: &str) -> f32 {
        height.to_string()
    }
}
