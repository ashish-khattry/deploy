fn add(a:i32,b:i32)->Result<(),String>{
    let result=a+b;
    if result==a+b{
        Ok(())
    }
    else{
        Err(String::from("Math fail"))
    }
}
#[cfg(test)]
mod testing{
    use super::*;
    #[test]
    fn test()->Result<(),String>{
        add(4,4)?;
        Ok(())
    }
}
