//! Keep Markdown code literal while preventing captured text from supplying tags.
use pulldown_cmark::{CodeBlockKind, Event, LinkType, Parser, Tag};

pub(super) fn body(source: &str) -> String {
    // Mask '<' without changing offsets so HTML blocks cannot hide code ranges.
    let masked = source.replace('<', "x");
    let mut code = Vec::new();
    let mut close = String::new();
    for (event, range) in Parser::new(&masked).into_offset_iter() {
        match event {
            Event::Code(_) => code.push(range),
            Event::Start(Tag::CodeBlock(kind)) => {
                if let CodeBlockKind::Fenced(_) = kind {
                    let block = &source[range.clone()];
                    let first = block.lines().next().unwrap_or("").trim_start();
                    if let Some(marker @ ('`' | '~')) = first.chars().next() {
                        let count = first.chars().take_while(|c| *c == marker).count();
                        let last = block.lines().last().unwrap_or("").trim();
                        let closed = block.lines().count() > 1
                            && last.chars().all(|c| c == marker)
                            && last.len() >= count;
                        if !closed {
                            close = format!("\n{}", marker.to_string().repeat(count));
                        }
                    }
                }
                code.push(range);
            }
            _ => {}
        }
    }
    for (event, range) in Parser::new(source).into_offset_iter() {
        if matches!(
            event,
            Event::Start(Tag::Link {
                link_type: LinkType::Autolink | LinkType::Email,
                ..
            })
        ) {
            code.push(range);
        }
    }
    code.sort_by_key(|r| r.start);
    let mut rendered = String::new();
    let mut ranges = code.iter().peekable();
    for (offset, c) in source.char_indices() {
        while ranges.peek().is_some_and(|r| r.end <= offset) {
            ranges.next();
        }
        if c == '<' && !ranges.peek().is_some_and(|r| r.contains(&offset)) {
            rendered.push_str("&lt;");
        } else {
            rendered.push(c);
        }
    }
    rendered.push_str(&close);
    rendered
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn markup_code_and_unclosed_fences() {
        assert_eq!(
            body("# Task\n> quote\n\n- **bold** `</x>`\n\n<x>"),
            "# Task\n> quote\n\n- **bold** `</x>`\n\n&lt;x>"
        );
        assert_eq!(
            body("~~~xml\n</agent_context>"),
            "~~~xml\n</agent_context>\n~~~"
        );
        assert_eq!(body("```xml\n<x>\n```"), "```xml\n<x>\n```");
    }
}
