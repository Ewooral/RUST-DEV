#[derive(Debug)]
pub struct Student {
    name: String,
    house: String,
    #[allow(dead_code)]
    patronus: String,
    grade: String,
}

impl Student {
    pub fn new(name: &str, house: &str, patronus: &str) -> Result<Student, &'static str> {
        if name.is_empty() {
            return Err("Missing name");
        }

        if !["Gryffindor", "Mission House", "Hufflepuff", "Ravenclaw", "Slytherin"].contains(&house) {
            return Err("House is not available");
        }

        Ok(Student {
            name: name.to_string(),
            house: house.to_string(),
            patronus: patronus.to_string(),
            grade: "89%".to_string(),
        })
    }

    pub fn charm(&self) -> Option<&str> {
        match self.name.as_str() {
            "Stag" => Some("Stag"),
            "Otter" => Some("Otter"),
            _ => None,
        }
    }

    pub fn set_grade(&mut self, grade: &str) {
        self.grade = grade.to_string();
    }

    pub fn get_grade(&self) -> &str {
        &self.grade
    }
}

fn main() {
    let mut student = Student::new("Elijah", "Gryffindor", "Gudah").unwrap();
    println!("{:?}", student);
    println!("{:?}", student.charm());
    println!("{}", student.house);
    student.set_grade("100%");
    println!("{}", student.get_grade());
}