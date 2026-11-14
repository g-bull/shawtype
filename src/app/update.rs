// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use iced::keyboard;

use shawtype::jafl;
use shawtype::data::words::Words;

use super::App;
use super::Message;

impl App {

    
    pub fn update(&mut self, message: Message) {

        match message {
            Message::KeyEvent(keyboard_event) => {
                match keyboard_event {
                    keyboard::Event::KeyPressed { key, modifiers: _, text, .. } => {
                        // Handle key press here (e.g., check `key` or `modifiers`)
                        match key {
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                                // Handle Backspace key press
                                self.pop();
                                self.transliterator.clear();
                            }
                            _ => {}
                        }
                        let text = text.map(|smol| smol.to_string()).unwrap_or_default();
                        if let Some(latin) = text.chars().next() {
                            if latin.is_ascii_graphic() || latin == ' ' {

                                let shaw_vec = self.transliterator.process(latin);
                                for shaw in &shaw_vec {
                                    self.push(shaw.clone());
                                }
                            }
                        }
                    }
                    _ => {}
                }
                //Task::none()
            }
        }
    }


    fn push(&mut self, event : jafl::KeyEvent) {

        match event {
            jafl::KeyEvent::Partial(c) => {
                self.partial_char = Some(c);
            }
            jafl::KeyEvent::Complete(c) => {
                self.partial_char = None;

                // Only push if that won't overflow the line.
                if (self.errors.len() + self.typed.len()) < self.line.chars().count() {
                    // If c doesn't match the equivalent position in the target,
                    // or there are already errors, push c to errors, 
                    // other wise push it to typed.
                    let target = self.line.chars().nth(self.typed.len());
                    if self.errors.len() > 0
                        || target.is_none() /* typed past end of line? */
                        || target.unwrap() != c
                    {
                        if c == ' ' {
                            self.errors.push('\u{25a0}');
                        }
                        else {
                            self.errors.push(c);
                        }
                    }
                    else {
                        self.typed.push(c);
                    }
                }
                if self.typed.len() == self.line.chars().count() {
                    // A line has been completed
                    self.typed.clear();
                    if let Some(c) = self.unused_chars.pop_front() {
                        self.using_chars.insert(c);
                    }
                    let mut words = Words::dictionary(self.using_chars.clone());
                    self.line = self.next_line.clone();
                    self.next_line = words.line(Self::CHARS_PER_LINE, &self.least_accurate);


                }
            }
        }
    }

    fn pop(&mut self) -> Option<char> {
        if self.partial_char.is_none() {
            if self.errors.len() > 0 {
                return self.errors.pop();
            }
            else {
                return self.typed.pop();
            }
        }
        else {
            let result = self.partial_char;
            self.partial_char = None;
            return result;
        }
    }


}