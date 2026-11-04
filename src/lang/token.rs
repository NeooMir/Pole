#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Alg,        // alg / algorithm declaration
    Nach,       // begin / algorithm body start
    Kon,        // end / algorithm body finish
    Isp,        // use / import module
    Cel,        // integer type
    Veshch,     // float / real type
    Log,        // boolean type
    Sim,        // char type
    Lit,        // string type
    Esli,       // if statement
    To,         // then branch
    Inache,     // else branch
    Vse,        // fi / end of if block
    Nc,         // loop start (begin cycle)
    Kc,         // loop end (end cycle)
    Poka,       // while loop condition
    Raz,        // times (loop repetition count)
    Dlya,       // for loop
    Ot,         // from (range start)
    Do,         // to (range end)
    Shag,       // step (range increment)
    Vyvod,      // output / print
    Vvod,       // input / read
    Ns,         // newline
    I,          // logical and
    Ili,        // logical or
    Ne,         // logical not
    Da,         // true literal
    Net,        // false literal
    Div,        // integer division
    Mod,        // modulo remainder

    // Robot performer commands & sensors
    Vlevo,      // move left
    Vpravo,     // move right
    Vverh,      // move up
    Vniz,       // move down
    Zakrasit,   // paint current cell
    Sleva,      // at left sensor
    Sprava,     // at right sensor
    Sverhu,     // at top sensor
    Snizu,      // at bottom sensor
    Svobodno,   // free path sensor
    Stena,      // wall obstacle sensor
    Kletka,     // cell
    Zakrashena, // painted cell sensor
    Chistaya,   // clean cell sensor

    // Literals and Identifiers
    Ident(String),
    IntNumber(i64),
    FloatNumber(f64),
    StringLit(String),

    // Symbols & Operators
    Assign,     // :=
    Equal,      // =
    NotEqual,   // <>
    Less,       // <
    Greater,    // >
    LessEq,     // <=
    GreaterEq,  // >=
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    LParen,     // (
    RParen,     // )
    Comma,      // ,
    Colon,      // :
    Semicolon,  // ;

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

pub struct Lexer<'a> {
    chars: Vec<(usize, usize, char)>, // (line, col, char)
    pos: usize,
    _input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = Vec::new();
        let mut line = 1;
        let mut col = 1;

        for ch in input.chars() {
            chars.push((line, col, ch));
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        Self {
            chars,
            pos: 0,
            _input: input,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).map(|&(_, _, c)| c)
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).map(|&(_, _, c)| c)
    }

    fn advance(&mut self) -> Option<(usize, usize, char)> {
        if self.pos < self.chars.len() {
            let item = self.chars[self.pos];
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }

    fn current_pos(&self) -> (usize, usize) {
        if self.pos < self.chars.len() {
            (self.chars[self.pos].0, self.chars[self.pos].1)
        } else if let Some(last) = self.chars.last() {
            (last.0, last.1 + 1)
        } else {
            (1, 1)
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            let (line, col) = self.current_pos();

            // Skip whitespace
            if ch.is_whitespace() {
                self.advance();
                continue;
            }

            // KuMir comments: '|' or '//'
            if ch == '|' || (ch == '/' && self.peek_next() == Some('/')) {
                // Skip until end of line
                while let Some(c) = self.peek() {
                    self.advance();
                    if c == '\n' {
                        break;
                    }
                }
                continue;
            }

            // Number literal
            if ch.is_ascii_digit() {
                tokens.push(self.read_number(line, col)?);
                continue;
            }

            // String literal
            if ch == '"' || ch == '\'' {
                tokens.push(self.read_string(ch, line, col)?);
                continue;
            }

            // Two-character operators
            if ch == ':' && self.peek_next() == Some('=') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::Assign, line, col });
                continue;
            }
            if ch == '<' && self.peek_next() == Some('>') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::NotEqual, line, col });
                continue;
            }
            if ch == '<' && self.peek_next() == Some('=') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::LessEq, line, col });
                continue;
            }
            if ch == '>' && self.peek_next() == Some('=') {
                self.advance();
                self.advance();
                tokens.push(Token { kind: TokenKind::GreaterEq, line, col });
                continue;
            }

            // Single-character symbols and operators
            match ch {
                '=' => { self.advance(); tokens.push(Token { kind: TokenKind::Equal, line, col }); }
                '<' => { self.advance(); tokens.push(Token { kind: TokenKind::Less, line, col }); }
                '>' => { self.advance(); tokens.push(Token { kind: TokenKind::Greater, line, col }); }
                '+' => { self.advance(); tokens.push(Token { kind: TokenKind::Plus, line, col }); }
                '-' => { self.advance(); tokens.push(Token { kind: TokenKind::Minus, line, col }); }
                '*' => { self.advance(); tokens.push(Token { kind: TokenKind::Star, line, col }); }
                '/' => { self.advance(); tokens.push(Token { kind: TokenKind::Slash, line, col }); }
                '(' => { self.advance(); tokens.push(Token { kind: TokenKind::LParen, line, col }); }
                ')' => { self.advance(); tokens.push(Token { kind: TokenKind::RParen, line, col }); }
                ',' => { self.advance(); tokens.push(Token { kind: TokenKind::Comma, line, col }); }
                ':' => { self.advance(); tokens.push(Token { kind: TokenKind::Colon, line, col }); }
                ';' => { self.advance(); tokens.push(Token { kind: TokenKind::Semicolon, line, col }); }
                _ if is_ident_start(ch) => {
                    tokens.push(self.read_ident_or_keyword(line, col));
                }
                _ => {
                    self.advance();
                    // Unknown symbol, skip silently
                }
            }
        }

        let (line, col) = self.current_pos();
        tokens.push(Token { kind: TokenKind::Eof, line, col });
        Ok(tokens)
    }

    fn read_number(&mut self, line: usize, col: usize) -> Result<Token, String> {
        let mut s = String::new();
        let mut is_float = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !is_float && self.peek_next().map_or(false, |next| next.is_ascii_digit()) {
                is_float = true;
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if is_float {
            let val = s.parse::<f64>().map_err(|e| format!("Invalid float number '{}': {}", s, e))?;
            Ok(Token { kind: TokenKind::FloatNumber(val), line, col })
        } else {
            let val = s.parse::<i64>().map_err(|e| format!("Invalid integer number '{}': {}", s, e))?;
            Ok(Token { kind: TokenKind::IntNumber(val), line, col })
        }
    }

    fn read_string(&mut self, quote: char, line: usize, col: usize) -> Result<Token, String> {
        self.advance(); // consume open quote
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if c == quote {
                self.advance(); // consume close quote
                return Ok(Token { kind: TokenKind::StringLit(s), line, col });
            } else if c == '\n' {
                return Err(format!("Unterminated string literal on line {}", line));
            } else {
                s.push(c);
                self.advance();
            }
        }

        Err(format!("Unterminated string literal at EOF on line {}", line))
    }

    fn read_ident_or_keyword(&mut self, line: usize, col: usize) -> Token {
        let mut s = String::new();

        while let Some(c) = self.peek() {
            if is_ident_part(c) {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }

        let lower = s.to_lowercase();
        let kind = match lower.as_str() {
            "алг" | "alg" => TokenKind::Alg,
            "нач" | "begin" => TokenKind::Nach,
            "кон" | "end" => TokenKind::Kon,
            "исп" | "использовать" | "use" => TokenKind::Isp,
            "цел" | "int" => TokenKind::Cel,
            "вещ" | "float" => TokenKind::Veshch,
            "лог" | "bool" => TokenKind::Log,
            "сим" | "char" => TokenKind::Sim,
            "лит" | "string" => TokenKind::Lit,
            "если" | "if" => TokenKind::Esli,
            "то" | "then" => TokenKind::To,
            "иначе" | "else" => TokenKind::Inache,
            "все" | "всё" | "fi" => TokenKind::Vse,
            "нц" | "loop" => TokenKind::Nc,
            "кц" | "pool" => TokenKind::Kc,
            "пока" | "while" => TokenKind::Poka,
            "раз" | "times" => TokenKind::Raz,
            "для" | "for" => TokenKind::Dlya,
            "от" | "from" => TokenKind::Ot,
            "до" | "to" => TokenKind::Do,
            "шаг" | "step" => TokenKind::Shag,
            "вывод" | "print" | "output" => TokenKind::Vyvod,
            "ввод" | "input" => TokenKind::Vvod,
            "нс" | "newline" => TokenKind::Ns,
            "и" | "and" => TokenKind::I,
            "или" | "or" => TokenKind::Ili,
            "не" | "not" => TokenKind::Ne,
            "да" | "true" => TokenKind::Da,
            "нет" | "false" => TokenKind::Net,
            "div" => TokenKind::Div,
            "mod" => TokenKind::Mod,

            // Robot performer keywords
            "влево" | "left" => TokenKind::Vlevo,
            "вправо" | "right" => TokenKind::Vpravo,
            "вверх" | "up" => TokenKind::Vverh,
            "вниз" | "down" => TokenKind::Vniz,
            "закрасить" | "paint" => TokenKind::Zakrasit,
            "слева" | "at_left" => TokenKind::Sleva,
            "справа" | "at_right" => TokenKind::Sprava,
            "сверху" | "at_top" => TokenKind::Sverhu,
            "снизу" | "at_bottom" => TokenKind::Snizu,
            "свободно" | "free" => TokenKind::Svobodno,
            "стена" | "wall" => TokenKind::Stena,
            "клетка" | "cell" => TokenKind::Kletka,
            "закрашена" | "painted" => TokenKind::Zakrashena,
            "чистая" | "clean" => TokenKind::Chistaya,

            // KuMir annotations (ignored)
            "дано" | "надо" | "утв" | "given" | "needed" | "assert" => TokenKind::Ident(lower),

            _ => TokenKind::Ident(s),
        };

        Token { kind, line, col }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_part(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
