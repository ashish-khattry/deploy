fn internal_add(a:i32,b:i32)->i32{
    a+b
}
#[cfg(test)]
mod testing{
    use super::*;
    #[test]
    fn test(){
        assert_eq!(internal_add(10,10),20);
    }
}
