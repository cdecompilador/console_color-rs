#[cfg(target_family = "windows")]
extern crate winapi;

#[cfg(target_family = "windows")]
use winapi::um::{
	wincon::{ GetConsoleScreenBufferInfo, SetConsoleTextAttribute, CONSOLE_SCREEN_BUFFER_INFO, FOREGROUND_RED, FOREGROUND_BLUE, FOREGROUND_GREEN, FOREGROUND_INTENSITY, BACKGROUND_RED, BACKGROUND_GREEN, BACKGROUND_BLUE, BACKGROUND_INTENSITY },
	consoleapi::WriteConsoleA,
	processenv::GetStdHandle,
	winnt::HANDLE,
	winbase::STD_OUTPUT_HANDLE
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color
{
	Black, DarkBlue, DarkGreen, DarkCyan, DarkRed, DarkMagenta, DarkYellow, DarkGray, Gray, Blue, Green, Cyan, Red, Magenta, Yellow, White, None
}

pub struct AnsiTerminal
{
	#[cfg(target_os = "windows")]
	console_handle: HANDLE
}

impl AnsiTerminal
{
	pub fn new() -> AnsiTerminal
	{
		return AnsiTerminal {
			#[cfg(target_os = "windows")]
			console_handle: unsafe { GetStdHandle(STD_OUTPUT_HANDLE) }
		};
	}

	#[cfg(target_family = "windows")]
	pub fn write(&self, msg: &str, fg: Color, bg: Color)
	{
		let mut console_info: CONSOLE_SCREEN_BUFFER_INFO = Default::default();

		unsafe
		{
			// Store the old console attributes
			GetConsoleScreenBufferInfo(self.console_handle, &mut console_info);

			let fg_color_enum = match fg
			{
				Color::None => console_info.wAttributes & 0x000f,
				Color::Black => 0,
				Color::DarkBlue => FOREGROUND_BLUE,
				Color::DarkGreen => FOREGROUND_GREEN,
				Color::DarkCyan => FOREGROUND_GREEN | FOREGROUND_BLUE,
				Color::DarkRed => FOREGROUND_RED,
				Color::DarkMagenta => FOREGROUND_RED | FOREGROUND_BLUE,
				Color::DarkYellow => FOREGROUND_RED | FOREGROUND_GREEN,
				Color::DarkGray => FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE,
				Color::Gray => FOREGROUND_INTENSITY,
				Color::Blue => FOREGROUND_INTENSITY | FOREGROUND_BLUE,
				Color::Green => FOREGROUND_INTENSITY | FOREGROUND_GREEN,
				Color::Cyan => FOREGROUND_INTENSITY | FOREGROUND_GREEN | FOREGROUND_BLUE,
				Color::Red => FOREGROUND_INTENSITY | FOREGROUND_RED,
				Color::Magenta => FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_BLUE,
				Color::Yellow => FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN,
				Color::White => FOREGROUND_INTENSITY | FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE
			};

			let bg_color_enum = match bg
			{
				Color::None => console_info.wAttributes & 0x00f0,
				Color::Black => 0,
				Color::DarkBlue => BACKGROUND_BLUE,
				Color::DarkGreen => BACKGROUND_GREEN,
				Color::DarkCyan => BACKGROUND_GREEN | BACKGROUND_BLUE,
				Color::DarkRed => BACKGROUND_RED,
				Color::DarkMagenta => BACKGROUND_RED | BACKGROUND_BLUE,
				Color::DarkYellow => BACKGROUND_RED | BACKGROUND_GREEN,
				Color::DarkGray => BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE,
				Color::Gray => BACKGROUND_INTENSITY,
				Color::Blue => BACKGROUND_INTENSITY | BACKGROUND_BLUE,
				Color::Green => BACKGROUND_INTENSITY | BACKGROUND_GREEN,
				Color::Cyan => BACKGROUND_INTENSITY | BACKGROUND_GREEN | BACKGROUND_BLUE,
				Color::Red => BACKGROUND_INTENSITY | BACKGROUND_RED,
				Color::Magenta => BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_BLUE,
				Color::Yellow => BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN,
				Color::White => BACKGROUND_INTENSITY | BACKGROUND_RED | BACKGROUND_GREEN | BACKGROUND_BLUE
			};

			SetConsoleTextAttribute(self.console_handle, fg_color_enum | bg_color_enum);
			WriteConsoleA(self.console_handle, msg.as_ptr() as *const winapi::ctypes::c_void, msg.len() as u32, std::ptr::null_mut(), std::ptr::null_mut());

			// Restore the old console attributes
			SetConsoleTextAttribute(self.console_handle, console_info.wAttributes);
		}
	}

	#[cfg(target_family = "unix")]
	pub fn write(&self, msg: &str, fg: Color, bg: Color)
	{
		let fg_enum = match fg
		{
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

	pub fn writeln(&self, msg: &str, fg: Color, bg: Color)
	{
		self.write(msg, fg, bg);
		println!();
	}
}

/*
impl std::io::Write for AnsiTerminal
{
	fn write(&mut self, buf: &[u8])
	{
		
	}
}
*/
