trait Draw {
    fn draw(&self);
}
struct Button {
    width: u32,
    height: u32,
}
impl Draw for Button {
    fn draw(&self) {
        println!("\nButton has been drawed");
        println!("Height is :{}", self.height);
        println!("Width is :{}", self.width);
    }
}
struct TextField {
    text: String,
}
impl Draw for TextField {
    fn draw(&self) {
        println!(
            "Textfield has been drawed on screen\nWith text -\n{}",
            self.text
        );
    }
}
struct Screen {
    components: Vec<Box<dyn Draw>>,
}
impl Screen {
    fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
fn main() {
    let screen=Screen{
        components:vec![Box::new(Button{width:140, height:150,}),Box::new(TextField{text:String::from("This is the main screen of the Application\nYou will find all the neccesaary components here..")})],
    };
    screen.run();
}
