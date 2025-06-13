# Advanced Usage

## nginx reverse proxy to support multiple books

You can deploy multiple books on the same host with only one compiler server and nginx to support mmultiple programming languages.


### install nginx

#### ubuntu

```bash
sudo apt-get install nginx
```

#### centos
```bash
sudo yum install nginx
```

#### macos
```bash
brew install nginx
```
#### windows
```bash
choco install nginx
```
### configure nginx
```bash
sudo vim /etc/nginx/conf.d/mdbook-lang.conf
```
and add the following content:
```bash
# /etc/nginx/conf.d/mdbook.conf
map $http_upgrade $connection_upgrade {
    default upgrade;
    '' close;
}

server {
    # nginx listen port
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

    # rust-course mdbook
    location /rust-course/{
        proxy_pass http://127.0.0.1:2001/;
    }
    location /rust-course/__livereload{
        proxy_pass http://127.0.0.1:2001/__livereload/;
    }
    proxy_http_version 1.1;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection 'upgrade';
    proxy_set_header Host $host;
    proxy_cache_bypass $http_upgrade;
}
```

### start nginx
```bash
sudo nginx -s reload
```
### start compiler server
```bash
mdbook-lang server --hostname 127.0.0.1 --port 3333
```

### start mdbook
```bash
$ cd /path/to/joop/
$ mdbook serve --hostname 127.0.0.1  --port 2000 > joop-mdbook.log 2> &1 &
$ cd /path/to/rust-course/
mdbook serve --hostname 127.0.0.1 --port 2001 > rust-course.log > 2&1 &
```

### modify book.toml's server option

```toml
server = "http://183.205.132.14:3000/playground/api/v1/build-code"
```

### access mdbook
- 通过浏览器访问joop:[http://127.0.0.1:3000/joop](http://127.0.0.1:3000/joop)
- 通过浏览器访问rust-course:[http://127.0.0.1:3000/rust-course](http://127.0.0.1:3000/rust-course)


## suport sandbox for security

### install firejail

#### ubuntu
```bash
sudo apt-get install firejail
```
#### centos
```bash
sudo yum install firejail
```
#### macos
```bash
brew install firejail
```


### configure firejail
```bash
sudo vim /etc/firejail/mdbook-lang-server.profile
```

and add the following content:
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
private-bin python

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
### configure envs
```bash
sudo vim ~/.bashrc
```
and add the following content:

```bash
export MDBOOKLANG_SERVER_SANDBOX_CMD="firejail"
export MDBOOKLANG_SERVER_SANDBOX_ARGS="--profile=/etc/firejail/mdbook-lang-server.profile:--quiet"
```

> Note: sandbox is for single or multiple books, local or remote server is ok.



Enjoy it!