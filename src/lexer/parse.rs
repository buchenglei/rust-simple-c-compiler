use lexer::file::{Source};
use lexer::dfa;
use lexer::token::{Word, Token};

// 执行词法分析器parse
pub fn run(filepath: &str) -> Vec<Token>{
    let mut f = Source::new(filepath);
	// 识别相应词法单元的函数，由choose_dfa生成相应的识别函数
    let mut dfa: Option<dfa::DFA> = None;
	// 当一个词素识别完成后，这是重新生成dfa识别函数的的标志
    let mut change_dfa: bool = true;
	// 当前状态
    let mut state: dfa::State = dfa::State::MoveTo(0);
    // 标记单词在文本中实际的位置
    let mut start_row: u32 = 1;
    let mut start_col: u32 = 1;
	// 词素开始的位置
    let mut start: usize = 0;
	// 词素结束的位置
    let mut end: usize = 0;
	// 匹配到的单词
    let mut word: String;
    // 用于存储Token的Vector
    let mut tokens: Vec<Token> = Vec::new();

    while let Some(c) = f.get_char() {
		// 这里只有当状态为Accepted是才更新dfa选择函数
        if change_dfa {
            // nc == next char
            if let Some(nc) = f.look_forward() {
                dfa = dfa::choose_dfa(c, nc);
            } else {
                dfa = dfa::choose_dfa(c, ' ');
            }
			// 重置位置
            start = f.get_pointer();
            // 重置状态
            state = dfa::State::MoveTo(0);
            // 重新选择标DFA的地方就是标记单词开始的地方
            start_row = f.position().0; 
            start_col = f.position().1;
        }
		// check state
        match state {
            dfa::State::Accepted(ref t) => {
                // 当状态转移到可接受状态时，指针已经超出单词两个位置
                // 当指针回退一个的时候，正好指向单词的后一个字符上
                f.back_pointer();
				// 这里的end就是紧跟在单词后面的第一个字符
                end = f.get_pointer();
				// 这里获取单词时的范围，包含start，但不包含end，遵循rust的语法风格
                word = f.get_word(start, end);
                // 对DFA返回的不同类型的结果做分别处理
                match *t {
                    dfa::WordType::Id => {
                        if Token::is_keyword(&word) {
                            tokens.push(Token::new(
                                Word::Keyword(Word::get_index("kw", word)),
                                start_row,
                                start_col
                            ));
                        } else {
                            tokens.push(Token::new(
                                Word::Id(word),
                                start_row,
                                start_col
                            ));
                        }
                    },
                    dfa::WordType::Operator => {
                        tokens.push(Token::new(
                            Word::Operator(Word::get_index("op", word)),
                            start_row,
                            start_col
                        ));
                    },
                    dfa::WordType::Separator => {
                        tokens.push(Token::new(
                            Word::Separator(Word::get_index("sp", word)),
                            start_row,
                            start_col
                        ));
                    },
                    dfa::WordType::Value => {
                        tokens.push(Token::new(
                            Word::Value(word),
                            start_row,
                            start_col
                        ));
                    },
                    _ => (),
                }
				// 已完成一个单词的识别，需要重新选择dfa识别函数
                change_dfa = true; 
            },        
            dfa::State::Unaccepted => {
                // 获取该单词在源文件中实际的结束位置
                println!("遇到一个词法错误，在第{}行,第{}列附近，请检查!", start_row, start_col);
                panic!("Miss error, please check!");
            },
            dfa::State::MoveTo(s) => {
				// 状态还在转移中，不需要重新选择识别函数
                change_dfa = false;
				// 需要考虑把这个unwrap弄掉
                state = dfa.unwrap()(s, c);
                // 当DFA的状态转移还没有结束的时候
                // 就将字符指针向后移动一位
                if (c as u8) == Source::newline() {
                    f.add_row();
                    f.init_col();
                } else {
                    f.add_col();
                }
                f.next_pointer();
            }
        }                                                             
    }
    tokens
}
