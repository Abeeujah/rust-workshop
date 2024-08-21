use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub id: u32,
    pub age: u8,
    pub height: f32,
    pub sex: Sex,
    pub enrolled_courses: HashMap<u32, String>,
}

#[derive(Debug, Clone)]
pub enum Sex {
    Male,
    Female,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub id: u32,
    pub name: String,
    pub capacity: u32,
    pub enrolled_students: Vec<Student>,
}

#[derive(Debug, Clone)]
pub struct CourseRegistry {
    pub courses: HashMap<u32, Course>,
    pub total_courses: u32,
}

#[derive(Debug, Clone)]
pub struct StudentRegistry {
    pub total_students: Vec<Student>,
    pub course_registry: CourseRegistry,
}

/***
 *
 * [
 *  {1},
 *  {2},
 *  {3},
 * ]
 */
