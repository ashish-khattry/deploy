fn main() {
    let my_string = String::from("Rust is best");
    let my_clo = move || {
        let consume_data = my_string;
        println!("String {consume_data}");
    };
    my_clo();
    //my_clo();// ownership error here 
}
