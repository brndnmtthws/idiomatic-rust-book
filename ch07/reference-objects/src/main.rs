#[derive(Debug)]
struct Student {
    name: String,
    id: u32,
}

impl Student {
    fn new(name: String, id: u32) -> Self {
        Self { name, id }
    }

    fn name(&self) -> &str {
        self.name.as_ref()
    }

    fn id(&self) -> u32 {
        self.id
    }
}

impl<'a> Student {
    fn to_ref(&'a self) -> StudentRef<'a> {
        StudentRef::new(self)
    }
}

#[derive(Debug)]
pub struct StudentList {
    students: Vec<Student>,
}

impl StudentList {
    pub fn new(students: &[(&str, u32)]) -> Self {
        Self {
            students: students
                .iter()
                .map(|(name, id)| Student::new((*name).into(), *id))
                .collect(),
        }
    }
}

impl<'a> StudentList {
    pub fn find_student_by_id(&'a self, id: u32) -> Option<StudentRef<'a>> {
        self.students
            .iter()
            .find(|s| s.id() == id)
            .map(Student::to_ref)
    }
    pub fn find_student_by_name(
        &'a self,
        name: &str,
    ) -> Option<StudentRef<'a>> {
        self.students
            .iter()
            .find(|s| s.name() == name)
            .map(Student::to_ref)
    }
}

#[derive(Debug)]
pub struct StudentRef<'a> {
    student: &'a Student,
}

impl<'a> StudentRef<'a> {
    fn new(student: &'a Student) -> Self {
        Self { student }
    }
}

impl<'a> PartialEq for StudentRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.student.id() == other.student.id()
    }
}

fn main() {
    let student = Student {
        name: "Walter".into(),
        id: 582,
    };
    let student_ref = StudentRef { student: &student };
    dbg!(&student);
    dbg!(student_ref);

    let student_list = StudentList::new(&[("Lyle", 621), ("Anna", 286)]);

    dbg!(&student_list);
    dbg!(student_list.find_student_by_id(621));
    dbg!(student_list.find_student_by_name("Anna"));

    let student_ref_621 = student_list.find_student_by_id(621).unwrap();
    let student_ref_286 = student_list.find_student_by_id(286).unwrap();
    dbg!(student_ref_286 == student_ref_621);
    dbg!(student_ref_286 != student_ref_621);
}
