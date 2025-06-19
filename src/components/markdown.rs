use iced::{
    widget::{column, row, text, Column, Row},
    Element,
};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::types::Message;

#[derive(Default)]
struct InlineStyle {
    bold: bool,
    italic: bool,
}

impl InlineStyle {
    fn apply<'a>(&self, content: &'a str, size: u16) -> Element<'a, Message> {
        let mut txt = text(content).size(size);
        let mut font = iced::Font::default();
        if self.bold {
            font.weight = iced::font::Weight::Bold;
            txt = txt.font(font);
        }
        if self.italic {
            font.style = iced::font::Style::Italic;
            txt = txt.font(font);
        }
        txt.font(font).into()
    }
}

struct MarkdownRenderer<'a> {
    elements: Vec<Element<'a, Message>>,
    inline_buffer: Vec<Element<'a, Message>>,
    current_style: InlineStyle,
    current_block_size: u16,
}

impl<'a> MarkdownRenderer<'a> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            inline_buffer: Vec::new(),
            current_style: InlineStyle::default(),
            current_block_size: 14,
        }
    }

    fn push_text(&mut self, content: &'a str) {
        let txt: Element<'a, Message> = self
            .current_style
            .apply(content, self.current_block_size)
            .into();
        self.inline_buffer.push(txt);
    }

    fn flush_block(&mut self) {
        if !self.inline_buffer.is_empty() {
            let line: Row<'a, Message> = row(self.inline_buffer.drain(..)).spacing(0);
            self.elements.push(line.into());
        }
    }

    fn push_list_item(&mut self) {
        self.inline_buffer
            .insert(0, text("• ").size(self.current_block_size).into());
        self.flush_block();
    }

    fn finish(self) -> Column<'a, Message> {
        column(self.elements).spacing(10)
    }
}

pub fn markdown<'a>(content: &'a str) -> Element<'a, Message> {
    let parser = Parser::new(content);
    let mut renderer = MarkdownRenderer::new();

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    renderer.current_block_size = match level as i32 {
                        1 => 26,
                        2 => 22,
                        3 => 18,
                        4 => 16,
                        5 => 14,
                        _ => 12,
                    };
                }
                Tag::Emphasis => renderer.current_style.italic = true,
                Tag::Strong => renderer.current_style.bold = true,
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(..) => {
                    renderer.flush_block();
                    renderer.current_block_size = 14;
                }
                TagEnd::Paragraph => renderer.flush_block(),
                TagEnd::Item => renderer.push_list_item(),
                TagEnd::Emphasis => renderer.current_style.italic = false,
                TagEnd::Strong => renderer.current_style.bold = false,
                _ => {}
            },
            Event::Text(txt) | Event::Code(txt) => {
                renderer.push_text(Box::leak(Box::new(txt)));
            }
            Event::SoftBreak | Event::HardBreak => renderer.push_text("\n"),
            _ => {}
        }
    }

    renderer.flush_block();
    renderer.finish().into()
}
