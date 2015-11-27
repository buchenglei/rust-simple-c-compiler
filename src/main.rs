// 词法分析器模块
mod lexer;

fn main() {
    let tokens = lexer::parse::run("e:/workspace/rustlang/foo.txt");
    for i in &tokens {
        println!("{:?}", i);
    }
}
