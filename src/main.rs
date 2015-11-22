// 词法分析器模块
mod lexer;

use lexer::file::Source;
use lexer::nfa;

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
    run_lexer();
}

fn run_lexer() {
    let mut f = Source::new("foo.txt");
    let mut nfa: Option<nfa::NFA> = None;
    let mut change_nfa: bool = true;
    let mut status: nfa::Status = nfa::Status::MoveTo(0);
    while let Some(c) = f.next_char() {
        if change_nfa {
            nfa = nfa::choose_nfa(c);
        }
        match status {
            nfa::Status::Accepted => {
                // 生成token
                // ...
                change_nfa = true;
                println!("accept");
            },
            nfa::Status::Unaccepted => println!("Error!!!"),
            nfa::Status::MoveTo(s) => {
                change_nfa = false;
                status = nfa.unwrap()(s, c);
                println!("{:?}", status);
            }
        }
    }
}
