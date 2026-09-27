struct Guess{
    no:i32
}
fn guess_number(g:Guess){
    if g.no>=1 && g.no<=100{
        panic!("Code is panic!");
    }
    else{
        println!("Code running successfully!");
    }
}
#[cfg(test)]
mod testing{
    use super::*;
    #[should_panic(expected="code is panic")]
    fn test(){
        let g1=Guess{ no:200 };
        guess_number(g1);
    }
}
