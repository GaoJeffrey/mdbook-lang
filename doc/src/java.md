# Java


Java is a multi-paradigm programming language that supports object-oriented, imperative, and functional programming styles. It is a compiled language, which means that code written in Java is compiled into bytecode that can run on any Java virtual machine (JVM) regardless of the underlying computer architecture.

Java is used for developing a wide range of applications, including desktop applications, web applications, mobile applications, and enterprise applications. It is also used for developing large-scale distributed systems, such as the internet, and for developing real-time systems, such as video games.

The [ACE editor](https://ace.c9.io) for Java is a simple text-based interface that allows you to write and run Java code. The compiler/run server use the `javac` command to compile your code, and the `java` command to run it. The compiler/run server will find the `public class` to save proper file name, and find the `main class` and `main` method to run it.


```java
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }
}
```


You cannot write multiple `public class` in the same file, otherwise , the compiler will throw an error.

For example, the following codes will throw an errors:


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



Additionaly, the compiler/run server will find the `main class` to run it. If you have multiple `main class`, the compiler/run server will run the first `main class` found int the order of the `class` name.


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