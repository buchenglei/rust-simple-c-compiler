use std::io::prelude::*;
use std::fs::File;
use std::ascii::AsciiExt;

#[derive(Debug)]
pub struct Source {
	buffer: Vec<u8>,
	pointer: usize,
	len: usize,
	//filename: &str,
}

impl Source {
	pub fn new(path: &str) -> Source {
		//加载源代码文件，打开失败则报错
		let mut source_file = File::open(path).ok().expect("Error: can't open this file");
		// 将所有源代码读入到缓冲区中
		//let mut buf = String::new();
		let mut buf:Vec<u8> = Vec::new();
		source_file.read_to_end(&mut buf).ok().expect("Error: can't read file content");
		// 构造Source，并返回
		Source {
			pointer: 0,
			len: buf.len(),
			//filename: _
			buffer: buf,
 		}
	}
	
	pub fn len(&self) -> usize { self.len }
	
	// 判断当前符号是否为空白符（或者多个重复字符只需要保留一个的情况）
	fn is_invisible_char(c: u8) -> bool {
		match c {
			// 空格
			32 => true,
			// \r\n
			10 | 13 => true,
				// tab
			9 | 11 => true,
			// other
			_ => false
		}
	}
	
	pub fn next_char(&mut self) -> Option<char> {
		let mut c: u8;
		while self.pointer < self.len {
			c = self.buffer[self.pointer];
			if c.is_ascii() {
				if Source::is_invisible_char(c) {
					// 这里处理空白符号的情况
					self.pointer += 1;
					// 当下一个字符不是空白符时，则返一个空格
					if !Source::is_invisible_char(self.buffer[self.pointer]) {
						return Some(' ')
					}
				} else {
					// 这里处理非空白符号的情况
					self.pointer += 1;
					return Some(c as char);
				}
			} else {
				panic!("Error: now, the program can't support this charset, please use ASCII");
			}
		}
		None
	}
	
	pub fn back_pointer(&mut self) {
		let mut c: u8;
		if self.pointer >= self.len {
			self.pointer = self.len -1;
		}		
		while self.pointer >= 0usize {
			self.pointer -= 1;
			c = self.buffer[self.pointer];
			if c.is_ascii() {
				if Source::is_invisible_char(c) &&
					Source::is_invisible_char(self.buffer[self.pointer - 1]) {
					// do nothing
					// don't add any code at here!!!
				} else {
					return;
				}
			}
		}
		self.pointer = 0;
	}
}