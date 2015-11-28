// 词法分析器模块
mod lexer;

fn main() {
    let tokens = lexer::parse::run("d:/foo.c");
    for i in &tokens {
        println!("{:?}", i);
    }
    println!("-----\n共发现 {} 个词法单元", tokens.len());
}
