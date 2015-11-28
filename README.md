# Rust Simple C Compiler

### 这是一个使用Rust开发的simple c的编译器。

## 进度
2015.11.15 初始化项目。<br>
2015.11.26 基本实现各种词法单元的DFA。<br>
2015.11.28 完成编译器前端中词法分析器的模块。<br>

## 语言说明
### 关键字
	
	void if else int char string
	
	这里的关键字string并不是c语言中的，只是因为当前编译器的词法分析模块中并不支持数组，为了识别字符串类型，暂时加入这个关键字，
	等到优化完当前的词法分析器才会支持数组，然后这个关键字就会被取消。
	
### 标识符

	符合C的标识符规则

####运算符

	比较运算符：< <= > >= != = == 
	算术运算符：+ += - -= * *= / /=
	逻辑运算符：! 目前支持取反，暂时不支持与、或运算
	
### 值

	这里需要根据后面语法分析器的需求，来确定这里具体需要支持的值类型。
	目前可以识别整型、浮点型(虽然现在还没有这样的关键字)、字符型、字符串。
	
### 分隔符

	同C语言
	{ } ( ) ;
	暂时只支持这么多，O(∩_∩)O~
	
### 注释
	
	同C语言
	支持行内注释和块注释
	/* */  //

## 安装

```
	git clone https://github.com/buchenglei/rust-simple-c-compiler.git
	cd rust-simple-c-compiler
	cargo run
```
注意：开发使用的Rust编译器版本是1.3

## 测试
目前只能使用固定路径，请将源代码文件放在这: "D:/foo.c"

测试代码：

```C
/*
   this is block comment!
 */
void main() {
	int a = 1;
	a += 1;
	// this is inline comment!
	if a == 1 {
		printf("hello");
	} else {
		printf("word");
	}
}
```

输出结果：

Token { w: VOID, t: Id, row: 4, col: 1 }<br>
Token { w: Id("main"), t: Id, row: 4, col: 6 }<br>
Token { w: LParenthesis, t: Separator, row: 4, col: 10 }<br>
Token { w: RParenthesis, t: Separator, row: 4, col: 11 }<br>
Token { w: LBrace, t: Separator, row: 4, col: 13 }<br>
Token { w: INT, t: Id, row: 5, col: 2 }<br>
Token { w: Id("a"), t: Id, row: 5, col: 6 }<br>
Token { w: EQ, t: Operator, row: 5, col: 8 }<br>
Token { w: Value("1"), t: Value, row: 5, col: 10 }<br>
Token { w: Semicolon, t: Separator, row: 5, col: 11 }<br>
Token { w: Id("a"), t: Id, row: 6, col: 2 }<br>
Token { w: ADDE, t: Operator, row: 6, col: 4 }<br>
Token { w: Value("1"), t: Value, row: 6, col: 7 }<br>
Token { w: Semicolon, t: Separator, row: 6, col: 8 }<br>
Token { w: IF, t: Id, row: 8, col: 2 }<br>
Token { w: Id("a"), t: Id, row: 8, col: 5 }<br>
Token { w: EE, t: Operator, row: 8, col: 7 }<br>
Token { w: Value("1"), t: Value, row: 8, col: 10 }<br>
Token { w: LBrace, t: Separator, row: 8, col: 12 }<br>
Token { w: Id("printf"), t: Id, row: 9, col: 3 }<br>
Token { w: LParenthesis, t: Separator, row: 9, col: 9 }<br>
Token { w: Value("\"hello\""), t: Value, row: 9, col: 10 }<br>
Token { w: RParenthesis, t: Separator, row: 9, col: 17 }<br>
Token { w: Semicolon, t: Separator, row: 9, col: 18 }<br>
Token { w: RBrace, t: Separator, row: 10, col: 2 }<br>
Token { w: ELSE, t: Id, row: 10, col: 4 }<br>
Token { w: LBrace, t: Separator, row: 10, col: 9 }<br>
Token { w: Id("printf"), t: Id, row: 11, col: 3 }<br>
Token { w: LParenthesis, t: Separator, row: 11, col: 9 }<br>
Token { w: Value("\"word\""), t: Value, row: 11, col: 10 }<br>
Token { w: RParenthesis, t: Separator, row: 11, col: 16 }<br>
Token { w: Semicolon, t: Separator, row: 11, col: 17 }<br>
Token { w: RBrace, t: Separator, row: 12, col: 2 }<br>
Token { w: RBrace, t: Separator, row: 13, col: 1 }<br>

-----
共发现 34 个词法单元

## 更多
[Rust官方网站](https://www.rust-lang.org/)<br>
[Rust文档](https://doc.rust-lang.org/)