use url::Url;
use std::{
    iter::Peekable,
    str::Chars,
};

pub fn format_to_string(url: &Url, template: &str) -> String {
    format_command(url, template).join("")
}


pub fn format_command(url: &Url, command: &str) -> Vec<String> {
    let lexed_command = lexer(command);
    let contains_variable = lexed_command.contains(&LexItem::StartVariable);
    // Create command from tokens
    let mut result: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut it = lexed_command.iter();
    while let Some(lex_item) = it.next() {
        match lex_item {
            LexItem::Word(word) => current.push_str(&word),
            LexItem::Space => {
                result.push(current);
                current = String::new();
            },
            LexItem::StartVariable => {
                if let Some(LexItem::Word(word)) = it.next() {
                    let value = get_variable(url, &word)
                        .expect("Variable not valid");
                    current.push_str(&value);
                } else {
                    panic!("Next token should be word");
                }
            },
            _ => (),
        }
    }
    result.push(current);
    // Add url to the end if no variable is present in the command
    if !contains_variable {
        result.push(url.as_str().to_string());
    }
    return result;
}

fn get_variable<'a>(url: &'a Url, key: &str) -> Option<&'a str> {
    match key {
        "url" => Some(url.as_str()),
        "scheme" => Some(url.scheme()),
        "host" => url.host_str(),
        "path" => Some(url.path()),
        "query" => url.query(),
        "fragment" => url.fragment(),
        _ => None,
    }
}

#[derive(PartialEq)]
pub enum LexItem {
    Word(String),
    Space,
    StartVariable,
    EndVariable
}

fn lexer(command: &str) -> Vec<LexItem> {
    let mut result = Vec::new();
    let mut it = command.chars().peekable();
    let mut in_variable = false;
    while let Some(c) = it.next() {
        let item = match c {
            // Start of variable if the next char is '{'
            '$' if !in_variable => {
                if Some(&'{') == it.peek() {
                    it.next();
                    in_variable = true;
                    LexItem::StartVariable
                } else {
                    LexItem::Word(get_word(c, &mut it, in_variable))
                }
            },
            // New word
            ' ' => LexItem::Space,
            // End of variable
            '}' if in_variable => {
                in_variable = false;
                LexItem::EndVariable
            },
            // Start of word
            x => LexItem::Word(get_word(x, &mut it, in_variable)),
        };
        result.push(item);
    }
    return result;
}

fn get_word(first: char, iter: &mut Peekable<Chars>, in_variable: bool) -> String {
    let mut result = String::new();
    result.push(first);
    while let Some(c) = iter.peek() {
        if c == &' ' {
            break
        }
        if !in_variable && c == &'$' {
            break;
        }
        let c = iter.next().unwrap();
        if in_variable && c == '}' {
            break;
        }
        result.push(c);
    }
    return result;
}

#[cfg(test)]
mod test {

}
