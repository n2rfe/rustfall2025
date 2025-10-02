fn main() {
    let mut student1 = Student::new(String::from("Efren"), String::from("Psychology"));
    println!("Name: {}, Major: {}", student1.get_name(), student1.get_major());
    
    student1.set_major(String::from("Computer Science"));
    println!("Updated Major: {}", student1.get_major());
}

struct Student{
    name: String,
    major: String,
}


impl Student{
    fn new(n:String,m:String) -> Student {
        Self {
            name: n,
            major: m,
        }
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_major(&self) -> &String {
        &self.major
    }

    fn set_major(&mut self, new_major: String){
        self.major = new_major;
    }
}