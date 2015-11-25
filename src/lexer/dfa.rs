use lexer::token::{Word, WordType, Token};
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
	if c == ' ' {
		return Some(dfa_whitespace)
	}
    if c.is_numeric() {
        return Some(dfa_num)
    }
	
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
			if c == ' ' {
				State::MoveTo(1)
			} else {
				State::Unaccepted
			}
		}
		1 => {
			if c == ' ' {
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
