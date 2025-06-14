# mdBook Lang

[中文](./README-zh.md)

[中文文档](https://gaojeffrey.github.io/mdbook-lang/zh)
[英文文档](https://gaojeffrey.github.io/mdbook-lang/)

___
A playground mdbook preprocessor and compiler server for multiple programming languages inspired by [Rust rlayground](https://rust-lang.github.io/mdBook/format/mdbook.html#rust-playground), which supports only Rust programming language.

Version 0.1.0 supports c/c++, go, python, java, javascript, typescript, scheme in build-in manner on Unix like os, such as Linux/MacOS/FreeBSD etc., while windows is supported from version 0.1.1.

## platform support 
| Version | OS | Arch |
| ------- | ------- | ------- |
| 0.1.x | Linux | x86, x86_64, arm|
| 0.1.x | MacOS| x86, x86_64, arm |
| 0.1.0 | Windows | no |
| 0.1.1 | Windows | x86, x86_64, arm |

- v0.1.1 is online now.

---

## Simple usage
**Make sure that you have the rust development environment installed, and then:**
```bash
$ cargo install mdbook mdbook-lang
$ cd your/mdbook/directory
$ mdbook-lang install
$ mdbook-lang server start
$ mdbook serve -o
```


## Pre-installed Compiler
You should install the corresponding compiler(s) to use the playground for your select programming language(s):

- clang++ for C/C++
- golang for go
- python2/python3 for python(`python` binary should be in the `PATH` env.)
- sun jdk/openjdk for java
- node.js for javascript and typescript
- tsc for typescript
- gambit-scheme for scheme/lisp(`gsi` binary should be in the `PATH` env.)

---

### Unix like OS
Needn't config the `PATH` environment if you install it in `/usr/local/bin` or other directory through `yum`, `apt` or `brew` etc. commands.

### Windows
On Windows OS, install the compiler and config the `PATH` environment of `System` not `Users`（`mdbook-lang.exe` is installed as a `system service`, and running as another `system user`. Taking C/C++ for example:
- install compiler
  - download and install [mingw-w64](https://github.com/mstorsjo/llvm-mingw/releases) for C/C++ by your self
    - extract the mingw archive
    - find the directory contain `clang++.exe`
    - add the directory to `PATH` environment of `System` not `Users`
  - or install [chocolatey](https://chocolatey.org/install) and use [chocolatey mingw](https://community.chocolatey.org/packages?q=mingw), `choco install mingw`
    - find the `chocolatey` bin directory
    - add the directory to `PATH` environment of `System` not `Users`
      - like `C:\ProgramData\chocolatey\bin`
    - check the `PATH` environment of `System`
- [`system environment` setting](https://www.wikihow.com/Change-the-PATH-Environment-Variable-on-Windows)
---
# Comprehensive usage

## `mdbook-lang install` changes `book.toml`
**At the directory where `book.toml` exists, install the mdbook-lang support**
```bash
$ mdbook-lang install
```

After installation, the are two sections `[output.html]` and `[preprocessor.lang]` added or modifed as:

```bash
[output.html]
additional-css = ["lang.css"]
additional-js = ["disable-devtool.js", "lang.js", "jquery.js"]
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
ace-strict = true
```

`server = "http://127.0.0.1:3333/api/v1/build-code"` is used for local/remote playground server to run code block.

`cpp-enable = true`: C/C++ is supported by mdbook-lang, or `cpp-enable = false`: C/C++ is not supported by mdbook-lang.

`disable-devtool-auto` is for `disable-devtool.js` tools to disable or enable the it auto.

`editable = true` to enable edit the code block in `ACE editor` `editable = false` to disable.

`ace-strict = true` disable  copy/cut/paste in `ACE editor`

And all the value are default.

## start the mdbook and begin playing


Install mdbook-lang plugin, start the mdbook and programming language server in the directory of example mdbook:
```bash
# install mdbook-lang plugin
$ mdbook-lang install
# start mdbook
$ mdbook serve -n 127.0.0.1 -p 2000
# start mdbook-lang server use default configure as 127.0.0.1:3333
$ mdbook-lang server start
# or start mdbook-lang server use given hostname and port
$ mdbook-lang server start --hostname 127.0.0.1 --port 3333
```

## Notice

### Unix like OS
#### compiling server is daemonized
##### start
Start the compiling server to enable playground for multiple programming languages, and running as a daemon at the stdin/stdout/stderr and pid files as /tmp/mdbook-lang-server.[pid/err/out]

```shell
# start for default hostname and port: 127.0.0.1:3333
$mdbook-lang server start
# start for listening 127.0.0.1:9876
$mdbook-lang server start -n 127.0.0.1 -p 9876
```
##### stop
Stop the compiler server
```shell
$ mdbook-lang server stop
```
##### restart
restart the compiler server use the configure as the same as the last start command.
```shell
$ mdbook-lang server restart
```
##### status
show the status of the compiler server
```shell
$ mdbook-lang server status
```

### Windows
Tht mdbook-lang playground server is install as a windows service, you can use the following command to start/stop/restart the service.

- Open the command prompt as administrator, and run the following command:

```bash
C:\Windows\system32> mdbook-lang server install --hostname 127.0.0.1 --port 3333
```

Then the service is installed as a windows service, you can use the `System Service Manager` provided by `Microsoft` or through the command line to start/stop/restart the service as in Unix like OS do(needing administrator privilege):
```bash
C:\Windows\system32> mdbook-lang server start # there are no arguments for start sub-command
C:\Windows\system32> mdbook-lang server stop
C:\Windows\system32> mdbook-lang server restart
```

- Anytime, you can delete the `mdbook-lang` service through:
```bash
C:\Windows\system32> mdbook-lang uninstall
```

## Deploy globally and serve to all the world

You need a host with `ipv4/ipv6` address, and the security is important.

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

```Handlebars
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

- nginx -s  stop/quit/reopen/reload
