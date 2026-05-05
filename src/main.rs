// A struct that models a simple rectangle, which is merely width and height
struct Rectangle {
    width :i32, 
    height: i32,
}

impl Rectangle{
    pub fn new(width:i32, height:i32)-> Self{
        // Here, we provide a constructor-like new() method which returns a new Rectangle. 
        // Creating new() constructors is a common pattern in Rust. 
        Self{width, height}
    }
}

// A Rust Struct that models a simple square
struct Square{
    length: i32, 
}

impl Square {
    // Providing a constructor following the new pattern 
    pub fn new( length:i32) -> Self {
        Self{length}
    }
    // Adds an accessor to fetch the square's length if we know that we have a square
    pub fn get_length(&self)->i32{
        self.length 
    }
}


// we define the Rectangular trait, which provides accessors
// to properties common to rectangles and squares 
pub trait Rectangular {
    fn get_width(&self) ->i32; 
    fn get_height(&self) ->i32; 
    fn get_area(&self) ->i32; 
}

// Implementing the Rectangular trait for Rectangle struct 
impl Rectangular for Rectangle{ 
    fn get_width(&self)->i32 {
        self.width
    }
    fn get_height(&self)-> i32 {
        self.height
    }
    fn get_area(&self)->i32{
        self.width * self.height
    }
}

// Implementing the Rectangular trait for Square Struct
impl Rectangular for Square{
    fn get_width(&self)->i32 { 
        self.length
    }
    fn get_height(&self)->i32 {
        self.length
    }
    fn get_area(&self)->i32 {
        self.length * self.length
    }
}


// Testing our Rectangular trait
fn main(){
    let rect = Rectangle::new(2,3); 
    let square = Square::new(5); 
    // Testing the implementation related to the implementation of the Rectangle Struct
    println!(
        "rect has width {}, height{}, and area{}",
        rect.get_width(), 
        rect.get_height(),
        rect.get_area()
        ); 
    // Testing the implementation related to the implementation of the Square Struct
    println!(
        "square has length{} and area {}",
        square.get_length(), square.get_area()
    ); 
}
