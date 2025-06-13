# Example
c++ code example


```c++,editable=false
#include <stdio.h>
#include <iostream>

int main(int argc, char** argv){
	printf("editable=false without norun\n");
	return 0;
}
```

```c++,norun
#include <stdio.h>
#include <iostream>

int main(int argc, char** argv){
	printf("without editable, with norun, but editable is enabled default\n");
	return 0;
}
```

```c++, editable,norun
#include <stdio.h>
#include <iostream>

int main(int argc, char** argv){
	printf("editable and norun\n");
	return 0;
}
```

```c++,norun
#include <stdio.h>
#include <iostream>

int main(int argc, char** argv){
	printf("with norun only\n");
	return 0;
}
```

```c++,nolang
#include <stdio.h>
#include <iostream>

int main(int argc, char** argv){
	printf("with nolang only\n");
	return 0;
}
```

javascript
```javascript
console.log("Hello JavaScript World!!!")
```

python
```python, editable=true
print("Hello Python Wrold")
```
java
```java, editable
public class Hello{

	public static void main(String[] args){
		System.out.println("Hello Java World!");
	}
}
```
go

```go, editable
package main

import "fmt"

func main() {
    fmt.Println("Hello, go 世界")
}

```

```lisp
;scheme,lisp
(define (greet-world)
  (let ((southern-germany "Grüß Gott!")
        (chinese "世界，你好")
        (english "World, hello"))
    (let ((regions (list southern-germany chinese english)))
      (for-each (lambda (region)
                  (display region)
                  (newline))
                regions))))
(greet-world)
```