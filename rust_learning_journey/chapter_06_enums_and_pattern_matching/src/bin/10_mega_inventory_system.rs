//10_mega_inventory_system
#[derive(Debug)]
#[allow(dead_code)]
enum Item {
    Weapon { damage: i32 },
    Potion(i32),
    Empty,
}
fn main() {
    let my_inventory: Option<Item> = Some(Item::Weapon { damage: 50 });
    match my_inventory {
        Some(item) => {
            if let Item::Weapon { damage } = item {
                println!("Weapon equaipped! Deals {:?} damage", damage);
            } else {
                println!("Its not an weapon");
            }
        }
        None => {
            println!("Inventory slot is totally empty (Null-safe)");
        }
    }
}
