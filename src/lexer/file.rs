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
	
	pub fn next_char(&mut self) -> Option<char> {
		let mut c: u8;
		while self.pointer < self.len {
			c = self.buffer[self.pointer];
			if c.is_ascii() {
				match c {
					32 => self.pointer += 1,
					10 | 13 => self.pointer += 1,
					_ => {
						self.pointer += 1;
						return Some(c as char);
					},
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
				match c {
					32 => {1;},
					10 | 13 => {1;},
					_ => {
						return;
					},
				}
			
			}
		}
		self.pointer = 0;
	}
}