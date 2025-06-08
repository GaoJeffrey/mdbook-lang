# Usage

THis is designed as a preprocessor plugin for mdbook and a compiler server running standalone. 

## Installation

There are two ways to install mdbook-lang:

You can install it with cargo if you have rust environment:

```bash
cargo install mdbook-lang
```

Or you can download the binary from [github release page](https://github.com/GaoJeffrey/mdbook-lang). You should put the binary in your system's PATH.

You can cheeck the instalation with:
```bash
mdbook-lang --version
```
## Configuration

After installation, you can configure mdbook-lang in your book's `book.toml` file through `mdbook-lang install` to config the plugin usefull for your mdbook.

```bash
$ mdbook-lang install /path/to/your/book
```

This will set the lang preprocessor plugins and the url of compiler server parameters.

For example:

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

- server: The url of compiler server.

I deployed a compiler server, you can modify the value of `server` use it for test directly.

```toml
server = "http://183.205.132.14:3333/api/v1/build-code"
```

- language.enable: Enable the language for the repl, default value is true.

- disable-devtool-auto : Disable the browser debugger automatically, default value is false.
- ace-strict: Enable the strict mode of ace editor, default value is false. When enabled, the editor will not allow the user to cut, copy, and paste code in ACE editor.


## Run the compiler server
You can run the compiler server with:
```bash
mdbook-lang server start
```

Or you can run the compiler server with `--hostname` or short `-n` and `--port` or short `-p`arguments:
```bash
mdbook-lang server start --hostname 127.0.0.1 -port 3333
```

## Run the mdbook
```bash
mdbook serve
```

## Options

### norun

norun option will make the codeblock not rendered by the preprocessor. You can use this option if you want to show some code examples that should not be executed, and the `language` is enabled by `language.enable=true`.

```markdown
    ``` java,norun
    // java codeblock with norun option
    public class HelloWorld {
        public static void main(String[] args) {
            System.out.println("Hello, world!");
        }
    }
    ```
```


And it will not be rendered by this preprocessor:

```java,norun
// java codeblock with norun option
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
```

```markdown

    ``` java
    // java codeblock without norun option
    public class HelloWorld {
        public static void main(String[] args) {
            System.out.println("Hello, world!");
        }
    }
    ```

```

And it will be rendered by this preprocessor:

```java
// java codeblock with norun option
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, world!");
    }
}
```

## Shortcuts

|OS| Shortcut | Description |
| --- | --- | --- |
|Windows| Ctrl + Enter | Run the code |
|Mac| Command + Enter | Run the code |
|Windows| Ctrl + L | Run the code and show the output |
|Mac| Command + L | Run the code and show the output |

## Language Extensions

This preprocessor only recongnizes specific extensions for sepecific language. For example, you can only use use c++ or cpp codeblock for cpp code, use python or py codeblock for python code.

Here is the full list of extensions:

| Language | Extension | compiler |
| --- | --- | --- |
| C++ | cpp, c++, c| clang++|
| Java | java|sun jdk/openjdk|
| Go | go|golang|
| Python | py, python|python2,python3|
| JavaScript | js, javascript|node.js|
| TypeScript | ts, typescript|node.js,tsc|
| Scheme | lisp, scheme|gambit-scheme|

Into to support langguage playground, you should install the compiler for the language on compiler server host.


## Performance

There is no doubt that the execution of the code is really fast compared with backend server playgrounds. However, the bottleneck is the compiling and loading time of the codeblock, which is depending on each language.