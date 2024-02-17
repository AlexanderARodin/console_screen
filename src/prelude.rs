
pub type ResultOf <T> = Result< T, Box<dyn std::error::Error> >;

pub use crate::ConsoleWindow as ConsoleWindow;
pub use crate::console_draw::ConsoleDraw as ConsoleDraw;

pub use crossterm::event    as xEvent;
pub use crossterm::style::Colors as xColors;
pub use crossterm::style::Color as xColor;
