use lexer::file;
use lexer::dfa;
use lexer::token::{WordType, Token};

// 执行词法分析器parse
pub fn run(filepath: &str) {
    let mut f = file::Source::new(filepath);
	// 识别相应词法单元的函数，由choose_dfa生成相应的识别函数
    let mut dfa: Option<dfa::DFA> = None;
	// 当一个词素识别完成后，这是重新生成dfa识别函数的的标志
    let mut change_dfa: bool = true;
	// 当前状态
    let mut state: dfa::State = dfa::State::MoveTo(0);
	// 词素开始的位置
    let mut start: usize = 0;
	// 词素结束的位置
    let mut end: usize = 0;
	// 匹配到的单词
    let mut word: String;

    while let Some(c) = f.get_char() {
		// 这里只有当状态为Accepted是才更新dfa选择函数
        if change_dfa {
            dfa = dfa::choose_dfa(c);
			// 重置位置
            start = f.get_pointer();
            // 重置状态
            state = dfa::State::MoveTo(0);
        }
		// check state
        match state {
            dfa::State::Accepted(ref t) => {
                // 当状态转移到可接受状态时，指针已经超出单词两个位置
                // 当指针回退一个的时候，正好指向单词的后一个字符上
                f.back_pointer();
				// 这里的end就是紧跟在单词后面的第一个字符
                end = f.get_pointer();
                //match *t {
                //    WordType::Unknown => continue,
               //     _ => {println!("test");},
                //}
				// 这里获取单词时的范围，包含start，但不包含end，遵循rust的语法风格
                word = f.get_word(start, end);
                println!("Accept word is |{}|, is keyword? {}", &word, Token::is_keyword(&word));
				// 已完成一个单词的识别，需要重新选择dfa识别函数
                change_dfa = true; 
            },        
			// 总感觉按照现在的程序逻辑，是不可能执行到这个分支的!!!!!!!!!!!!
            dfa::State::Unaccepted => println!("Error!!!"),
            dfa::State::MoveTo(s) => {
				// 状态还在转移中，不需要重新选择识别函数
                change_dfa = false;
				// 需要考虑把这个unwrap弄掉
                state = dfa.unwrap()(s, c);
                // 当DFA的状态转移还没有结束的时候
                // 就将字符指针向后移动一位
                f.next_pointer();
            }
        }                                                             
    }
}