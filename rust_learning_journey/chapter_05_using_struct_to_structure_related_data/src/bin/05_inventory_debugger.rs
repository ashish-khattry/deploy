//05_inventory_debugger
#[derive(Debug)]
#[allow(dead_code)]
struct Product {
    name: String,
    price: u32,
    stock: u32,
}
fn main() {
    let product = Product {
        name: String::from("Nirma"),
        price: 22,
        stock: 255,
    };
    println!("\n\n");
    println!("Product detail:{:?}", product);
    println!("Product detail:{:#?}", product);
    dbg!(&product);
    println!("\n\n");
}
