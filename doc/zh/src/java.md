# Java


Java 是一种多范式编程语言，支持面向对象、命令式和函数式等编程风格。它是一种编译型语言，其源代码会被编译为字节码，可以在任何 Java 虚拟机（JVM）上运行，而不受底层计算机架构的影响。

Java 被广泛用于开发各种应用，包括桌面应用、Web 应用、移动应用和企业级应用。它也常用于开发大规模分布式系统（如互联网应用）以及实时系统（如视频游戏）。

编译器/运行服务器使用 `javac[.exe]` 编译器编译`java`源代码代码为字节码，通过 `java[.exe]` 解释器解释运行字节码。编译器/运行服务器会查找 `public class` 以确定不要保存的正确文件名，并查找 `main class` 和 `main` 方法来运行程序。


```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
```


你不能在同一个文件中编写多个 `public class`，否则编译器会报错。例如，下面的代码会导致错误：


```java
public class HelloWorld {
    public static void main(String[] args) {
        new Fibonacci().run(10);
    }
}

public class Fibonacci {
    public void run(int n) {
        int n = 10;
        int a = 0, b = 1;
        for (int i = 0; i < n; i++) {
            System.out.println(a);
            int temp = a;
            a = b;
            b = temp + b;
        }
    }
}
```


```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}

public class Sum {
     public static void main(String[] args) {
        int sum = 0;
        for (int i = 0; i <= 100; i++) {
            sum += i;
        }
        System.out.println("Sum of first 100 numbers is: " + sum);
    }
}
```



另外，编译器或运行服务器会查找包含 `main` 方法的主类（`main class`）来运行程序。如果存在多个主类，编译器或运行服务器通常会按照类名的字母表顺序选择第一个找到的主类来执行。



```java
class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}

class Sum {
     public static void main(String[] args) {
        int sum = 0;
        for (int i = 0; i <= 100; i++) {
            sum += i;
        }
        System.out.println("Sum of first 100 numbers is: " + sum);
    }
}
```


The output of the above code will be:
```shell
Hello, World!
```

此处的`main`方法，实际函数头部声明应为：`public static void main(String[] args)`。包含了主方法的类称为是主类。如下几个代码不包含主方法、所在类也不是主类

```java
class HelloWorld {
    public void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
```

```java
class Sum {
    public static int main(String[] args) {
        int n = 100;
        int sum = 0;
        for (int i = 0; i <= n; i++) {
            sum += i;
        }
        System.out.println("Sum of first " + n + " numbers is: " + sum);
    }
}
```

```java
class Fib {
    public static void main(String args) {
        int fib[] = new int[30];
        fib[0] = fib[1] = 1;

        for (int i = 2; i < 30; i++) {
            fib[i] = fib[i - 1] + fib[i - 2];
        }

        for (int i = 0; i < 30; i++) {
            System.out.println(fib[i]);
        }

    }
}
```


```java
class Mul {
    public static long main() {
        long n = 30;
        long  m = 1;
        for (int i = 1; i <= n; i++) {
            m *= i;
        }
        return m;
    }
}
```


