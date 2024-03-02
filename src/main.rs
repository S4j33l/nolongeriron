mod department;
mod doctor;
mod hms;
mod patient;
use department::Department;
use doctor::Doctor;
use patient::Patient;
use std::io;
fn main() {
    println!("Welcome to HMS!");
    println!("You will be acting as a creator of a Hospital.");
    println!("You may create different departments and assign doctors to said departments");
    println!("Furthermore, you may also assign patients to the doctors in the departments.");
    println!("Press Y/y if you want to continue and N/n if you want to exit");
    let mut hms_starting_choice: String = String::new();
    io::stdin().read_line(&mut hms_starting_choice).unwrap();
    hms_starting_choice = hms_starting_choice.trim().to_string();
    if hms_starting_choice.eq("Y") || hms_starting_choice.eq("y") {
        println!("Vamos");
    } else if hms_starting_choice.eq("N") || hms_starting_choice.eq("n") {
        panic!();
    }
    let mut pat = Patient::new("Sajeel".to_string(), 20, "Nigga".to_string(), 83.0, 176.0);
    pat.show_patient_information();
    pat.change_patient_information();
    pat.show_patient_information();
}
