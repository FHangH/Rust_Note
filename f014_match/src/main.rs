#[derive(Debug)] #[allow(dead_code)]
enum Level
{
    S, A, B, C
}

#[allow(dead_code)]
enum Student
{
    NAME,
    LEVEL(Level)
}

fn student_info(stu: &Student) -> u8
{
    match stu
    {
        Student::NAME => return 0,
        Student::LEVEL(level) => 
        {
            println!("Level: {:?}", level);
            return 1;
        }
    }
}

fn main() 
{
    let student = Student::LEVEL(Level::S);
    println!("{}", student_info(&student));
}
