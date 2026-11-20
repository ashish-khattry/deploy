struct Point{
    x:i32,
    y:i32,
}
fn print_coords(Point{x,y}:Point){
    println!("X={x} Y={y}");
}
fn main(){
    let p1=Point{x:10, y:20};
    print_coords(p1);
}
