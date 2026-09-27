fn heavy_database_test(){
    for i in 1..=100000{
        println!("{i}");
    }
}
#[cfg(test)]
mod testing{
    use super::*;
    #[ignore]
    fn test(){
        heavy_database_test();
    }
}
