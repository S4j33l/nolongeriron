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
}
