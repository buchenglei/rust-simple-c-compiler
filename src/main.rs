// 词法分析器模块
mod lexer;

use lexer::file::Source;

fn main() {
    let mut f = Source::new("foo.txt");
    println!("All char of f:");
    while let Some(c) = f.next_char() {
        print!("{}", c);
    }
    println!(" ");
    f.back_pointer();
    f.back_pointer();
    f.back_pointer();
    println!("{:?}", f.next_char());
    
}
