fn main() {
    let my_string = vec![
        String::from("rust"),
        String::from("java"),
        String::from("c++"),
        String::from("python"),
    ];
    let my_clo = || println!("my string={:?}", my_string);
    my_clo();
}
