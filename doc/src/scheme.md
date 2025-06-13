# Scheme

Scheme is a programming language that is designed to be easy to learn and use. It is a dialect of Lisp, which is a family of programming languages. Scheme is a functional programming language, which means that it uses functions to perform computations.


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