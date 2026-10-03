fn main() {
    let my_string = vec![
        String::from("rust"),
        String::from("java"),
        String::from("c++"),
        String::from("python"),
    ];
    println!("my vector before closure call {:?}",my_string);
    let my_clo = || println!("my string={:?}", my_string);
    my_clo();
    println!("my vector after closure call {:?}",my_string);
}
