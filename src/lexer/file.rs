use std::io::prelude::*;
use std::fs::File;
use std::ascii::AsciiExt;

#[derive(Debug)]
pub struct Source {
	buffer: Vec<u8>,
	pointer: usize,
	len: usize,
	row: u32,
	col: u32,
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
		// 因为在后面读取字符的时候，每个换行符都会被换成空格返回
		// 在后面的逻辑中，字符流的结尾至少需要加2个空格才能正常工作
		buf.push(32);
		buf.push(32);
		// 构造Source，并返回
		Source {
			pointer: 0,
			len: buf.len(),
			row: 1,
			col: 0, // 初始位置在第1行，第0列
			buffer: buf,
 		}
	}
	
	pub fn len(&self) -> usize { self.len }
	
	// 判断当前符号是否为空白符（或者多个重复字符只需要保留一个的情况）
	pub fn is_invisible_char(c: u8) -> bool {
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
	
	// 屏蔽不同平台下换行符的差异
	#[cfg(target_os = "windows")]
	pub fn newline() -> u8 { 10 }
	
	#[cfg(target_os = "linux")]
	pub fn newline() -> u8 { 10 }
	
	#[cfg(target_os = "mac")]
	pub fn newline() -> u8 { 13 }
	
	pub fn get_char(&mut self) -> Option<char> {
		let c:u8;
		if self.pointer < self.len {
			c = self.buffer[self.pointer];
		} else {
			return None;
		}
		if c.is_ascii() {
			// 标记字符的位置
			if c == Source::newline() {
				self.row += 1;
				self.col = 0;
			} else {
				self.col += 1;
			}
			// 指针后移一位
			//self.pointer += 1;
			// 将所有空白符替换为空格
			//if Source::is_invisible_char(c) { return Some(' '); }
			// 反回当前字符
			Some(c as char)
		} else {
			None
		}
	}
	
	pub fn back_pointer(&mut self) {
		self.pointer -= 1;
        if self.col != 0 {
            self.col -= 1;
        }
	}
	
	// 返回当前指针处理到的位置
	pub fn position(&self) -> (u32, u32) {
		(self.row, self.col)
	}

    pub fn next_pointer(&mut self) {
        self.pointer += 1
    }
	
	// 获得指定范围的字符并组成一个String返回
	pub fn get_word(&self, start: usize, end: usize) -> String {
		let tmp_vec = &self.buffer[start..end];
		let mut word = String::with_capacity(end - start + 1);
		for c in tmp_vec {
			word.push(*c as char);
		}
		word
	}
	
	pub fn get_pointer(&self) -> usize {
		self.pointer
	}
}
