use std::io::BufRead;

// the words that lines which start a block can start with
pub const BLOCK_STARTS: &[&str] = &["Background", "Scenario", "Scenario Outline"];

/// the words that lines which start a step can start with
pub const STEP_STARTS: &[&str] = &["Given", "When", "Then", "And"];

/// lexes the given file content
fn file(text: impl BufRead) -> Vec<Line> {
    text.lines()
        .into_iter()
        .enumerate()
        .map(|(number, line)| Line::new(line.unwrap(), number))
        .collect()
}

struct Line {
    /// the line number in the file
    number: usize,
    /// complete text of the line, as it is in the file
    full_text: String,
    /// line text without preceding whitespace
    trimmed_text: TrimmedLine,
    /// how much the line is indented
    indent: Indentation,
    /// whether this is a Given/When/Then line or not
    line_type: LineType,
}

impl Line {
    fn new(text: String, number: usize) -> Line {
        let (indent, trimmed) = clip_start(&text);
        Line {
            number,
            full_text: text,
            trimmed_text: trimmed,
            indent: indent,
            line_type: todo!(),
        }
    }
}

enum LineType {
    /// this line starts a block, i.e. "Background", "Scenario", etc
    BlockStart,
    /// this line starts a step, i.e. "Given", "When", "Then", etc
    StepStart,
    /// this line is neither a block or step start
    Other,
}

/// describes how much a line is indented
struct Indentation(usize);

/// a line without the initial whitespace
#[derive(Debug, Eq, PartialEq)]
struct TrimmedLine(String);

impl From<&str> for TrimmedLine {
    fn from(value: &str) -> Self {
        TrimmedLine(value.to_string())
    }
}

impl TrimmedLine {
    fn line_type(&self) -> LineType {
        if self.is_block_start() {
            LineType::BlockStart
        } else if self.is_step_start() {
            LineType::StepStart
        } else {
            LineType::Other
        }
    }

    fn is_block_start(&self) -> bool {
        BLOCK_STARTS.iter().any(|word| self.0.starts_with(word))
    }
    fn is_step_start(&self) -> bool {
        STEP_STARTS.iter().any(|word| self.0.starts_with(word))
    }
}

impl PartialEq<&str> for TrimmedLine {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

fn clip_start(line: &str) -> (Indentation, TrimmedLine) {
    for (i, c) in line.char_indices().into_iter() {
        if c == ' ' || c == '\t' {
            continue;
        }
        return (Indentation(i), TrimmedLine::from(&line[i..]));
    }
    (Indentation(0), TrimmedLine::from(line))
}

#[cfg(test)]
mod tests {

    mod clip_start {
        use crate::gherkin::lexer::clip_start;

        #[test]
        fn no_indent() {
            let (indent, clipped) = clip_start("text");
            assert_eq!(indent.0, 0);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn two() {
            let (indent, clipped) = clip_start("  text");
            assert_eq!(indent.0, 2);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn four() {
            let (indent, clipped) = clip_start("    text");
            assert_eq!(indent.0, 4);
            assert_eq!(clipped, "text");
        }

        #[test]
        fn only_spaces() {
            let (indent, clipped) = clip_start("    ");
            assert_eq!(indent.0, 4);
            assert_eq!(clipped, "");
        }
    }
}
