use std::collections::HashMap;

use crate::student_registry_project::{
    types::basic_types::{Course, CourseRegistry, Student},
    utils::convert_to_string,
};

impl CourseRegistry {
    pub fn new() -> CourseRegistry {
        CourseRegistry {
            courses: HashMap::new(),
            total_courses: 0,
        }
    }

    pub fn new_course(&mut self, name: &str, capacity: u32) -> u32 {
        self.total_courses += 1;
        let id = self.total_courses;
        let course = Course {
            id,
            name: convert_to_string(name),
            capacity,
            enrolled_students: Vec::with_capacity(capacity as usize),
        };
        self.courses.insert(id, course);
        id
    }

    pub fn get_course_by_id(&self, course_id: u32) -> Course {
        self.courses
            .get(&course_id)
            .expect("Must be a valid course ID.")
            .to_owned()
    }
}

impl Course {
    pub fn get_registered_students(&self) -> Vec<Student> {
        self.enrolled_students.to_owned()
    }

    pub fn get_registered_student_by_id(&self, student_id: usize) -> Student {
        self.enrolled_students
            .get(student_id)
            .expect("Must be a valid Student ID")
            .to_owned()
    }

    pub fn is_student_enrolled(&self, student_id: u32) -> bool {
        self.enrolled_students.iter().any(|s| s.id == student_id)
    }

    pub fn is_at_capacity(&self) -> bool {
        self.enrolled_students.len() >= self.capacity as usize
    }

    pub fn enroll_student(&mut self, student: Student) {
        self.enrolled_students.push(student);
    }
}
