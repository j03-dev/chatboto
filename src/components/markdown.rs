use iced::{
    widget::{column, text, Column},
    Element, Font,
};
use pulldown_cmark::{Event, Parser, TagEnd};

use crate::types::Message;

struct MarkdownRenderer<'a> {
    elements: Vec<Element<'a, Message>>,
    buffer: String,
}

impl<'a> MarkdownRenderer<'a> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            buffer: String::new(),
        }
    }

    fn flush_paragraph(&mut self) {
        if !self.buffer.trim().is_empty() {
            self.elements
                .push(text(self.buffer.clone()).size(14).into());
        }
        self.buffer.clear();
    }

    fn push_heading(&mut self, level: u32) {
        let size = match level {
            1 => 26,
            2 => 22,
            3 => 18,
            4 => 16,
            5 => 14,
            _ => 12,
        };

        self.elements.push(
            text(self.buffer.clone())
                .size(size)
                .font(Font::default())
                .into(),
        );
        self.buffer.clear();
    }

    fn push_list_item(&mut self) {
        let list_item = format!("• {}", self.buffer.trim());
        self.elements.push(text(list_item).size(14).into());
        self.buffer.clear();
    }

    fn finish(self) -> Column<'a, Message> {
        column(self.elements)
    }
}

pub fn markdown<'a>(content: &'a str) -> Element<'a, Message> {
    let parser = Parser::new(content);
    let mut renderer = MarkdownRenderer::new();

    for event in parser {
        match event {
            Event::End(tag) => match tag {
                TagEnd::Heading(level) => renderer.push_heading(level as u32),
                TagEnd::Paragraph => renderer.flush_paragraph(),
                TagEnd::Item => renderer.push_list_item(),
                _ => {}
            },
            Event::Text(txt) | Event::Code(txt) => renderer.buffer.push_str(&txt),
            Event::SoftBreak | Event::HardBreak => renderer.buffer.push('\n'),
            _ => {}
        }
    }

    renderer.flush_paragraph(); // Flush remaining if not ended correctly
    renderer.finish().into()
}

#[cfg(test)]
mod test {
    use super::markdown;

    #[test]
    fn test_parse() {
        let text = r#"
# Heading level 1

## Heading level 2

- item 1
- item 2
- item 3
- item 4
- item 5
- item 6

### Heading _italic_ level 3

#### Heading **Bold** level 4
"#;
        markdown(text);
    }
}
