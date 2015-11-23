// 词法分析器模块
mod lexer;

use lexer::file::Source;
use lexer::dfa;

fn main() {
    /*println!("the first poistion at {:?}", f.position());
    while let Some(c) = f.next_char() {
        if c != ' ' {
            let (x, y) = f.position();
            println!("pos:{}-{}, {}", x, y,c);
        }
    }
    println!("the end poistion at {:?}", f.position());
    f.back_pointer();
    println!("{:?}", f.next_char());*/
    run_lexer_dfa();
}

fn run_lexer_dfa() {
    let mut f = Source::new("foo.txt");
    let mut dfa: Option<dfa::DFA> = None;
    let mut change_dfa: bool = true;
    let mut state: dfa::State = dfa::State::MoveTo(0);
    let mut start_col: u32 = 1;
    let mut end_col: u32 = 1;
    let mut word: String;
    while let Some(c) = f.next_char() {
        if change_dfa {
            dfa = dfa::choose_dfa(c);
            start_col = f.position().1;
        }
        match state {
            dfa::State::Accepted(ref t) => {
                f.back_pointer();
                end_col = f.position().1;
                word = f.get_word(start_col as usize, end_col as usize);
                println!("Accept word is |{}|", word);
                change_dfa = true; 
            },
            dfa::State::Unaccepted => println!("Error!!!"),
            dfa::State::MoveTo(s) => {
                change_dfa = false;
                state = dfa.unwrap()(s, c);
            }
        }
    }
}
