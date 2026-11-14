// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use iced::{Font, Size};


mod app;
use app::App;


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
