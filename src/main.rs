// use std::io::{self, stdout};
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
#![warn(clippy::all, clippy::pedantic)]
mod document;
mod editor;
mod row;


// fn die(e: std::io::Error) {
    // panic!("{}",e);
// }
// 

mod terminal;
pub use document::Document;
use editor::Editor;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    // let _stdout = stdout().into_raw_mode().unwrap();
    Editor::default().run();    

    // for key in io::stdin().keys(){
    //     match key {
    //         Ok(key) => match key {
    //             Key::Char(c) => {
    //                 if c.is_control(){
    //                     println!("{:?} \r", c as u8);
    //                 } else {
    //                     println!("{:?} ({}) \r", c as u8, c);
    //                 }
                    

    //             }
    //             Key::Ctrl('q') => break,
    //             _=> println!("{:?}\r", key),

    //         },
    //         Err(err) => die(err),
        
    //     }

    // }
}
