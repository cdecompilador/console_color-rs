#[cfg(target_family = "windows")]
use crate::platform::windows::RawTerminal;

#[cfg(target_family = "unix")]
use crate::platform::unix::RawTerminal;

/// Enumarations of colors, maybe remove the `None` option and change the 
/// signature of some methods that use it to `Option<Color>`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
	Black, DarkBlue, DarkGreen, DarkCyan, DarkRed, DarkMagenta, 
	DarkYellow, DarkGray, Gray, Blue, Green, Cyan, Red, Magenta, 
	Yellow, White, None
}

/// Representation of a handle to the current terminal, allows writting to it
/// for the moment
pub struct Terminal {
    raw_terminal: RawTerminal,
}

impl Terminal {
    /// Instantiates a new `Terminal`
    pub fn new() -> Self {
        Terminal {
            raw_terminal: RawTerminal::new()
        }
    }

    /// Writes to the console specifing the foreground and
    /// background colors
    pub fn write(&self, msg: &str, fg: Color, bg: Color) {
        // Call to the platfrom dependent underlying `RawTerminal`
        self.raw_terminal.write(msg, fg, bg);
    }

    /* For linux specific should be better to use low level tty, maybe the ansi
     * option should be implemented as a tooglable feature
     *
    #[cfg(target_family = "unix")]
    pub fn write(&self, msg: &str, fg: Color, bg: Color)
    {
        let fg_enum = match fg
        {
            Color::None => 39,
            Color::Black => 30,
            Color::DarkRed => 31,
            Color::DarkGreen => 32,
            Color::DarkYellow => 33,
            Color::DarkBlue => 34,
            Color::DarkMagenta => 35,
            Color::DarkCyan => 36,
            Color::Gray=> 37,
            Color::DarkGray => 91,
            Color::Red => 91,
            Color::Green => 92,
            Color::Yellow => 93,
            Color::Blue => 94,
            Color::Magenta => 95,
            Color::Cyan => 96,
            Color::White => 97
        };

        let bg_enum = match bg
        {
            Color::None => 49,
            Color::Black => 40,
            Color::DarkRed => 41,
            Color::DarkGreen => 42,
            Color::DarkYellow => 43,
            Color::DarkBlue => 44,
            Color::DarkMagenta => 45,
            Color::DarkCyan => 46,
            Color::Gray=> 47,
            Color::DarkGray => 101,
            Color::Red => 101,
            Color::Green => 102,
            Color::Yellow => 103,
            Color::Blue => 104,
            Color::Magenta => 105,
            Color::Cyan => 106,
            Color::White => 107
        };

        let io = std::io::stdout();
        let mut handle = io.lock();

        handle.write(format!("\x1b[{};{}m{}\x1b[0m", fg_enum, bg_enum, msg).as_bytes()).unwrap();
        handle.flush().unwrap();
    }
    */
    
    /// Prints `msg` adding a line break at the end
    pub fn writeln(&self, msg: &str, fg: Color, bg: Color) {
        self.write(msg, fg, bg);

        // Faster than `println!`
        print!("\n");
    }
}
