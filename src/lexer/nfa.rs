/*
use lexer::token::{Word, Token};
pub type NFA = fn(u8, char) -> Status;

#[derive(Debug)]
pub enum Status {
	MoveTo(u8), // 待转移状态
	Unaccepted, // 不可接受状态
	Accepted // 可接受状态
}

pub fn choose_nfa(c: char) -> Option<NFA> {
	// c是一个字母
	if c.is_alphabetic() {
		return Some(nfa_id);
	}
	// 如果c不是以上任何一个类型的字符，则报错
	panic!("Error: Unrecognized char!");
}

// 识别标识符的NFA
pub fn nfa_id(s: u8, c: char) -> Status {
	match s {
		0 => {
			if c.is_alphabetic() || c == '_'{
				Status::MoveTo(1)
			} else {
				Status::Unaccepted
			}
		},
		1 => {
			if c == ' ' {
				Status::MoveTo(3)
			} else {
				Status::MoveTo(2)
			}
		},
		2 => {
			if c == '_' || c.is_alphabetic() ||
				c.is_numeric() {
				Status::MoveTo(2)
			} else {
				Status::MoveTo(3)
			}
		},
		3 => {
			Status::Accepted
		},
		_ => Status::Unaccepted
	}
}
*/
