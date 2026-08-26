//07_ecommerce_cart_mut
struct Cart {
    item_count: u32,
}
impl Cart {
    fn add_item(&mut self) {
        self.item_count += 1;
    }
}
fn main() {
    let mut c = Cart { item_count: 1 };
    for _ in 0..3 {
        c.add_item();
    }
    println!("Final item = {}", c.item_count);
}
