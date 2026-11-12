use std::collections::{HashSet, VecDeque};

use iced::keyboard::{self, key::Named, Key};

use iced::widget::{
    button, center, center_y, column, container, responsive, row, scrollable, text,
};
use iced::{Alignment, Center, Color, Element, Fill, Font, Size, Subscription, Task,
        Theme};

use iced::event::{self, Event};



use shawtage::data::words::Words;

use shawtage::jafl::Shaw;
use shawtage::jafl::KeyEvent;
use shawtage::jafl;




pub fn main() -> iced::Result {
    //iced::run(update, view)
    iced::application(App::new, App::update, App::view)
        //.font(include_bytes!("../fonts/mikadoshavianregular.ttf").as_slice())
        .font(include_bytes!("../fonts/DoolittleGaramond-Regular.ttf").as_slice())
        // 2. Instruct Iced to use your custom font family as the default
        //.default_font(Font::with_name("Mikado Shavian"))
        .default_font(Font::with_name("Doolittle Garamond"))
        .window(iced::window::Settings {
            size: Size::new(800.0, 600.0), // (width, height)
            ..Default::default()
        })
        .subscription(App::subscription)
        .run()

}

//#[derive(Debug, Clone, Copy)]
#[derive(Debug, Clone)]
enum Message {
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

struct App {
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

    fn new() -> Self {
        let needs_improvement = vec![];
                //'𐑧', '𐑴', '𐑱', '𐑲', '𐑒', '𐑜', '𐑢', '𐑘', '𐑷',];
        let mut letter_order: VecDeque<char> = vec![
            '𐑦', '𐑩', '𐑰', '𐑨', '𐑮', '𐑕', '𐑛', '𐑯', '𐑑', '𐑥', 
            '𐑤', '𐑼', '𐑳', '𐑻', '𐑧', '𐑪', '𐑢', '𐑵', '𐑸', '𐑭',
            '𐑿', '𐑷', '𐑘', '𐑖', '𐑡', '𐑙', '𐑗', '𐑫', '𐑓', '𐑹',
            '𐑚', '𐑟', '𐑝', '𐑠', '𐑞', '𐑔', '𐑐', '𐑒', '𐑜', '𐑣',
            '𐑴', '𐑱','𐑺', '𐑲', '𐑬', '𐑶', '𐑾', '𐑽'
        ].into();
        let mut using_chars: HashSet<char> = HashSet::new();
        for _ in 0..6 {
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


    fn update(&mut self, message: Message) {

        match message {
            Message::KeyEvent(keyboard_event) => {
                /*
                if let iced::keyboard::Event(keyboard::Event::KeyPressed {
                    key: Key::Named(key::Named::Backspace),
                    ..
                }) = keyboard_event
                {
                    // Handle backspace logic here
                    println!("Backspace pressed!");
                    self.pop();
                    return;
                }
                */
                match keyboard_event {
                    keyboard::Event::KeyPressed { key, modifiers, text, .. } => {
                        // Handle key press here (e.g., check `key` or `modifiers`)

                        match key {
                            iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace) => {
                                // Handle Backspace key press
                                println!("Backspace pressed!");
                                self.pop();
                            }
                            _ => {}
                        }
                        let text = text.map(|smol| smol.to_string()).unwrap_or_default();
                        if let Some(latin) = text.chars().next() {
                            if latin.is_ascii_graphic() || latin == ' ' {
                                let unicode_val = latin as u32;
                                println!("Latin {} unicode {}", latin, unicode_val);
                                let shaw_vec = self.transliterator.process(latin);
                                for shaw in &shaw_vec {
                                    match shaw {
                                        KeyEvent::Complete(c) => {
                                            println!("Complete {}", c);
                                        },
                                        KeyEvent::Partial(c) => {
                                            println!("Partial {}", c);
                                        }
                                    }
                                    self.push(shaw.clone());
                                }
                            }
                            else {
                            }
                        }
                    }
                    keyboard::Event::KeyReleased { key, modifiers, .. } => {
                        // Handle key release
                    }
                    _ => {}
                }
                //Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {

        let typed_s: String = self.typed.clone().into_iter().collect();
        let error_s: String = self.errors.clone().into_iter().collect();
        let partial_s: String = self.partial_char.into_iter().collect();
        let line_s: String = self.line.chars().skip(self.typed.len() + self.errors.len())
                                .into_iter().collect();

        let active_line = row![
            text(typed_s.clone()).size(24)
                .style(text::success),
            text(error_s.clone()).size(24)
                .style(text::danger),
            text(line_s.clone()).size(24)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.palette().text),
                }),
        ];

        column![
            active_line,
            text(partial_s.clone()).size(24)
                .style(text::warning),
            text(self.next_line.clone()).size(24)
                .style(text::primary),
        ]
        .align_x(Alignment::Start)
        .spacing(2)
        .padding(40)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Keyboard(keyboard_event) => Some(Message::KeyEvent(keyboard_event)),
            _ => None,
        })
    }
/*
    fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed { key, modifiers, .. } = event
            else {
                return None;
            };
            if !modifiers.command() {
                return None;
            }
            handle_hotkey(key)
        })
    }

    */

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
                        self.errors.push(c);
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
