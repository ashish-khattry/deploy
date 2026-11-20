fn main(){
    let marks=83;
    match marks{
        0..=32=>{
            println!("Fail");
        },
        33..=80=>{
            println!("Average");
        },
        81..=99|100=>{
            println!("Excellent");
        },
        _=>{
            println!("ERROR!");
        },
    }
}
