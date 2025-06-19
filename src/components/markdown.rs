use pulldown_cmark::{Event, HeadingLevel, Parser, TagEnd};

use iced::{
    font,
    widget::{column, text},
    Element, Font,
};

use crate::types::Message;

pub fn markdown<'l>(content: &'static str) -> Element<'l, Message> {
    let mut elements: Vec<Element<Message>> = Vec::new();
    let parser = Parser::new(&content);
    let mut txt = String::new();
    for event in parser {
        match event {
            Event::Text(cow_str) => txt.push_str(&cow_str),
            Event::End(tag_end) => {
                let elt = match tag_end {
                    TagEnd::Heading(heading_level) => {
                        let mut font = Font::default();
                        font.weight = font::Weight::Semibold;
                        text(txt.clone()).font(font).size(match heading_level {
                            HeadingLevel::H1 => 18,
                            HeadingLevel::H2 => 16,
                            HeadingLevel::H3 => 14,
                            HeadingLevel::H4 => 12,
                            HeadingLevel::H5 => 10,
                            HeadingLevel::H6 => 8,
                        })
                    }
                    TagEnd::Item => text(format!(" - {}", txt.clone())).size(12),
                    TagEnd::Emphasis => {
                        let mut font = Font::default();
                        font.style = font::Style::Italic;
                        text(txt.clone()).font(Font::default()).size(12)
                    }
                    TagEnd::Strong => {
                        let mut font = Font::default();
                        font.weight = font::Weight::Bold;
                        text(txt.clone()).font(Font::default()).size(12)
                    }
                    _ => text(txt.clone()).size(12),
                };
                elements.push(elt.into());
                txt.clear();
            }
            _ => (),
        };
    }
    column(elements).into()
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
