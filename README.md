# Rust Simple C Compiler

### 这是一个使用Rust开发的simple c的编译器。

## 进度
基本实现各种词法单元的DFA。

## 测试

测试文件：
```C
void main() {
	printf("hello world");
}
```
输出：

>Accept word is |void|, it is Id, but it is Keyword? true <BR>
Accept word is |main|, it is Id, but it is Keyword? false<BR>
Accept word is |(|, it is Separator<BR>
Accept word is |)|, it is Separator<BR>
Accept word is |{|, it is Separator<BR>
Accept word is |printf|, it is Id, but it is Keyword? false<BR>
Accept word is |(|, it is Separator<BR>
Accept word is |"hello world"|, it is Value<BR>
Accept word is |)|, it is Separator<BR>
Accept word is |;|, it is Separator<BR>
Accept word is |}|, it is Separator<BR>