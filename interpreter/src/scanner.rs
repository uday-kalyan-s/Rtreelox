use std::collections::HashMap;
use crate::scanner::TokenType::*;

#[derive(Debug)]
pub struct Scanner {
    pub tokens: Vec<TokenType>,
    pub char_index: u64,
    pub line_no: u64,
    state: ScannerState,
    is_commented: bool,
    keyword_map: HashMap<&'static str, TokenType>
}

#[derive(PartialEq, Debug)]
enum ShouldConsume {
    Consume,
    Stay
}
struct MachineOutput {
    consume: ShouldConsume,
    output: Result<Option<TokenType>, String>,
    state: ScannerState
}

impl MachineOutput {
    pub fn cons_out(token: TokenType, state: ScannerState) -> Self {
        MachineOutput { consume: ShouldConsume::Consume, output: Ok(Some(token)), state }
    }
    pub fn cons_none(state: ScannerState) -> Self {
        MachineOutput { consume: ShouldConsume::Consume, output: Ok(None), state }
    }
    pub fn err(error: String) -> Self {
        MachineOutput { consume: ShouldConsume::Stay, output: Err(error), state: ScannerState::Blank }
    }
    pub fn stay_out(token: TokenType, state: ScannerState) -> Self {
        MachineOutput { consume: ShouldConsume::Stay, output: Ok(Some(token)), state }
    }
}

#[derive(Debug)]
enum ScannerState {
    Blank,
    ExpectingDouble(char),
    ExpectingIdentifier(String),
    ExpectingString(String),
    ExpectingNumber(String),
}

impl Default for ScannerState {
    fn default() -> Self {
        Self::Blank
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Scanner {
            tokens: Vec::new(),
            char_index: 0,
            state: ScannerState::Blank,
            is_commented: true,
            keyword_map: HashMap::from([
                ("var", Var),
                ("if", If),
                ("else", Else),
                ("for", For),
                ("while", While),
                ("fun", Fun),
                ("return", Return),
                ("class", Class),
                ("this", This),
                ("super", Super),
                ("true", True),
                ("false", False),
                ("and", And),
                ("or", Or),
                ("nil", Nil),
            ]),
            line_no: 0
        }
    }
}

impl Scanner {
    fn use_char(&mut self, x: char) -> MachineOutput {
        if x == '\n' {
            self.char_index = 0;
            self.line_no += 1;
        }
        let old_state = std::mem::take(&mut self.state);
        match old_state {
                ScannerState::Blank => {
                    match x {
                        // single character tokens
                        '{' => MachineOutput::cons_out(LeftBrace, old_state),
                        '}' => MachineOutput::cons_out(RightBrace, old_state),
                        '(' => MachineOutput::cons_out(LeftParan, old_state),
                        ')' => MachineOutput::cons_out(RightParan, old_state),
                        ',' => MachineOutput::cons_out(Comma, old_state),
                        '.' => MachineOutput::cons_out(Dot, old_state),
                        ';' => MachineOutput::cons_out(Semicolon, old_state),
                        '-' => MachineOutput::cons_out(Minus, old_state),
                        '+' => MachineOutput::cons_out(Plus, old_state),
                        '*' => MachineOutput::cons_out(Star, old_state),
                        // single/double char tokens
                        '>' | '<' | '=' | '!' | '/' => {
                            MachineOutput::cons_none(ScannerState::ExpectingDouble(x))
                        },
                        // any num
                        '0'..'9' => {
                            MachineOutput::cons_none(ScannerState::ExpectingNumber(x.to_string()))
                        },
                        // string
                        '\"' => {
                            MachineOutput::cons_none(ScannerState::ExpectingString(String::new()))
                        },
                        '\n' | ' ' => {
                            MachineOutput::cons_none(old_state)
                        }
                        // any char
                        _ => {
                            MachineOutput::cons_none(ScannerState::ExpectingIdentifier(x.to_string()))
                        }
                    }
                },
                ScannerState::ExpectingDouble(prev) => {
                    if prev == '/' {
                        if x == '/' {
                            self.is_commented = true;
                            return MachineOutput::cons_none(ScannerState::Blank);
                        }
                        else {
                            return MachineOutput::stay_out(Slash, ScannerState::Blank);
                        }
                    }
                    match x {
                        '=' => {
                            match prev {
                                '>' => MachineOutput::cons_out(GtEq, ScannerState::Blank),
                                '<' => MachineOutput::cons_out(LtEq, ScannerState::Blank),
                                '=' => MachineOutput::cons_out(EqEq, ScannerState::Blank),
                                '!' => MachineOutput::cons_out(BangEq, ScannerState::Blank),
                                _ => MachineOutput::err("unmatched token entered double state".to_string())
                            }
                        },
                        _ => {
                            self.state = ScannerState::Blank;
                            match prev {
                                '>' => MachineOutput::stay_out(Gt, ScannerState::Blank),
                                '<' => MachineOutput::stay_out(Lt, ScannerState::Blank),
                                '!' => MachineOutput::stay_out(Bang, ScannerState::Blank),
                                '=' => MachineOutput::stay_out(Eq, ScannerState::Blank),
                                _ => MachineOutput::err("unmatched token entered double state".to_string())
                            }
                        }
                    }
                },
                ScannerState::ExpectingIdentifier(mut str) => {
                    match x {
                        ' ' | '\n' => {
                            self.state = ScannerState::Blank;
                            return if self.keyword_map.contains_key(str.as_str()) {
                                MachineOutput::cons_out(self.keyword_map[str.as_str()].clone(), ScannerState::Blank)
                                // its ok to clone since it doesnt have any string
                            }
                            else {
                                MachineOutput::cons_out(Identifier(str), ScannerState::Blank)
                            }
                        },
                        _ => {
                            str.push(x);
                            MachineOutput::cons_none(ScannerState::ExpectingIdentifier(str))
                        }
                    }
                },
                ScannerState::ExpectingString(mut str) => {
                    match x {
                        '\"' => {
                            MachineOutput::cons_out(TkString(str), ScannerState::Blank)
                        },
                        _ => {
                            str.push(x);
                            MachineOutput::cons_none(ScannerState::ExpectingString(str))
                        }
                    }
                },
                ScannerState::ExpectingNumber(mut num) => {
                    match x {
                        ' ' | '\n' => {
                            let parsed = num.parse();
                            match parsed {
                                Ok(parsed_num) => MachineOutput::cons_out(TokenType::Number(parsed_num), ScannerState::Blank),
                                Err(error) => MachineOutput::err(error.to_string())
                            }
                        }
                        '0'..'9' | '.' => {
                            num.push(x);
                            MachineOutput::cons_none(ScannerState::ExpectingNumber(num))
                        }
                        _ => {
                            MachineOutput::err("number ends with space".to_string())
                        }
                    }
                }
            }
    }
    pub fn scan_line(&mut self, line: String) -> Result<(), String> {
        // let line = self.line.clone();
        let mut iter = line.chars().peekable();
        while let Some(&next_chr) = iter.peek() {
            let output = self.use_char(next_chr);
            self.state = output.state;
            if output.consume == ShouldConsume::Consume {
                iter.next();
                self.char_index += 1;
            }
            match output.output {
                Ok(token) => {
                    if let Some(token_unw) = token {
                        self.tokens.push(token_unw);
                    }
                },
                Err(err) => {
                    return Err(err)
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // single char
    LeftParan, RightParan, LeftBrace, RightBrace,
    Comma, Dot, Semicolon, Minus, Plus, Slash, Star,

    // one or 2 char
    Gt, GtEq,
    Lt, LtEq,
    Eq, EqEq,
    Bang, BangEq,

    // identifier/literal
    Identifier(String), TkString(String), Number(f64),

    // Keyword
    If, Else, For, While, 
    Fun, Return, Class, Var, This, Super,
    True, False, And, Or, Nil,
}