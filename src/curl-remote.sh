#!/bin/bash
# change 127.0.0.1 to global ip
#c plus plus language
curl -X POST http://127.0.0.1:3333/api/v1/build-code\
    -H "Content-Type: application/json" \
    -d '{"lang":"cpp","code_block":"#include<iostream>\n\nusing namespace std;\n\nint main(int argc, char** argv){\n\n  int i = 1;\n\n  i++;\n\n  cout << \"i = \" <<i << endl;\n\n}"}'

#c  language
curl -X POST http://127.0.0.1:3333/api/v1/build-code\
    -H "Content-Type: application/json" \
    -d '{"lang":"c","code_block":"#include<stdio.h>\n\nint main(int argc, char** argv){\n\n  int i = 1;\n\n  i++;\n\n  printf(\"i = %d\\n\", i);\n\n}"}'

#java language
curl -X POST http://127.0.0.1:3333/api/v1/build-code\
    -H "Content-Type: application/json" \
    -d '{"lang":"java","code_block":"public class Hello{\n\n    public static void main(String args[]){\n\n     int i = 2;\n\n      System.out.println(\"i = \" + i);\n     }\n\n}"}'

#python language
curl -X POST http://127.0.0.1:3333/api/v1/build-code\
    -H "Content-Type: application/json" \
    -d '{"lang":"python","code_block":"print(\"Hello Python\");"}'

#javascript language
curl -X POST http://127.0.0.1:3333/api/v1/build-code\
    -H "Content-Type: application/json" \
    -d '{"lang":"javascript","code_block":"console.log(\"Hello JavaScript\");"}'

#typescript language
curl -X POST http://127.0.0.1:3333/api/v1/build-code\
    -H "Content-Type: application/json" \
    -d '{"lang":"typescript","code_block":"console.log(\"Hello TypeScript\");"}'

