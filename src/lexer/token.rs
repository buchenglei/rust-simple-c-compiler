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

// 定义单词的种类
pub enum Word {
	// 定义关键字
	IF,
	ELSE,
	INT,
	VOID,
	CHAR,
	STRING,
	// 定义操作符
	LT, // <
	LE, // <=
	EQ, // =
	EE, // ==
	NE, // !=
	GT, // >
	GE, // >=
	ADD, // +
	SUB, // -
	// 定义分隔符
	LBrace, // {
	RBrace, // }
	LParenthesis, // (
	RParenthesis, // )
	Semicolon, // ;
	// 定义标识符
	Id(&'static str),
	// 定义值
	Value(&'static str),
}

const N: usize = 6;
const keyword: [&'static str; N] = [
	"if",
	"else",
	"int",
	"void",
	"char",
	"string"
];

// 定义单词的类型
pub enum WordType {
	Keyword,
	Operator,
	Separator, // 分隔符
	Id,
	Value, // 值类型
	Unknown
}

// 定义一个持有单词的Token
pub struct Token {
	w: Word,		// 该Token持有的单词
	row: u32,		// 单词开头所在的行
	col: u32,		// 单词开头所在的列
}

impl Token {
	pub fn new(w: Word, r: u32, c: u32) -> Token {
		Token { w: w, row: r, col: c }
	}
	
	// 获得当前token中的单词的类型
	pub fn get_type(&self) -> WordType {
		match self.w {
			Word::IF |
			Word::ELSE |
			Word::VOID |
			Word::INT |
			Word::CHAR |
			Word::STRING => WordType::Keyword,
			Word::LT |
			Word::LE |
			Word::EQ |
			Word::EE |
			Word::NE |
			Word::GT |
			Word::GE |
			Word::ADD |
			Word::SUB => WordType::Operator,
			Word::LBrace |
			Word::RBrace |
			Word::LParenthesis |
			Word::RParenthesis |
			Word::Semicolon => WordType::Separator,
			Word::Id(_) => WordType::Id,
			Word::Value(_) => WordType::Value,
		}
	}
	
	// 返回该token中单词的字符串形式
	pub fn get_str(&self) -> &str {
		match self.w {
			Word::IF => "if",
			Word::ELSE => "else",
			Word::VOID => "void",
			Word::INT => "int",
			Word::CHAR => "char",
			Word::STRING => "string",
			Word::LT => "<",
			Word::LE => "<=",
			Word::EQ => "=",
			Word::EE => "==",
			Word::NE => "!=",
			Word::GT => ">",
			Word::GE => ">=",
			Word::ADD => "+",
			Word::SUB => "-",
			Word::LBrace => "{",
			Word::RBrace => "}",
			Word::LParenthesis => "(",
			Word::RParenthesis => ")",
			Word::Semicolon => ";",
			Word::Id(ref s) => s,
			Word::Value(ref s) => s,
		}
	}
	
	// 返回当前token中单词在源文件中的位置
	pub fn get_position(&self) -> (u32, u32) {
		(self.row, self.col)
	}
	
	// 判断字符串是否是关键字
	pub fn is_keyword(word: &str) -> bool {
		for i in 0..N {
			if keyword[i] == word {
				return true;
			}
		}
		
		false
	}
}
