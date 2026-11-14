// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

mod update;
mod view;

use std::collections::{HashSet, VecDeque};

use iced::keyboard;

use iced::{Subscription};


use iced::event::{self, Event};

use shawtype::data::words::Words;

use shawtype::jafl::Shaw;


//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
pub enum Message {
    KeyEvent(keyboard::Event),
}

//fn update(_state: &mut (), _message: Message) {}
/*
fn view(_state: &()) -> Element<'_, Message> {
    center(text("𐑣𐑧𐑤𐑴 𐑢𐑻𐑤𐑛!").size(32)).into()
}
*/

struct Pos {
    row: u16,
    col: u16,
}

pub struct App {
    msg: String,
    least_accurate: HashSet<char>,
    using_chars: HashSet<char>,
    unused_chars: VecDeque<char>,
    line: String,
    next_line: String,
    typed: Vec<char>,
    errors: Vec<char>,
    partial_char: Option<char>,
    line_pos: Pos,
    next_line_pos: Pos,
    typed_pos: Pos,
    partial_pos: Pos,
    message_pos: Pos,
    transliterator: Shaw,
}

/*
Possible key reponses, and example triggers
partial: 𐑩 or 𐑦𐑩
complete: 𐑑  or  𐑩𐑮
complete,complete: 𐑩𐑑
complete,partial:  𐑩𐑩

*/


impl App {

    const CHARS_PER_LINE: usize = 52;

    pub fn new() -> Self {
        let needs_improvement = vec![ '𐑪', '𐑨', '𐑱', '𐑲', '𐑬', '𐑶'];
                //'𐑧', '𐑴', '𐑱', '𐑲', '𐑒', '𐑜', '𐑢', '𐑘', '𐑷',];
        let mut letter_order: VecDeque<char> = vec![
            '𐑦', '𐑰', '𐑨', '𐑮', '𐑕', '𐑛', '𐑯', '𐑩', '𐑑', '𐑥', 
            '𐑤', '𐑼', '𐑳', '𐑻', '𐑧', '𐑪', '𐑢', '𐑵', '𐑸', '𐑭',
            '𐑿', '𐑷', '𐑘', '𐑖', '𐑡', '𐑙', '𐑗', '𐑫', '𐑓', '𐑹',
            '𐑚', '𐑟', '𐑝', '𐑠', '𐑞', '𐑔', '𐑐', '𐑒', '𐑜', '𐑣',
            '𐑴', '𐑱','𐑺', '𐑲', '𐑬', '𐑶', '𐑾', '𐑽'
        ].into();
        let mut using_chars: HashSet<char> = HashSet::new();
        for _ in 0..40 {
            if let Some(c) = letter_order.pop_front() {
                using_chars.insert(c);
            }
        }

        let mut words = Words::dictionary(using_chars.clone());
        let least_accurate: HashSet<char> = needs_improvement.iter().cloned().collect();

        Self {
            msg: "𐑣𐑧𐑤𐑴 𐑢𐑻𐑤𐑛!".to_string(),
            least_accurate: least_accurate.clone(),
            using_chars: using_chars,
            unused_chars: letter_order,
            line: words.line(Self::CHARS_PER_LINE, &least_accurate),
            next_line: words.line(Self::CHARS_PER_LINE, &least_accurate),
            typed: Vec::new(),
            errors: Vec::new(),
            partial_char: None,
            line_pos: Pos{row: 3, col: 10},
            next_line_pos: Pos{row: 6, col: 10},
            typed_pos: Pos{row: 3, col: 10},
            partial_pos: Pos{row: 4, col: 10},
            message_pos: Pos{row: 2, col: 30},
            transliterator: Shaw::new()
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard_event) => Some(Message::KeyEvent(keyboard_event)),
            _ => None,
        })
    }

}
