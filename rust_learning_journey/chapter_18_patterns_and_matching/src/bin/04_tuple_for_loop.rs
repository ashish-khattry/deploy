fn main(){
    let fruits=vec!["Apple","Mango","Banana"];
    for (index,element) in fruits.iter().enumerate(){
        println!("{index}:{element}");
    }
}
