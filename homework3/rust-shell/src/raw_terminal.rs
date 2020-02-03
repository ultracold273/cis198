
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::cursor::DetectCursorPos;
use std::io::{Write, Stdout, stdout, stdin};
use std::convert::TryInto;

use log::debug;

/// Object representing the cursor. Allows transparent creation of new lines.
pub struct RawTerminal {
    position: (u16, u16),
    stdout: termion::raw::RawTerminal<Stdout>,
}

impl RawTerminal {
    /// Creates a new RawTerminal and changes the terminal to raw mode.
    pub fn new() -> Self {
        let mut stdout = stdout().into_raw_mode().unwrap();
        let position = stdout.cursor_pos().unwrap();
        RawTerminal {
            position: position, 
            stdout: stdout,
        }
    }

    /// Write a string to the terminal.
    pub fn write(&mut self, msg: &str) { 
        // self.stdout.write_all(msg.as_bytes()).unwrap();
        write!(self.stdout, "{}", msg).unwrap();
        self.flush();
    }

    /// Write a string ending with a newline to the terminal.
    pub fn writeln(&mut self, msg: &str) {
        self.write(msg);
        self.newline();
    }

    /// Write a new line to the terminal.
    pub fn newline(&mut self) {
        write!(self.stdout, "\r\n").unwrap();
        self.flush();
    }

    /// Move the cursor to the beginning of line.
    pub fn move_to_beginning_of_line(&mut self) { 
        write!(self.stdout, "\r").unwrap();
        self.flush();
    }

    /// Write a message to our output log. (See next section).
    pub fn debug(&mut self, msg: &str) { 
        // write!(self.stdout, "Position: {}\r\n", self.y_position).unwrap()
        debug!("{}", msg);
    }

    /// Flush results. Results are not written to the terminal immediately, 
    /// so we flush after a command to write our output.
    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    fn move_cursor(&mut self, x: u16, y: u16) {
        write!(self.stdout, "{}", termion::cursor::Goto(x, y)).unwrap();
        self.flush();
    }

    /// read line
    pub fn read_line(&mut self) -> String {
        let (x, y) = self.stdout.cursor_pos().unwrap();
        let mut cur_x = x;
        let mut content = Vec::new();
        loop {
            for c in stdin().keys() {
                match c.unwrap() {
                    Key::Char('\n') => {
                        self.newline();
                        return content.iter().collect::<String>()
                    },
                    Key::Char(c) => {
                        let p: usize = (cur_x - x).try_into().unwrap();
                        content.insert(p, c);
                        self.write(& content.iter().skip(p).collect::<String>());
                        cur_x += 1;
                        self.move_cursor(cur_x, y);
                    },
                    Key::Backspace => {
                        if cur_x > x {
                            cur_x -= 1;
                            let p: usize = (cur_x - x).try_into().unwrap();
                            content.remove(p);
                            self.move_cursor(cur_x, y);
                            self.write(& content.iter().skip(p).collect::<String>());
                            self.write(" ");
                            self.move_cursor(cur_x, y);
                        }
                    },
                    Key::Delete => {
                        let size = content.len() as u16;
                        if cur_x < x + size {
                            let p: usize = (cur_x - x).try_into().unwrap();
                            content.remove(p);
                            self.write(& content.iter().skip(p).collect::<String>());
                            self.write(" ");
                            self.move_cursor(cur_x, y);
                        }
                    }
                    Key::Left => {
                        if cur_x > x {
                            cur_x -= 1;
                            self.move_cursor(cur_x, y);
                        }
                    },
                    Key::Right => {
                        let size = content.len() as u16;
                        if cur_x < x + size {
                            cur_x += 1;
                            self.move_cursor(cur_x, y);
                        }
                    },
                    _ => break,
                }
            }
        }
    }
}