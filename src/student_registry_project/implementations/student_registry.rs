use std::collections::HashMap;

use crate::student_registry_project::types::basic_types::{
    Course, CourseRegistry, Sex, Student, StudentRegistry,
};

impl StudentRegistry {
    // this initializes a new StudentRegistry
    // new array of students
    pub fn new_session() -> StudentRegistry {
        StudentRegistry {
            total_students: Vec::new(),
            course_registry: CourseRegistry::new(),
        }
    }

    // register student
    pub fn register(
        &mut self,
        f_name: String,
        l_name: String,
        _age: u8,
        _height: f32,
        _sex: Sex,
    ) -> Student {
        // generate a unique ID
        let student_id: u32 = self.total_students.len() as u32 + 1;
        let student = Student {
            first_name: f_name,
            last_name: l_name,
            id: student_id,
            age: _age,
            height: _height,
            sex: _sex,
            enrolled_courses: HashMap::new(),
        };
        self.total_students.push(student.clone());
        student
        // student_id
    }

    // util function to get student by id
    pub fn get_student_by_id(&self, id: u32) -> Option<&Student> {
        self.total_students.get(id as usize)
    }

    pub fn enroll_student_course(&mut self, course_id: u32, student_id: u32) -> Result<(), String> {
        // Validate course and student exists in the registry,
        // Return a mutable borrow if they do.
        let (course, student) = self.get_course_and_student(course_id, student_id)?;

        // Check if student is already enrolled for this course,
        // No double enrolls allowed.
        if course.is_student_enrolled(student.id) {
            return Err(format!(
                "Student with ID {} is already enrolled in course {:#?}",
                student_id, course.name
            ));
        }

        // Confirm students enrolled in the course are within the class size.
        // Prevent enrollment if otherwise.
        if course.is_at_capacity() {
            return Err(format!(
                "Class capacity exceeded: {}/{} students enrolled.",
                course.enrolled_students.len(),
                course.capacity
            ));
        }

        // Enroll student in course
        course.enroll_student(student.to_owned());
        student.enroll_in_course(course_id, course.name.to_owned());

        Ok(())
    }

    fn get_course_and_student(
        &mut self,
        course_id: u32,
        student_id: u32,
    ) -> Result<(&mut Course, &mut Student), String> {
        let course = self
            .course_registry
            .courses
            .get_mut(&course_id)
            .ok_or_else(|| {
                format!(
                    "Invalid Course ID. There are {} courses available.",
                    self.course_registry.total_courses
                )
            })?;
        let student = self
            .total_students
            .get_mut((student_id - 1) as usize)
            .ok_or_else(|| format!("Invalid Student ID: {}", student_id))?;

        Ok((course, student))
    }
}

impl Student {
    pub fn get_registered_courses(&self) -> HashMap<u32, String> {
        self.enrolled_courses.to_owned()
    }

    pub fn get_registered_course_by_id(&self, course_id: u32) -> String {
        self.enrolled_courses
            .get(&course_id)
            .expect("Must be a valid Course ID.")
            .to_owned()
    }

    fn enroll_in_course(&mut self, course_id: u32, course_name: String) {
        self.enrolled_courses.insert(course_id, course_name);
    }
}
