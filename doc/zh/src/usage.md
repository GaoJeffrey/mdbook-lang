# 用法

本工具是一个`mdbook`预处理插件和可独立运行的`playground`编译和运行服务器。

## 安装

有两种方式可以安装 `mdbook-lang`：

- 如果你已经有 [Rust](https://www.rust-lang.org/tools/install) 环境，可以通过`cargo`工具安装`mdbook-lang`：

```bash
cargo install mdbook-lang
```

- 或者你可以从 [github 页面](https://github.com/GaoJeffrey/mdbook-lang)下载二进制文件，并将该二进制文件最终存放路径加入到用户或全局 `PATH`环境变量中。

你可以通过以下命令检查安装情况：
```bash
mdbook-lang --version
```

## 配置

安装完成后，通过 `mdbook-lang install` 命令自动在书籍的 `book.toml` 文件中配置 `mdbook-lang` 插件，使得现有基于`mdbook`工具的电子书启用该插件，该默认配置根据需要可进行修改。

```bash
$ mdbook-lang install /path/to/your/book
```
该语句将设置 `mdbook-lang` 预处理插件和编译服务器的 `url` 参数。

例如：

```toml
[book]
authors = ["gaoxu.jeffrey"]
language = "en"
multilingual = false
src = "src"
title = "mdbook-lang example"

[preprocessor]

[preprocessor.lang]
command = "mdbook-lang"
server = "http://127.0.0.1:3333/api/v1/build-code"
cpp-enable = true
java-enable = true
go-enable = true
python-enable = true
javascript-enable = true
typescript-enable = true
scheme-enable = true
editable = true
disable-devtool-auto = false
disable-menu = false
clear-log = false
disable-select = false
disable-copy = false
disable-cut = false
disable-paste = false
ace-strict = false

[output]

[output.html]
additional-js = ["jquery.js", "disable-devtool.js", "lang.js"]
additional-css = ["lang.css"]
```

- `server`: 编译服务器的 `url`。

现已部署了一个编译服务器，你可以直接修改 `server` 的值进行测试。

```toml
server = "https://183.205.132.14:3000/playground/api/v1/build-code"
```

**注意**
你需要先在浏览器中打开 [https://183.205.132.14:3000](https://183.205.132.14:3000) 并忽略或关闭安全警告，否则在电子书中运行编程语言代码时无法访问该服务。

- `language-enable`: `启用`某种语言，默认值为 `true`。

- `disable-devtool-auto`: 禁用浏览器调试器功能，默认值为 `false`，即不禁用。
- `ace-strict`: 启用 `ACE`编辑器的严格模式，默认值为 `false`。启用后，`ACE` 编辑器将不允许电子书读者在玩转代码时在`ACE`编辑环境中剪切、复制和粘贴代码等功能。

## 运行编译服务器
### 仅 Windows 7/8/10/11 需要安装/卸载服务
所有命令都以 `管理员` 身份运行。

- 安装 `mdbook-lang` 编译、运行服务器软件为Windows操作系统中的一个服务：
```bash
C:\Windows\System32>mdbook-lang server install --hostname 127.0.0.1 --port 3333
```

或者当`install`无参数时候，采用默认参数： `hostname`为`127.0.0.1`、 `port`为`3333`：
```bash
C:\Windows\System32>mdbook-lang server install
```

当不再需要 `mdbook-lang` 服务或想更改 `hostname` 和/或 `port` 时，应先 以子`server`的子命令命令`uninstall`卸载服务，再以不同参数通过`install`重新安装服务 ：

永久删除服务：
```bash
mdbook-lang uninstall
```

如果你想更改 `hostname` 和/或 `port`：

```bash
mdbook-lang server uninstall
mdbook-lang server install --hostname 0.0.0.0
```

### 类 Unix 和 Windows 7/8/10/11操作系统

#### 启动 mdbook-lang 编译服务器


用以下命令启动编译服务器：
```bash
mdbook-lang server start
```

或者在类 Unix 系统中使用 `--hostname` 或简写 `-n` 和 `--port` 或简写 `-p` 参数启动编译服务器：
```bash
mdbook-lang server start --hostname 127.0.0.1 -port 3333
```
对于 Windows，需要具有管理员权限。
- 启动服务
```bash
C:\Windows\System32>mdbook-lang server start
```
或者通过操作系统提供的 `服务管理器` 或 `任务管理器` 图形界面工具启动。

#### `stop/restart/status` 子命令
- 停止服务器
```bash
mdbook-lang server stop
```
- 重启服务器
```bash
mdbook-lang server restart
```
- 检查服务器状态
```bash
mdbook-lang server status
```

## 运行 mdbook，并打开默认浏览器，阅读电子书，playing内嵌代码段：

```bash
mdbook serve -o
```

## 选项
在`mdbook-lang`所支持的语言的markdown代码块中，可使用扩展的选项，完成定制功能：
### norun

`norun` 选项会使代码块不被`mdbook-lang`预处理器渲染。如果你想展示一些不应被执行的代码示例，并且 `language-enable=true` 时，可以使用该选项。

<pre>
<code class="language-markdown">
&#96;&#96;&#96;java,norun
// java codeblock with norun option
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
&#96;&#96;&#96;
</code>
</pre>

这样该代码块不会被预处理器渲染：

```java,norun
// java codeblock with norun option
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
```

<pre><code class="language-markdown">
&#96;&#96;&#96;java
// java codeblock without norun option
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
&#96;&#96;&#96;
</code>
</pre>

`java-enable=true`，且无`nolang`选项时该代码块会被预处理器渲染：

```java
// java codeblock with norun option
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
```

## 快捷键

|操作系统| 快捷键 | 说明 |
| --- | --- | --- |
|Windows| Ctrl-Enter | 运行代码 |
|Mac| Command-Enter | 运行代码 |
|Windows| Ctrl-Shift-Enter | 清除代码编译或运行结果 |
|Mac| Command-Shift-Enter | 清除代码编译或运行结果 |

## 语言扩展

该预处理器只识别特定语言的特定扩展。例如，C++ 代码只能用 c++ 或 cpp 代码块，Python 代码只能用 python 或 py 代码块。

完整扩展名列表如下：

| 语言 | 扩展名 | 编译器 |
| --- | --- | --- |
| C++ | cpp, c++, c| clang++|
| Java | java|sun jdk/openjdk|
| Go | go|golang|
| Python | py, python|python2, python3（`python` 目录需在 `PATH` 环境变量中）|
| JavaScript | js, javascript|node.js|
| TypeScript | ts, typescript|node.js, tsc|
| Scheme | lisp, scheme|gambit-scheme（`gsi` 目录需在 `PATH` 环境变量中）|

如需支持某些语言，你需要在编译服务器主机上安装相应的编译器。

## 性能

通过浏览器远程执行代码块的速度非常快，仅受网络延迟和编译服务器的负载影响。
