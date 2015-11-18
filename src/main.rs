// 词法分析器模块
mod lexer;

use lexer::file::Source;

fn main() {
    let mut f = Source::new("foo.txt");
    while let Some(c) = f.next_char() {
        if c != ' ' {
            let (x, y) = f.position();
            println!("pos:{}-{}, {}", x, y,c);
        }
    }
    println!(" ");
    f.back_pointer();
    f.back_pointer();
    f.back_pointer();
    println!("{:?}", f.next_char());
    
}
