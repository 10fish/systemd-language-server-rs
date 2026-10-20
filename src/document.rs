use std::collections::HashMap;
use std::sync::RwLock;

use tower_lsp::lsp_types::{Position, Range, Url};

#[derive(Debug, Clone)]
pub struct DocumentState {
    pub version: i32,
    pub text: String,
    pub parsed: Document,
}

impl DocumentState {
    fn new(version: i32, text: String) -> Self {
        let parsed = Document::parse(&text);
        Self {
            version,
            text,
            parsed,
        }
    }
}

#[derive(Debug, Default)]
pub struct DocumentStore {
    documents: RwLock<HashMap<Url, DocumentState>>,
}

impl DocumentStore {
    pub fn open(&self, uri: Url, version: i32, text: String) {
        self.write().insert(uri, DocumentState::new(version, text));
    }

    pub fn change(&self, uri: Url, version: i32, text: String) -> bool {
        let mut documents = self.write();
        let Some(document) = documents.get(&uri) else {
            return false;
        };
        if version <= document.version {
            return false;
        }
        documents.insert(uri, DocumentState::new(version, text));
        true
    }

    pub fn close(&self, uri: &Url) -> Option<DocumentState> {
        self.write().remove(uri)
    }

    pub fn get(&self, uri: &Url) -> Option<DocumentState> {
        self.read().get(uri).cloned()
    }

    fn read(&self) -> std::sync::RwLockReadGuard<'_, HashMap<Url, DocumentState>> {
        self.documents
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, HashMap<Url, DocumentState>> {
        self.documents
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Document {
    pub sections: Vec<Section>,
    pub errors: Vec<ParseError>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub range: Range,
    pub name_range: Range,
    pub directives: Vec<Directive>,
}

#[derive(Debug, Clone)]
pub struct Directive {
    pub key: String,
    pub value: String,
    pub range: Range,
    pub key_range: Range,
    pub value_range: Range,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub range: Range,
    pub message: String,
}

impl Document {
    pub fn parse(text: &str) -> Self {
        let mut document = Self::default();
        let mut current_section = None;
        let mut continuation = false;

        for (line_index, raw_line) in text.split('\n').enumerate() {
            let line_index = line_index as u32;
            let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
            let trimmed = line.trim();

            if continuation {
                if trimmed.starts_with('#') || trimmed.starts_with(';') {
                    continue;
                }
                if let Some(section_index) = current_section {
                    let section: &mut Section = &mut document.sections[section_index];
                    if let Some(directive) = section.directives.last_mut() {
                        let continued = trimmed.strip_suffix('\\').unwrap_or(trimmed).trim_end();
                        if !continued.is_empty() {
                            directive.value.push(' ');
                            directive.value.push_str(continued);
                        }
                        directive.range.end = position(line_index, line);
                        directive.value_range.end = position(line_index, line);
                    }
                }
                continuation = trimmed.ends_with('\\');
                continue;
            }

            if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
                continue;
            }

            let leading_bytes = line.len() - line.trim_start().len();
            let leading = utf16_len(&line[..leading_bytes]);
            let line_range = Range::new(
                Position::new(line_index, leading),
                position(line_index, line),
            );

            if trimmed.starts_with('[') {
                let Some(close) = trimmed.find(']') else {
                    document.errors.push(ParseError {
                        range: line_range,
                        message: "section header is missing a closing ']'".into(),
                    });
                    current_section = None;
                    continue;
                };

                let name = &trimmed[1..close];
                let trailing = trimmed[close + 1..].trim();
                if name.is_empty() {
                    document.errors.push(ParseError {
                        range: line_range,
                        message: "section name cannot be empty".into(),
                    });
                    current_section = None;
                    continue;
                }
                if !trailing.is_empty() && !trailing.starts_with('#') && !trailing.starts_with(';')
                {
                    document.errors.push(ParseError {
                        range: line_range,
                        message: "unexpected text after section header".into(),
                    });
                }

                let name_start = leading + 1;
                let name_end = name_start + utf16_len(name);
                document.sections.push(Section {
                    name: name.into(),
                    range: line_range,
                    name_range: Range::new(
                        Position::new(line_index, name_start),
                        Position::new(line_index, name_end),
                    ),
                    directives: Vec::new(),
                });
                current_section = Some(document.sections.len() - 1);
                continue;
            }

            let Some(equals) = line.find('=') else {
                document.errors.push(ParseError {
                    range: line_range,
                    message: "directive is missing '='".into(),
                });
                continue;
            };

            let key_source = &line[..equals];
            let key = key_source.trim();
            if key.is_empty() {
                document.errors.push(ParseError {
                    range: line_range,
                    message: "directive name cannot be empty".into(),
                });
                continue;
            }

            let Some(section_index) = current_section else {
                document.errors.push(ParseError {
                    range: line_range,
                    message: "directive appears before any section".into(),
                });
                continue;
            };

            let key_leading_bytes = key_source.len() - key_source.trim_start().len();
            let key_start = utf16_len(&line[..key_leading_bytes]);
            let key_end = key_start + utf16_len(key);
            let value_source = &line[equals + 1..];
            let value_leading_bytes = value_source.len() - value_source.trim_start().len();
            let value_start_bytes = equals + 1 + value_leading_bytes;
            let mut value = value_source.trim().to_string();
            continuation = value.ends_with('\\');
            if continuation {
                value.pop();
                value = value.trim_end().to_string();
            }
            let value_start = utf16_len(&line[..value_start_bytes]);

            document.sections[section_index].directives.push(Directive {
                key: key.into(),
                value,
                range: line_range,
                key_range: Range::new(
                    Position::new(line_index, key_start),
                    Position::new(line_index, key_end),
                ),
                value_range: Range::new(
                    Position::new(line_index, value_start),
                    position(line_index, line),
                ),
            });
        }

        document
    }

    pub fn section_at(&self, line: u32) -> Option<&Section> {
        self.sections
            .iter()
            .rev()
            .find(|section| section.range.start.line <= line)
    }
}

pub fn contains(range: Range, position: Position) -> bool {
    (position.line > range.start.line
        || (position.line == range.start.line && position.character >= range.start.character))
        && (position.line < range.end.line
            || (position.line == range.end.line && position.character <= range.end.character))
}

fn position(line: u32, text: &str) -> Position {
    Position::new(line, utf16_len(text))
}

fn utf16_len(text: &str) -> u32 {
    text.encode_utf16().count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_duplicate_directives_and_ranges() {
        let document = Document::parse("[Unit]\nAfter=a.target\nAfter=b.target\n");
        let directives = &document.sections[0].directives;
        assert_eq!(directives.len(), 2);
        assert_eq!(directives[1].value, "b.target");
        assert_eq!(directives[1].key_range.start, Position::new(2, 0));
    }

    #[test]
    fn uses_utf16_columns() {
        let document = Document::parse("[Unit]\nDescription=🐟\n");
        let directive = &document.sections[0].directives[0];
        assert_eq!(directive.value_range.end.character, 14);
    }

    #[test]
    fn recovers_after_invalid_lines() {
        let document = Document::parse("broken\n[Unit]\nDescription=ok\n");
        assert_eq!(document.errors.len(), 1);
        assert_eq!(document.sections.len(), 1);
        assert_eq!(document.sections[0].directives.len(), 1);
    }

    #[test]
    fn joins_continuation_lines() {
        let document = Document::parse("[Unit]\nAfter=a.target \\\n+ b.target\n");
        let directive = &document.sections[0].directives[0];
        assert_eq!(directive.value, "a.target + b.target");
        assert_eq!(directive.range.end.line, 2);
    }

    #[test]
    fn ignores_stale_document_versions() {
        let store = DocumentStore::default();
        let uri = Url::parse("file:///tmp/example.service").unwrap();
        store.open(uri.clone(), 2, "new".into());
        assert!(!store.change(uri.clone(), 1, "old".into()));
        assert_eq!(store.get(&uri).unwrap().text, "new");
    }

    #[test]
    fn ignores_changes_for_documents_that_are_not_open() {
        let store = DocumentStore::default();
        let uri = Url::parse("file:///tmp/example.service").unwrap();
        assert!(!store.change(uri.clone(), 1, "text".into()));
        assert!(store.get(&uri).is_none());
    }
}
