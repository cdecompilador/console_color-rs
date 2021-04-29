extern crate console_color;

use console_color::{
    Terminal,
    Color,
};

fn main() {
    let term = Terminal::new();
    term.write("Hello World\n", Color::Black, Color::Green);
    println!("Hello World");
    term.write("Hello World\n", Color::Red, Color::Green);
    println!("Hello\nWorld");
}
