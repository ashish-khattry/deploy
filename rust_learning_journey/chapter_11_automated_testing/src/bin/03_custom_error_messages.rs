fn greeting(name:&str)->bool{
    !name.is_empty()
}
fn main(){}
#[cfg(test)]
mod testing{
    use super::*;
    #[test]
    fn test(){
        assert!(greeting("devkinandan"),"Error chief: name is missing!");
    }
}