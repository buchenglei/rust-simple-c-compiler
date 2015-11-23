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