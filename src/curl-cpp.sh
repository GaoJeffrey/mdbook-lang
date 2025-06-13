#!/bin/bash

#c plus plus language
curl -X POST http://127.0.0.1:3333/api/v1/build-code\
    -H "Content-Type: application/json" \
    -d '{"lang":"cpp","code_block":"#include<iostream>\n\nusing namespace std;\n\nint main(int argc, char** argv){\n\n  int i = 1;\n\n  i++;\n\n  cout << \"i = \" <<i << endl;\n\n}"}'

