use crate::department::Department;
use chrono::prelude::*;
pub struct hms {
    name: String,
    year_opened: DateTime<Local>,
    departments: Vec<Department>,
}
