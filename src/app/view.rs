// SPDX-FileCopyrightText: 2026 Geoff Bull
// SPDX-License-Identifier: MIT

use iced::widget::{
    button, center, center_y, column, container, responsive, row, scrollable, text,
};
use iced::{Alignment, Center, Color, Element, Fill, Task, Theme};


use super::App;
use super::Message;

impl App {

    
    pub fn view(&self) -> Element<'_, Message> {

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

        let partial_line = row![
            text(typed_s.clone()).size(24)
                .color(Color::TRANSPARENT)
                .line_height(0.5),
            text(error_s.clone()).size(24)
                .color(Color::TRANSPARENT)
                .line_height(0.5),
            text(partial_s.clone()).size(24)
                .style(text::warning)
                .line_height(0.5),
        ];

        column![
            active_line,
            partial_line,
            /*text(partial_s.clone()).size(24)
                .style(text::warning),*/
            text(self.next_line.clone()).size(24)
                .style(text::secondary),
        ]
        .align_x(Alignment::Start)
        .spacing(0)
        .padding(40)
        .into()
    }

}