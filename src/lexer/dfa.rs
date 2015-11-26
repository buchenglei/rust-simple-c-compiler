use lexer::token::{Word, WordType, Token};
use lexer::file::Source;
pub type DFA = fn(u8, char) -> State;

pub enum State {
	MoveTo(u8), // 待转移状态
	Unaccepted, // 不可接受状态
	Accepted(WordType) // 可接受状态
}

pub fn choose_dfa(c: char) -> Option<DFA> {
	// c是一个字母
	if c.is_alphabetic() {
		return Some(dfa_id);
	}
	if Source::is_invisible_char(c as u8) {
		return Some(dfa_whitespace)
	}
    if c.is_numeric() {
        return Some(dfa_num)
    }
    if c == '\'' {
        return Some(dfa_char)
    }
    if c == '"' {
        return Some(dfa_string)
    }
    if c == '#' {
        return Some(dfa_comments)
    }
    if c == '{' || c =='}' || c == '(' || c == ')' || c == ';' || c == ','{
        return Some(dfa_separator)
    }

    // 由于要检测< > = ...这些字符，比较多，所以就放最后
    // 也就是说运算符是我需要匹配的最后一类词法单元
    return Some(dfa_operator);
	
	// 如果c不是以上任何一个类型的字符，则报错
	panic!("Error: Unrecognized char!");
}

// 识别标识符的DFA
pub fn dfa_id(s: u8, c: char) -> State {
	match s {
		0 => {
			if c == '_' || c.is_alphabetic() {
				State::MoveTo(1)
			} else {
				State::Unaccepted
			}
		}
		// 这是一个终态
		1 => {
			if c == '_' || c.is_alphabetic() ||
				c.is_numeric() {
				State::MoveTo(1)
			} else {
				State::Accepted(WordType::Id)
			}
		}
		_ =>  State::Unaccepted
	}
}

// 匹配空白符的dfa
pub fn dfa_whitespace(s: u8, c: char) -> State {
	match s {
		0 => {
			if Source::is_invisible_char(c as u8) {
				State::MoveTo(1)
			} else {
				State::Unaccepted
			}
		}
		1 => {
			if Source::is_invisible_char(c as u8) {
				State::MoveTo(1)
			} else {
				State::Accepted(WordType::Unknown)
			}
		}
		_ =>  State::Unaccepted
	}
}

// 匹配数字(整形和浮点型)的dfa
pub fn dfa_num(s: u8, c: char) -> State {
    match s {
        0 => {
            if c.is_numeric() {
                State::MoveTo(1)
            } else {
				State::Unaccepted
            }
        },
        1 => {
            if c.is_numeric() {
                State::MoveTo(1)
            } else if c == '.' {
                State::MoveTo(2)
            } else {
                State::Accepted(WordType::Value)
            }
        },
        2 => {
            if c.is_numeric() {
                State::MoveTo(3)
            } else {
				State::Unaccepted
            }
        },
        3 => {
            if c.is_numeric() {
                State::MoveTo(3)
            } else {
                State::Accepted(WordType::Value)
            }
        },
		_ =>  State::Unaccepted
    }
}

// 匹配运算符的DFA
pub fn dfa_operator(s: u8, c: char) -> State {
    match s {
        0 => {
            match c {
                '<' |
                '>' |
                '=' |
                '!' |
                '+' |
                '-' |
                '*' |
                '/' => State::MoveTo(1),
                _ => State::Unaccepted,
            }
        },
        1 => {
            if c == '=' {
               State::MoveTo(2) 
            } else {
                State::Accepted(WordType::Operator)
            }
        },
        2 =>{
            State::Accepted(WordType::Operator)
        }
        _ => State::Unaccepted
    }
}

// 匹配值类型中的字符值
pub fn dfa_char(s: u8, c: char) -> State {
    match s {
        0 => {
            if c == '\'' {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        },
        1 => {
            // ASCII字符从32（空格）到126（~）
            if (c as u8) >= 32 && (c as u8) <= 126 {
                State::MoveTo(2)
            } else {
                State::Unaccepted
            }
        },
        2 => {
            if c == '\'' {
                State::MoveTo(3)
            } else {
                State::Unaccepted
            }
        },
        3 => {
            State::Accepted(WordType::Value)
        }
        _ => State::Unaccepted
    }
}

// 匹配分隔符的DFA
pub fn dfa_separator(s: u8, c: char) -> State {
    match s {
        0 => {
            match c {
                '{' |
                '}' |
                '(' |
                ')' |
                ',' |
                ';' => State::MoveTo(1),
                _ => State::Unaccepted
            }
        },
        1 => {
            State::Accepted(WordType::Separator)
        },
        _ => State::Unaccepted
    }
}

// 匹配值类型中的字符串
fn dfa_string(s: u8, c: char) -> State {
    match s {
        0 => {
            if c == '"' {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        },
        1 => {
            if c != '"' {
                State::MoveTo(1)
            } else {
                State::MoveTo(2)
            }
        },
        2 => {
            State::Accepted(WordType::Value)
        },
        _ => State::Unaccepted
    }
}

// 匹配注释的DFA
pub fn dfa_comments(s: u8, c: char) -> State {
    match s {
        0 => {
            if c == '#' {
                State::MoveTo(1)
            } else {
                State::Unaccepted
            }
        },
        1 => {
            if c == '*' {
                State::MoveTo(2)
            } else if c == '#' {
                State::MoveTo(3)
            } else {
                State::Unaccepted
            }
        },
        2 => {
            if c == '*' {
                State::MoveTo(4)
            } else {
                State::MoveTo(5)
            }
        },
        3 => {
            if (c as u8) == Source::newline() {
                State::MoveTo(7)
            } else {
                State::MoveTo(6)
            }
        },
        4 => {
            if c == '#' {
                State::MoveTo(7)
            } else {
                State::Unaccepted
            }
        },
        5 => {
            if c == '*' {
                State::MoveTo(4)
            } else {
                State::MoveTo(5)
            }
        },
        6 => {
            if (c as u8) == Source::newline() {
                State::MoveTo(7)
            } else {
                State::MoveTo(6)
            }
        },
        7 => {
            State::Accepted(WordType::Unknown)
        }
        _ => State::Unaccepted
    }
}
