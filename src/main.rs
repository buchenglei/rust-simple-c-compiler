use std::env;
// 词法分析器模块
mod lexer;

fn main() {
    // 将获得的命令行参数作为编译的文件名
    if env::args().len() != 2 {
        panic!("Usage: ./myC filename.c");
    }
    let filename = match env::args().nth(1) {
        Some(name) => name,
        None => panic!("Get filename error!")
    };
    let tokens = lexer::parse::run(&filename[..]);
    for i in &tokens {
        println!("{:?}", i);
    }
    println!("-----\n共发现 {} 个词法单元", tokens.len());
}
