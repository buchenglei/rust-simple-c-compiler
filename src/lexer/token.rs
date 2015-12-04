#[test]
fn test_token() {
	let token1 = Token::new(Word::VOID, 10, 20);
	match token1.get_type() {
		WordType::Keyword => assert!(true),
		_ => assert!(false),
	}
	assert_eq!("void", token1.get_str());
	assert_eq!((10u32, 20u32), token1.get_position());
	
	let token2 = Token::new(Word::Id("token"), 5, 7);
	match token2.get_type() {
		WordType::Id => assert!(true),
		_ => assert!(false),
	}
	assert_eq!("token", token2.get_str());
	assert_eq!((5u32, 7u32), token2.get_position());
	
	let token3 = Token::new(Word::LParenthesis, 8, 10);
	match token3.get_type() {
		WordType::Separator => assert!(true),
		_ => assert!(false),
	}
	assert_eq!("(", token3.get_str());
	assert_eq!((8u32, 10u32), token3.get_position());
	
}

const N_KW: usize = 7;
const N_OP: usize = 16;
const N_SP: usize = 7;

const keyword: [&'static str; N_KW] = [
	"if", "else", "while",
	"int", "void", "char", "string" 
];
const operator: [&'static str; N_OP] = [
    "<", "<=", ">", ">=", "!=", "==",
    "+", "+=", "-", "-=", "*", "*=", "/", "/=", "=",
    "!"
];
const separator: [&'static str; N_SP] = [
    "(", ")", "{", "}", "[", "]",
    ";"
];

// 定义单词的类型
#[derive(Debug)]
pub enum Word {
    Keyword(usize),
	Operator(usize),
	Separator(usize), // 分隔符
	Id(String),
	Value(String), // 值类型
}

impl Word {
    pub fn get_index(t: &str ,s: String) -> usize {
        match t {
            "kw" => {
                for i in 0..N_KW {
                    if keyword[i] == &s[..] {
                        return i
                    }
                }
            },
            "op" => {
                for i in 0..N_OP {
                    if operator[i] == &s[..] {
                        return i
                    }
                }
            },
            "sp" => {
                for i in 0..N_SP {
                    if separator[i] == &s[..] {
                        return i
                    }
                }
            },
            _ => {
            },
        }
        return 0;
    }
}

// 定义一个持有单词的Token
#[derive(Debug)]
pub struct Token {
	w: Word,		// 该Token持有的单词
	row: u32,		// 单词开头所在的行
	col: u32,		// 单词开头所在的列
}

impl Token {
	pub fn new(word: Word, r: u32, c: u32) -> Token {
		Token { w: word, row: r, col: c }
	}
	
	// 返回当前token中单词在源文件中的位置
	pub fn get_position(&self) -> (u32, u32) {
		(self.row, self.col)
	}
	
	// 判断字符串是否是关键字
	pub fn is_keyword(word: &str) -> bool {
		for i in 0..N_KW {
			if keyword[i] == word {
				return true;
			}
		}
		
		false
	}

}
