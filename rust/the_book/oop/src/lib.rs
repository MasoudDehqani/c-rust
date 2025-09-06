pub trait Gui {
    fn draw(&self) {
        println!("DRAWING...")
    }
}

pub struct Image {
    pub width: u32,
    pub height: u32,
}

pub struct Button {
    pub text: String,
    pub width: u32,
    pub height: u32,
}

impl Gui for Image {
    fn draw(&self) {
        println!("width: {}, height: {}", self.width, self.height)
    }
}

impl Gui for Button {
    fn draw(&self) {
        println!(
            "text: {}, width: {}, height: {}",
            self.text, self.width, self.height
        )
    }
}
