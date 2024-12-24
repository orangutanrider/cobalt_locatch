use std::str::Chars;
use locatch_macro::impl_from_json;
use serde::Deserialize;

use super::ticket::Ticket;

#[derive(Deserialize)]
#[derive(Clone)]
pub struct FilenameMacro {
    file_extension: Option<String>,
    prefix: Option<String>,
    suffix: Option<String>,
}
impl_from_json!(FilenameMacro);

/// Escapes a character in the filename for a specific funtion in the filename macro
const ESCAPE_CHAR: char = '$';
/// When escaped it will replace itself with the index of the ticket in the vec of tickets
const INDEX_CHAR: char = 'i';

/// It is still possible for the filename to be a None value after this completes.
pub(crate) fn apply_filename_macro(filename_macro: &FilenameMacro, tickets: &mut Vec<Ticket>) { 
    apply_prefix_vec(&filename_macro.prefix, tickets);
    apply_suffix_vec(&filename_macro.suffix, tickets);
    apply_suffix_vec(&filename_macro.file_extension, tickets);
    apply_escaped_char_functions(tickets);
}

fn apply_escaped_char_functions(tickets: &mut Vec<Ticket>) {
    let mut index: usize = 0;
    for ticket in tickets.iter_mut() {
        match &ticket.filename {
            Some(filename) => {
                ticket.filename = Some(iter_escapes(filename.chars(), String::with_capacity(filename.len()), index))
            },
            None => {/* Do nothing */},
        }

        index = index + 1;
    }
}

fn iter_escapes<'a>(mut chars: Chars<'a>, mut output: String, list_index: usize) -> String {
    let Some(token) = chars.next() else {
        return output;
    };

    if token != ESCAPE_CHAR {
        output.push(token);
        return iter_escapes(chars, output, list_index);
    }

    let Some(token) = chars.next() else {
        return output;
    };

    match token {
        INDEX_CHAR => {
            return apply_index(chars, output, list_index);
        },
        _ => { // No valid escape
            return iter_escapes(chars, output, list_index);
        },
    }
}

fn apply_index<'a>(chars: Chars<'a>, mut output: String, list_index: usize) -> String {
    output.push_str(&list_index.to_string());
    return iter_escapes(chars, output, list_index);
}

fn apply_prefix_vec(prefix: &Option<String>, tickets: &mut Vec<Ticket>) {
    let Some(prefix) = prefix else {
        return;
    };

    for ticket in tickets.iter_mut() {
        apply_prefix(&prefix, ticket);
    }
}

fn apply_suffix_vec(suffix: &Option<String>, tickets: &mut Vec<Ticket>) {
    let Some(suffix) = suffix else {
        return;
    };

    for ticket in tickets.iter_mut() {
        apply_suffix(&suffix, ticket);
    }
}

fn apply_prefix(prefix: &str, ticket: &mut Ticket) {
    match &ticket.filename {
        Some(filename) => {
            ticket.filename = Some(prefix.to_owned() + filename);
        },
        None => {
            ticket.filename = Some(prefix.to_owned());
        },
    }
}

fn apply_suffix(suffix: &str, ticket: &mut Ticket) {
    match &mut ticket.filename {
        Some(filename) => {
            filename.push_str(suffix);
        },
        None => {
            ticket.filename = Some(suffix.to_owned());
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_escapes_index_test() {
        const TEST_STR: &str = "foo-$i-bar";
        let processed = iter_escapes(TEST_STR.chars(), String::with_capacity(TEST_STR.len()), 0);

        assert_eq!(processed, "foo-0-bar");
    }

    #[test]
    fn iter_escapes_nothing_test() {
        const TEST_STR: &str = "foobar";
        let processed = iter_escapes(TEST_STR.chars(), String::with_capacity(TEST_STR.len()), 0);

        assert_eq!(processed, "foobar");
    }

    #[test]
    fn iter_escapes_invalid_test() {
        const TEST_STR: &str = "foo$#bar";
        let processed = iter_escapes(TEST_STR.chars(), String::with_capacity(TEST_STR.len()), 0);

        assert_eq!(processed, "foobar");
    }

    #[test]
    fn suffix_test() {
        let mut ticket: Ticket = Ticket{
            filename: Some("foo".to_string()),
            ..Default::default()
        };
        apply_suffix("bar", &mut ticket);

        assert_eq!(ticket.filename, Some("foobar".to_string()));
    }

    #[test]
    fn prefix_test() {
        let mut ticket: Ticket = Ticket{
            filename: Some("bar".to_string()),
            ..Default::default()
        };
        apply_prefix("foo", &mut ticket);

        assert_eq!(ticket.filename, Some("foobar".to_string()));
    }
}