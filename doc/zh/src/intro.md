# mdbook-lang 
`mdbook-lang`是一个`mdbook`预处理器插件和多编程语言`playground`服务器，支持在浏览器中通过与`playground`交互、运行`mdbook`电子书嵌入的多种编程语言代码，并展示结果。而该`playground`服务器可以自行部署或本地部署，可以容易扩展到其他编程语言。

本软件受`mdbook`和`mdbook-repl`启发，但`mdbook`基于[https://play.rust-lang.org](https://play.rust-lang.org)实现的playground，目前仅支持`rust`语言；而`mdbook-repl`主要支持`python`、`javascript`和`typescript`等解释型语言，两者都依赖在线`playground`服务器，使得`mdbook`支持的编程语言不易扩展。而本软件借助自主部署的编译器环境为`mdbook`电子书嵌入的`多编程语言代码段`架起浏览器和编译器之间的桥梁，且便于扩展，也给出了多电子书和沙箱安全等配置。

如下电子书中的`C/C++`代码，在安装了`mdbook-lang`主机环境中，是否让其提供支持的区别，使用`mdbook-lang`支持：

```cpp
#include <iostream>
using namespace std;
int main() {
    cout << "Hello, World!" << endl;
    return 0;
}
```

不使用`mdbook-lang`支持：

```cpp,nolang
#include <iostream>
using namespace std;
int main() {
    cout << "Hello, World!" << endl;
    return 0;
}
```
在启用`mdbook-lang`支持的代码中，以`ACE Editor`作为代码编辑器，可对其进行可配置的编辑、重置和运行等。您可以直接在浏览器中修改并执行代码，实时查看输出结果。如果您是教师为学生布置练习，还可以控制剪切、复制和粘贴等操作，并禁用浏览器调试功能，以实现更受控的教学环境。

本软件仍处于持续开发和优化中，未来将支持更多的编程语言。