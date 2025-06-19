use iced::{
    widget::{column, row, text, Column},
    Element,
};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use crate::types::Message;

#[derive(Default, Clone, Copy)]
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
        txt.into()
    }
}

enum StyledSegment<'a> {
    Text(&'a str, InlineStyle),
    LineBreak,
}

enum StyledWord<'a> {
    Text(&'a str, InlineStyle),
    Break,
}

struct MarkdownRenderer<'a> {
    elements: Vec<Element<'a, Message>>,
    current_block: Vec<StyledSegment<'a>>,
    current_style: InlineStyle,
    current_block_size: u16,
}

impl<'a> MarkdownRenderer<'a> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            current_block: Vec::new(),
            current_style: InlineStyle::default(),
            current_block_size: 14,
        }
    }

    fn push_text(&mut self, content: &'a str) {
        let segments = content.split('\n');
        let mut iter = segments.peekable();
        while let Some(segment) = iter.next() {
            if !segment.is_empty() {
                self.current_block
                    .push(StyledSegment::Text(segment, self.current_style));
            }
            if iter.peek().is_some() {
                self.current_block.push(StyledSegment::LineBreak);
            }
        }
    }

    fn flush_block(&mut self) {
        if self.current_block.is_empty() {
            return;
        }

        // Split block into words and breaks
        let mut words = Vec::new();
        for segment in &self.current_block {
            match segment {
                StyledSegment::Text(content, style) => {
                    for word in content.split_whitespace() {
                        words.push(StyledWord::Text(word, *style));
                    }
                }
                StyledSegment::LineBreak => {
                    words.push(StyledWord::Break);
                }
            }
        }

        // Group words into lines (max 80 chars per line)
        let mut lines: Vec<Vec<StyledWord>> = Vec::new();
        let mut current_line: Vec<StyledWord> = Vec::new();
        let mut current_line_len = 0;
        const MAX_LINE_LENGTH: usize = 80;

        for word in words {
            match word {
                StyledWord::Text(content, style) => {
                    let word_len = content.len();
                    if current_line_len + word_len > MAX_LINE_LENGTH && !current_line.is_empty() {
                        lines.push(current_line);
                        current_line = Vec::new();
                        current_line_len = 0;
                    }
                    current_line_len += word_len + 1; // +1 for space
                    current_line.push(StyledWord::Text(content, style));
                }
                StyledWord::Break => {
                    lines.push(current_line);
                    current_line = Vec::new();
                    current_line_len = 0;
                }
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        // Create a row for each line
        for line in lines {
            let mut row_children = Vec::new();
            for (i, word) in line.iter().enumerate() {
                if let StyledWord::Text(content, style) = word {
                    row_children.push(style.apply(content, self.current_block_size));
                    if i < line.len() - 1 {
                        row_children.push(text(" ").size(self.current_block_size).into());
                    }
                }
            }
            self.elements.push(row(row_children).spacing(0).into());
        }

        self.current_block.clear();
    }

    fn push_list_item(&mut self) {
        self.current_block
            .insert(0, StyledSegment::Text("• ", self.current_style));
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
            Event::SoftBreak | Event::HardBreak => {
                renderer.current_block.push(StyledSegment::LineBreak);
            }
            _ => {}
        }
    }

    renderer.flush_block();
    renderer.finish().into()
}
