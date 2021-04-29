fn main() {
    windows::build!(
        Windows::Win32::SystemServices::{
            GetConsoleScreenBufferInfo,
            SetConsoleTextAttribute,
            WriteConsoleA,
            CONSOLE_SCREEN_BUFFER_INFO,
            HANDLE,
            FOREGROUND_INTENSITY,
            FOREGROUND_RED,
            FOREGROUND_GREEN,
            FOREGROUND_BLUE,
            BACKGROUND_INTENSITY,
            BACKGROUND_RED,
            BACKGROUND_GREEN,
            BACKGROUND_BLUE,
            NonClosableHandle
        },
        Windows::Win32::WindowsProgramming::{
            GetStdHandle,
            STD_HANDLE_TYPE
        }
    );
}
