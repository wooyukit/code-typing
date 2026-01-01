use ratatui::style::Color;

// Syntax highlighting colors (GitHub Dark theme)
pub const SYN_KEYWORD: Color = Color::Rgb(255, 123, 114);    // #ff7b72 - red/coral - keywords
pub const SYN_TYPE: Color = Color::Rgb(255, 166, 87);        // #ffa657 - orange - types
pub const SYN_FUNCTION: Color = Color::Rgb(210, 168, 255);   // #d2a8ff - purple/lavender - functions
pub const SYN_STRING: Color = Color::Rgb(165, 214, 255);     // #a5d6ff - light blue - strings
pub const SYN_NUMBER: Color = Color::Rgb(121, 192, 255);     // #79c0ff - blue - numbers
pub const SYN_COMMENT: Color = Color::Rgb(139, 148, 158);    // #8b949e - gray - comments
pub const SYN_MACRO: Color = Color::Rgb(121, 192, 255);      // #79c0ff - blue - macros
pub const SYN_LIFETIME: Color = Color::Rgb(255, 123, 114);   // #ff7b72 - red/coral - lifetimes
pub const SYN_ATTRIBUTE: Color = Color::Rgb(139, 148, 158);  // #8b949e - gray - attributes
pub const SYN_OPERATOR: Color = Color::Rgb(201, 209, 217);   // #c9d1d9 - light gray - operators
pub const SYN_NORMAL: Color = Color::Rgb(201, 209, 217);     // #c9d1d9 - light gray - normal text

const KEYWORDS: &[&str] = &[
    "fn", "let", "mut", "if", "else", "match", "struct", "enum", "impl", "pub",
    "use", "mod", "crate", "super", "self", "Self", "for", "while", "loop",
    "return", "break", "continue", "where", "trait", "type", "const", "static",
    "unsafe", "async", "await", "move", "ref", "in", "as", "dyn", "true", "false",
    "Some", "None", "Ok", "Err",
];

const TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64", "bool", "char", "str",
    "String", "Vec", "Option", "Result", "Box", "Rc", "Arc", "RefCell", "Cell",
    "HashMap", "HashSet", "BTreeMap", "BTreeSet", "VecDeque", "LinkedList",
    "Mutex", "RwLock", "Cow", "Pin", "PhantomData",
    "Iterator", "IntoIterator", "FromIterator", "Ord", "PartialOrd", "Eq", "PartialEq",
    "Clone", "Copy", "Default", "Debug", "Display", "Hash", "Send", "Sync",
    "From", "Into", "AsRef", "AsMut", "Deref", "DerefMut", "Drop", "Fn", "FnMut", "FnOnce",
    "Add", "Sub", "Mul", "Div", "Index", "IndexMut",
];

/// Token types for syntax highlighting
#[derive(Clone, Copy, PartialEq)]
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

/// Compute syntax highlighting for each character position in the code
pub fn highlight(code: &str) -> Vec<TokenType> {
    let chars: Vec<char> = code.chars().collect();
    let mut result = vec![TokenType::Normal; chars.len()];
    let mut i = 0;

    while i < chars.len() {
        let remaining = &code[char_index_to_byte_index(code, i)..];

        // Skip whitespace
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }

        // Line comments
        if remaining.starts_with("//") {
            let start = i;
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            for j in start..i {
                result[j] = TokenType::Comment;
            }
            continue;
        }

        // Block comments
        if remaining.starts_with("/*") {
            let start = i;
            i += 2;
            while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2; // skip */
            for j in start..i.min(chars.len()) {
                result[j] = TokenType::Comment;
            }
            continue;
        }

        // Attributes (#[...])
        if chars[i] == '#' && i + 1 < chars.len() && chars[i + 1] == '[' {
            let start = i;
            let mut depth = 0;
            while i < chars.len() {
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
            for j in start..i.min(chars.len()) {
                result[j] = TokenType::Attribute;
            }
            continue;
        }

        // Strings
        if chars[i] == '"' {
            let start = i;
            i += 1;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1; // skip escape
                }
                i += 1;
            }
            i += 1; // skip closing quote
            for j in start..i.min(chars.len()) {
                result[j] = TokenType::String;
            }
            continue;
        }

        // Char literals
        if chars[i] == '\'' && i + 1 < chars.len() {
            // Check if it's a lifetime or char literal
            let maybe_lifetime = i + 1 < chars.len() &&
                (chars[i + 1].is_alphabetic() || chars[i + 1] == '_');

            if maybe_lifetime {
                // Look ahead to see if it's a lifetime (no closing quote nearby)
                let mut j = i + 1;
                while j < chars.len() && (chars[j].is_alphanumeric() || chars[j] == '_') {
                    j += 1;
                }
                // If next char after identifier is not a quote, it's a lifetime
                if j < chars.len() && chars[j] != '\'' {
                    // It's a lifetime
                    let start = i;
                    for k in start..j {
                        result[k] = TokenType::Lifetime;
                    }
                    i = j;
                    continue;
                }
            }

            // It's a char literal
            let start = i;
            i += 1;
            if i < chars.len() && chars[i] == '\\' {
                i += 2; // escape sequence
            } else if i < chars.len() {
                i += 1;
            }
            if i < chars.len() && chars[i] == '\'' {
                i += 1;
            }
            for j in start..i.min(chars.len()) {
                result[j] = TokenType::String;
            }
            continue;
        }

        // Numbers
        if chars[i].is_ascii_digit() || (chars[i] == '.' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit()) {
            let start = i;
            // Handle hex, octal, binary
            if chars[i] == '0' && i + 1 < chars.len() {
                if chars[i + 1] == 'x' || chars[i + 1] == 'o' || chars[i + 1] == 'b' {
                    i += 2;
                }
            }
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_') {
                i += 1;
            }
            for j in start..i {
                result[j] = TokenType::Number;
            }
            continue;
        }

        // Macros (identifier followed by !)
        if chars[i].is_alphabetic() || chars[i] == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();

            // Check if it's a macro (followed by !)
            if i < chars.len() && chars[i] == '!' {
                for j in start..=i {
                    result[j] = TokenType::Macro;
                }
                i += 1;
                continue;
            }

            // Check if it's a keyword
            if KEYWORDS.contains(&word.as_str()) {
                for j in start..i {
                    result[j] = TokenType::Keyword;
                }
                continue;
            }

            // Check if it's a type (starts with uppercase or is a known type)
            if TYPES.contains(&word.as_str()) || (word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) && word != "Some" && word != "None" && word != "Ok" && word != "Err") {
                for j in start..i {
                    result[j] = TokenType::Type;
                }
                continue;
            }

            // Check if it's a function (followed by '(' or preceded by 'fn ')
            // Skip whitespace to check for (
            let mut k = i;
            while k < chars.len() && chars[k] == ' ' {
                k += 1;
            }
            if k < chars.len() && chars[k] == '(' {
                for j in start..i {
                    result[j] = TokenType::Function;
                }
                continue;
            }

            // Check if preceded by "fn " or "::" (method call)
            if start >= 2 {
                let prev: String = chars[start.saturating_sub(3)..start].iter().collect();
                if prev.ends_with("fn ") {
                    for j in start..i {
                        result[j] = TokenType::Function;
                    }
                    continue;
                }
            }
            if start >= 2 && chars[start - 1] == ':' && chars[start - 2] == ':' {
                // After ::, could be function or type
                if k < chars.len() && chars[k] == '(' {
                    for j in start..i {
                        result[j] = TokenType::Function;
                    }
                } else {
                    for j in start..i {
                        result[j] = TokenType::Type;
                    }
                }
                continue;
            }

            // It's a normal identifier
            continue;
        }

        // Operators and punctuation
        if is_operator(chars[i]) {
            result[i] = TokenType::Operator;
            i += 1;
            continue;
        }

        i += 1;
    }

    result
}

fn char_index_to_byte_index(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

fn is_operator(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '!' | '&' | '|' | '^' | '~' | '?' | ':' | ';' | ',' | '.' | '@' | '#' | '$' | '(' | ')' | '[' | ']' | '{' | '}')
}
