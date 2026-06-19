use ratatui::style::Color;

use super::language::{LanguageSpec, SingleQuote};

// Syntax highlighting colors (GitHub Dark theme)
pub const SYN_KEYWORD: Color = Color::Rgb(255, 123, 114); // #ff7b72 - red/coral - keywords
pub const SYN_TYPE: Color = Color::Rgb(255, 166, 87); // #ffa657 - orange - types
pub const SYN_FUNCTION: Color = Color::Rgb(210, 168, 255); // #d2a8ff - purple/lavender - functions
pub const SYN_STRING: Color = Color::Rgb(165, 214, 255); // #a5d6ff - light blue - strings
pub const SYN_NUMBER: Color = Color::Rgb(121, 192, 255); // #79c0ff - blue - numbers
pub const SYN_COMMENT: Color = Color::Rgb(139, 148, 158); // #8b949e - gray - comments
pub const SYN_MACRO: Color = Color::Rgb(121, 192, 255); // #79c0ff - blue - macros
pub const SYN_LIFETIME: Color = Color::Rgb(255, 123, 114); // #ff7b72 - red/coral - lifetimes
pub const SYN_ATTRIBUTE: Color = Color::Rgb(139, 148, 158); // #8b949e - gray - attributes
pub const SYN_OPERATOR: Color = Color::Rgb(201, 209, 217); // #c9d1d9 - light gray - operators
pub const SYN_NORMAL: Color = Color::Rgb(201, 209, 217); // #c9d1d9 - light gray - normal text

/// Token types for syntax highlighting
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TokenType {
    Keyword,
    Type,
    Function,
    String,
    Number,
    Comment,
    Macro,
    Lifetime,
    Attribute,
    Operator,
    Normal,
}

impl TokenType {
    pub fn color(self) -> Color {
        match self {
            TokenType::Keyword => SYN_KEYWORD,
            TokenType::Type => SYN_TYPE,
            TokenType::Function => SYN_FUNCTION,
            TokenType::String => SYN_STRING,
            TokenType::Number => SYN_NUMBER,
            TokenType::Comment => SYN_COMMENT,
            TokenType::Macro => SYN_MACRO,
            TokenType::Lifetime => SYN_LIFETIME,
            TokenType::Attribute => SYN_ATTRIBUTE,
            TokenType::Operator => SYN_OPERATOR,
            TokenType::Normal => SYN_NORMAL,
        }
    }
}

/// Compute syntax highlighting for each character position in `code`, using the
/// rules described by `spec`. Returns one `TokenType` per character so the result
/// aligns index-for-index with the code's `Vec<char>`.
pub fn highlight(code: &str, spec: &LanguageSpec) -> Vec<TokenType> {
    let chars: Vec<char> = code.chars().collect();
    let n = chars.len();
    let mut result = vec![TokenType::Normal; n];
    let mut i = 0;

    while i < n {
        let c = chars[i];

        // Whitespace
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // Line comments
        if !spec.line_comment.is_empty() && matches_at(&chars, i, spec.line_comment) {
            let start = i;
            while i < n && chars[i] != '\n' {
                i += 1;
            }
            fill(&mut result, start, i, TokenType::Comment);
            continue;
        }

        // Block comments
        if let Some((open, close)) = spec.block_comment {
            if matches_at(&chars, i, open) {
                let start = i;
                i += open.chars().count();
                while i < n && !matches_at(&chars, i, close) {
                    i += 1;
                }
                if i < n {
                    i += close.chars().count();
                }
                fill(&mut result, start, i.min(n), TokenType::Comment);
                continue;
            }
        }

        // Rust attributes (#[...])
        if spec.rust_attributes && c == '#' && i + 1 < n && chars[i + 1] == '[' {
            let start = i;
            let mut depth = 0;
            while i < n {
                if chars[i] == '[' {
                    depth += 1;
                } else if chars[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            fill(&mut result, start, i.min(n), TokenType::Attribute);
            continue;
        }

        // C/C++ preprocessor directives (#include, #define, ...)
        if spec.preprocessor && c == '#' {
            let start = i;
            i += 1; // '#'
            while i < n && (chars[i] == ' ' || chars[i] == '\t') {
                i += 1;
            }
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            fill(&mut result, start, i, TokenType::Macro);
            continue;
        }

        // Decorators / annotations (@Override, @decorator)
        if spec.decorators
            && c == '@'
            && i + 1 < n
            && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_')
        {
            let start = i;
            i += 1; // '@'
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '.') {
                i += 1;
            }
            fill(&mut result, start, i, TokenType::Attribute);
            continue;
        }

        // Double-quoted strings
        if c == '"' {
            i = scan_string(&chars, i, '"', &mut result);
            continue;
        }

        // Backtick template strings (JS/TS)
        if spec.backtick_string && c == '`' {
            i = scan_string(&chars, i, '`', &mut result);
            continue;
        }

        // Single quotes: lifetime, char literal, or string depending on language
        if c == '\'' {
            if spec.lifetimes {
                // Rust: distinguish a lifetime from a char literal
                let maybe_lifetime =
                    i + 1 < n && (chars[i + 1].is_alphabetic() || chars[i + 1] == '_');
                if maybe_lifetime {
                    let mut j = i + 1;
                    while j < n && (chars[j].is_alphanumeric() || chars[j] == '_') {
                        j += 1;
                    }
                    // If the identifier is not closed by a quote, it's a lifetime
                    if j < n && chars[j] != '\'' {
                        fill(&mut result, i, j, TokenType::Lifetime);
                        i = j;
                        continue;
                    }
                }
                i = scan_char_literal(&chars, i, &mut result);
                continue;
            }
            match spec.single_quote {
                SingleQuote::Char => {
                    i = scan_char_literal(&chars, i, &mut result);
                }
                SingleQuote::Str => {
                    i = scan_string(&chars, i, '\'', &mut result);
                }
                SingleQuote::None => {
                    result[i] = TokenType::Operator;
                    i += 1;
                }
            }
            continue;
        }

        // Numbers (incl. hex/octal/binary prefixes)
        if c.is_ascii_digit() || (c == '.' && i + 1 < n && chars[i + 1].is_ascii_digit()) {
            let start = i;
            if c == '0' && i + 1 < n && matches!(chars[i + 1], 'x' | 'X' | 'o' | 'O' | 'b' | 'B') {
                i += 2;
            }
            while i < n && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_')
            {
                i += 1;
            }
            fill(&mut result, start, i, TokenType::Number);
            continue;
        }

        // Identifiers: macros, keywords, types, functions, or plain identifiers
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < n && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            // Rust macro invocation: ident!
            if spec.macros && i < n && chars[i] == '!' {
                fill(&mut result, start, i + 1, TokenType::Macro);
                i += 1;
                continue;
            }

            if spec.keywords.contains(&word.as_str()) {
                fill(&mut result, start, i, TokenType::Keyword);
                continue;
            }

            // A type is a known type name or any uppercase-initial identifier
            let is_type = spec.types.contains(&word.as_str())
                || word
                    .chars()
                    .next()
                    .map(|ch| ch.is_uppercase())
                    .unwrap_or(false);
            if is_type {
                fill(&mut result, start, i, TokenType::Type);
                continue;
            }

            // Function: identifier followed by '(' (skipping spaces)
            let mut k = i;
            while k < n && chars[k] == ' ' {
                k += 1;
            }
            if k < n && chars[k] == '(' {
                fill(&mut result, start, i, TokenType::Function);
                continue;
            }

            // After a `::` path separator: function if it precedes '(', else a type
            if start >= 2 && chars[start - 1] == ':' && chars[start - 2] == ':' {
                if k < n && chars[k] == '(' {
                    fill(&mut result, start, i, TokenType::Function);
                } else {
                    fill(&mut result, start, i, TokenType::Type);
                }
                continue;
            }

            // Plain identifier
            continue;
        }

        // Operators and punctuation
        if is_operator(c) {
            result[i] = TokenType::Operator;
            i += 1;
            continue;
        }

        i += 1;
    }

    result
}

/// Set `result[start..end]` to `t`.
fn fill(result: &mut [TokenType], start: usize, end: usize, t: TokenType) {
    for slot in &mut result[start..end] {
        *slot = t;
    }
}

/// True if `chars` starting at `i` matches the (ASCII) pattern `pat`.
fn matches_at(chars: &[char], i: usize, pat: &str) -> bool {
    let mut k = i;
    for pc in pat.chars() {
        if k >= chars.len() || chars[k] != pc {
            return false;
        }
        k += 1;
    }
    true
}

/// Color a string literal opening at `i` with delimiter `quote`; returns the index
/// just past the closing delimiter (handling backslash escapes).
fn scan_string(chars: &[char], i: usize, quote: char, result: &mut [TokenType]) -> usize {
    let n = chars.len();
    let mut j = i + 1;
    while j < n && chars[j] != quote {
        if chars[j] == '\\' && j + 1 < n {
            j += 1; // skip the escaped character
        }
        j += 1;
    }
    if j < n {
        j += 1; // closing delimiter
    }
    let end = j.min(n);
    fill(result, i, end, TokenType::String);
    end
}

/// Color a char literal opening at `i`; returns the index just past the close.
fn scan_char_literal(chars: &[char], i: usize, result: &mut [TokenType]) -> usize {
    let n = chars.len();
    let mut j = i + 1;
    if j < n && chars[j] == '\\' {
        j += 2; // escape sequence, e.g. '\n'
    } else if j < n {
        j += 1;
    }
    if j < n && chars[j] == '\'' {
        j += 1;
    }
    let end = j.min(n);
    fill(result, i, end, TokenType::String);
    end
}

fn is_operator(c: char) -> bool {
    matches!(
        c,
        '+' | '-'
            | '*'
            | '/'
            | '%'
            | '='
            | '<'
            | '>'
            | '!'
            | '&'
            | '|'
            | '^'
            | '~'
            | '?'
            | ':'
            | ';'
            | ','
            | '.'
            | '@'
            | '#'
            | '$'
            | '('
            | ')'
            | '['
            | ']'
            | '{'
            | '}'
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::language::{Language, ALL};

    /// The highlighter must emit exactly one token per character for every sample
    /// in every language — this is what keeps coloring aligned with the cursor.
    #[test]
    fn one_token_per_char_for_all_samples() {
        for &lang in ALL {
            let spec = lang.spec();
            for (code, _) in spec.samples {
                let tokens = highlight(code, spec);
                assert_eq!(
                    tokens.len(),
                    code.chars().count(),
                    "token/char misalignment in {}",
                    lang.name()
                );
            }
        }
    }

    #[test]
    fn rust_keyword_function_and_comment() {
        let code = "fn main() {} // hi";
        let tokens = highlight(code, Language::Rust.spec());
        assert_eq!(tokens[0], TokenType::Keyword); // 'fn'
        assert_eq!(tokens[code.find("main").unwrap()], TokenType::Function);
        assert_eq!(tokens[code.find("//").unwrap()], TokenType::Comment);
    }

    #[test]
    fn rust_macro_and_string() {
        let code = "println!(\"hi\");";
        let tokens = highlight(code, Language::Rust.spec());
        assert_eq!(tokens[0], TokenType::Macro);
        assert_eq!(tokens[code.find('"').unwrap()], TokenType::String);
    }

    #[test]
    fn python_hash_comment_is_not_rust_attribute() {
        let code = "def f():\n    # note\n    pass";
        let tokens = highlight(code, Language::Python.spec());
        assert_eq!(tokens[0], TokenType::Keyword); // 'def'
        assert_eq!(tokens[code.find('#').unwrap()], TokenType::Comment);
    }

    #[test]
    fn c_preprocessor_is_macro() {
        let code = "#include <stdio.h>";
        let tokens = highlight(code, Language::C.spec());
        assert_eq!(tokens[0], TokenType::Macro); // '#'
    }
}
