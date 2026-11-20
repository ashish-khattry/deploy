fn main(){
    let mut items=vec![1,2,3,4,5,6,7,8,9];
    while let Some(element)=items.pop(){
        println!("{element}");
    }
}
