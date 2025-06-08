# mdbook-lang 

An mdBook real-time playground for programming languages, executed in the browser with a compiler server deployed by yourself. You can also extend it with some more programming languages.

This is mostly iinspired by mdbook rust and mdbook-repl, but it's only limited to rust and some interpreted programming languages through [https://play.rust-lang.org](https://play.rust-lang.org) of iframe embeded wasm to support such as `python`,`javascript` and `typescript`. Then we can extend it with more programming languages through controled compiler server.

```cpp
#include <iostream>
usinag namespace std;
int main() {
    cout << "Hello, World!" << endl;
    return 0;
}
```
All code are editable, resettable, and runnable. You can modify and execute the code directly in the browser to see the output instantly. If you are a teacher assigning exercises to students, you can control cut, copy, and paste actions, as well as disable the browser debugger for a more controlled environment.

Development is ongoing to support many more programming languages in the future.
