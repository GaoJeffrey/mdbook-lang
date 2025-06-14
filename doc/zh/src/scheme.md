# Scheme
Scheme 是一种易于学习和使用的编程语言。它属于 Lisp 语言家族，是 Lisp 的一种方言。Scheme 是一门函数式编程语言，这意味着它通过函数来进行计算。


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