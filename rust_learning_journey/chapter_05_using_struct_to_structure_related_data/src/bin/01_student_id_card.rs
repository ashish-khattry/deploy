//01_student_id_card
struct Student {
    name: String,
    roll_number: u32,
    is_active: bool,
    branch: String,
}
fn main() {
    let s1 = Student {
        name: String::from("ashish khattry"),
        roll_number: 0001,
        is_active: false,
        branch: String::from("information technology"),
    };
    println!("\nStudent detail is as below ");
    println!("Name:{}", s1.name);
    println!("Roll number:{}", s1.roll_number);
    println!("Is active:{}", s1.is_active);
    println!("Branch:{}", s1.branch);
}
