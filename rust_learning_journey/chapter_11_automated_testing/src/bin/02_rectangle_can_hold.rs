struct Rectangle{
    height:i32,
    weight:i32,
}
fn can_hold(r1:Rectangle,r2:Rectangle)->bool{
    r1.height>r2.height && r1.weight>r2.weight
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test(){
        let rr1=Rectangle{height:20, weight: 40 };
        let rr2=Rectangle{ height:10, weight:20 };
        assert!(can_hold(rr1,rr2));
    }
}