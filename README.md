# mdBook Lang
___

A playground mdbook preprocessor and compiling server for multiple programming languages inspired by [Rust rlayground](https://rust-lang.github.io/mdBook/format/mdbook.html#rust-playground) supporting only Rust programming.

Current v0.1.0 supports c/c++, go, python, java, javascript, typescript, scheme in build-in manner.

Bellow is an example of a c/c++ code block that can be executed on local or remote server deployed by yourself, this is the same idea as the Rust Playground does.

___

## compiler

clang++ for C/C++

golang for go

python2 for python

jdk for java

nodejs for javascript and typescript
tsc for typescript


gambit for scheme/lisp


___ 
```cpp
// C/C++ code block
#include <iostream>
using namespace std;

int main(int argc, char** argv){
    cout << "Hello C Plus Plus  World! << endl;
    return 0;
}
```
___

## for the book.toml
### at the directory where `book.toml` exists, install the mdbook-lang support
```bash
$mdbook-lang install
```

After installation, the are two sections `[output.html]` and `[preprocessor.lang]` added or modifed as:

```bash
[output.html]
additional-css = ["lang.css"]
additional-js = ["disable-devtool.js", "lang.js", "jquery.js"]
[preprocessor.lang]
command = "mdbook-lang"
server = "http://127.0.0.1/api/v1/build-code"
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
ace-strict = true
```

`server = "http://127.0.0.1/api/v1/build-code"` is used for local/remote playground server to run code block.

`cpp-enable = true` is supported by mdbook-lang, or `cpp-enable = false` is not supported by mdbook-lang.

`disable-devtool-auto` etc. is for `disable-devtool.js` tools to disable or enable the javascript.

`editable = true` to enable edit the code block in `ACE editor`.

`ace-strict = true` disable `ACE editor` copy/cut/paste

And all the value are default.

___

### code block support option
```cpp,editable,norun
#inlcude <iostream>
using namespace std;
int main(int argc, char** argv){
    cout << "Hello C/C++ World in mdbook-lang code block" << endl;
    return 0;
}
```

this C/C++ code can be edited and cann't be played.


### code with `nolang`
```cpp,nolang
#inlcude <iostream>
using namespace std;
int main(int argc, char** argv){
    cout << "Hello C/C++ World in mdbook-lang code block" << endl;
    return 0;
}
```
this C/C++ code will not processed by mdbook-lang for the `nolang` option.

___

## start the mdbook and begin playing


Install mdbook-lang plugin, start the mdbook and programming language server in the directory of joop mdbook:
```bash
$mdbook mdbook-lang install
$mdbook serve -n 127.0.0.1 -p 2000
$mdbook mdbook-lang server start -n 127.0.0.1 -p 3333
```

## Notice

### compiling server is daemonized
#### start
Start the compiling server to enable playground for multiple programming languages, and running as a daemon at the stdin/stdout/stderr and pid files as /tmp/mdbook-lang-server.[pid/err/out]

```shell
# start for default hostname and port: 127.0.0.1:3333
$mdbook-lang server start
# start for listening 127.0.0.1:9876
$mdbook-lang server start -n 127.0.0.1 -p 9876
```
#### stop
Stop 
```shell
$mdbook-lang server stop
```
#### restart
```shell
$mdbook-lang server restart
```
#### status
```shell
$mdbook-lang server status
```


## deploy on global server and provide online mdbook and mdbook-lang service

### For security: the compiling server support sandbox such as firejail
- set two envs to enable firejail
```bash
export MDBOOKLANG_SERVER_SANDBOX_CMD="firejail"
export MDBOOKLANG_SERVER_SANDBOX_ARGS="--profile=/etc/firejail/mdbook-lang-server.profile:--quiet"
```
- firejail configures
```bash
# /etc/firejail/mdbook-lang-server.profile
include disable-common.inc
# include disable-exec.inc
# noexec ${HOME}
noexec ${RUNUSER}
noexec /dev/shm
noexec /var

# must be canceled for c/c++ execute output.exe in /tmp
# noexec /tmp

include disable-passwdmgr.inc
include disable-programs.inc
quiet

net none

nodbus
nodvd
nogroups
nonewprivs
noroot
nosound
notv
nou2f
novideo
protocol inet,inet6
seccomp
shell none
# tracelog

disable-mnt
private
# for debug porpose
# private-bin ls

# allow c/c++ tools chain
private-bin mdbook-lang
private-bin clang++
private-bin ld

# allow java tools chain
private-bin java
private-bin javac

# allow python tools chain
private-bin python2
private-bin python3

# allow go tools chain
private-bin go

# allow javascript/typescript tools chain
private-bin node
private-bin tsc

# allow lisp/scheme tools chain
private-bin scheme-r5rs

# allow c/c++ output executable
private-bin output.exe

# must be canceled for java/javac
# private-lib

blacklist /opt
blacklist /etc/nginx
blacklist /etc/firejail
blacklist /etc

blacklist /sbin
# checked for java,c/c++,
blacklist /bin
# for python interpreter
# blacklist /usr/bin
blacklist /usr/sbin
blacklist /usr/libexec
blacklist /usr/local/sbin
blacklist /usr/local/lib
blacklist /usr/local/libexec
blacklist /usr/libexec/firejail
blacklist /usr/libexec/firejail/firejail-config
blacklist /usr/libexec/firejail/firejail-profile
blacklist /usr/libexec/firejail/firejail-shell
blacklist /usr/libexec/firejail/firejail-shell-wrapper
blacklist /usr/libexec/firejail/firejail
```
- firejail -s start/stop/reload

### for multiple mdbooks: nginx reverse proxy configure

```conf
# /etc/nginx/conf.d/mdbook.conf
map $http_upgrade $connection_upgrade {
    default upgrade;
    '' close;
}

server {
    listen 3000;
    server_name 0.0.0.0;

    # compiler server
    location /playground/{
    	proxy_pass http://127.0.0.1:3333/;
    }
    # java object oriented programming mdbook
    location /joop/{
        proxy_pass http://127.0.0.1:2000/;
    }
    location /joop/__livereload{
        proxy_pass http://127.0.0.1:2000/__livereload/;
    }

    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection 'upgrade';
    proxy_set_header Host $host;
    proxy_cache_bypass $http_upgrade;
}
```

### nginx reverse proxy for mdbook changes while refresh front web page

#### modify `index.hbs` by yourself

Please pay attention to the the string `/joop/` in index.hb for WebSocket to enable nginx reverse proxy if you need deploying multiple mdbooks.

```html
<!-- joop/theme/index.hbs -->
{{#if live_reload_endpoint}}
<!-- Livereload script (if served using the cli tool) -->                                                        
 <script>
    const wsAddress = wsProtocol + "//" + location.host + "/joop/" + "{{{live_reload_endpoint}}}";
    const socket = new WebSocket(wsAddress);
    socket.onmessage = function (event) {
        if (event.data === "reload") {
            socket.close();
            location.reload();
        }
    };

    window.onbeforeunload = function() {
        socket.close();
    }
</script>
{{/if}}
```