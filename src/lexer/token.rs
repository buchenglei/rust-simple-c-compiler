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
	// 定义标识符
	Id(&'static str),
	// 定义值
	Value(&'static str),
}

// 定义单词的类型
pub enum WordType {
	Keyword,
	Operator,
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
	fn new(w: Word, r: u32, c: u32) -> Token {
		Token { w: w, row: r, col: c }
	}
	
	// 获得当前token中的单词的类型
	fn get_type(&self) -> WordType {
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
			Word::Id(_) => WordType::Id,
			Word::Value(_) => WordType::Value,
		}
	}
	
	// 返回该token中单词的字符串形式
	fn get_str(&self) -> &str {
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
			Word::Id(ref s) => s,
			Word::Value(ref s) => s,
		}
	}
	
	// 返回当前token中单词在源文件中的位置
	fn get_position(&self) -> (u32, u32) {
		(self.row, self.col)
	}
}