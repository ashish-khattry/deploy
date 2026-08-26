//05_multi_type_vector
enum Values {
    Text(String),
    Int(i32),
    Float(f32),
}
fn main() {
    let mut data: Vec<Values> = Vec::new();
    data.push(Values::Text(String::from("text")));
    data.push(Values::Int(255));
    data.push(Values::Float(26.355));
    for var in data {
        match var {
            Values::Text(text) => {
                println!("Text ={}", text);
            }
            Values::Int(num) => {
                println!("Integer ={}", num);
            }
            Values::Float(num) => {
                println!("Float ={}", num);
            }
        }
    }
}
