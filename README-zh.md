# mdBook Lang

[English](./README.md)

[中文文档](https://gaojeffrey.github.io/mdbook-lang/en)

[英文文档](https://gaojeffrey.github.io/mdbook-lang/en)

___
一个受 [Rust playground](https://rust-lang.github.io/mdBook/format/mdbook.html#rust-playground) 启发的多编程语言`mdbook` 预处理器和编译服务器。

`0.1.0` 版本在类 `Unix` 系统（如 `Linux/MacOS/FreeBSD` 等）内置支持 `c/c++`、`go`、`python`、`java`、`javascript`、`typescript`、`scheme` 等语言，`windows` 从 0.1.1 版本开始支持。

## 平台支持
| 版本 | 操作系统 | 架构 |
| ------- | ------- | ------- |
| 0.1.x | Linux | x86, x86_64, arm|
| 0.1.x | MacOS| x86_64, arm |
| 0.1.0 | Windows | 不支持 |
| 0.1.1 | Windows | x86, x86_64, arm |

- v0.1.1 已上线。

---

## 简单用法
**请确保已安装 Rust 开发环境，然后：**
```bash
$ cargo install mdbook mdbook-lang
$ cd your/mdbook/directory
$ mdbook-lang install
$ mdbook-lang server start
$ mdbook serve -o
```

- 如果是windows操作系统，则执行前需要先安装服务：
```bash
$ mdbook-lang server install --host 127.0.0.1 --port 3333
```

## 预安装编译器
如需在 playground 中使用相应语言，请安装对应编译器：

- C/C++：clang++
- go：golang
- python：python2/python3（`python` 可执行文件需在 `PATH` 环境变量中）
- java：sun jdk/openjdk
- javascript、typescript：node.js
- typescript：tsc
- scheme/lisp：gambit-scheme（`gsi` 可执行文件需在 `PATH` 环境变量中）

---

### 类 Unix 系统
如果通过 `yum`、`apt` 或 `brew` 等命令安装在 `/usr/local/bin` 或其他目录，无需配置 `PATH` 环境变量。

### Windows
在 Windows 系统上，需安装编译器并配置 `系统` 的 `PATH` 环境变量（`mdbook-lang.exe` 作为系统服务安装，以其他系统用户身份运行）。以 C/C++ 为例：
- 安装编译器
    - 自行下载并安装 [mingw-w64](https://github.com/mstorsjo/llvm-mingw/releases)
        - 解压 mingw
        - 找到包含 `clang++.exe` 的目录
        - 将该目录添加到 `系统` 的 `PATH` 环境变量
    - 或安装 [chocolatey](https://chocolatey.org/install),参考[chocolatey mingw](https://community.chocolatey.org/packages?q=mingw)，用命令`choco install mingw`安装llvm编译器
        - 找到 `chocolatey` 的 bin 目录
        - 将该目录添加到 `系统` 的 `PATH` 环境变量
            - 如 `C:\ProgramData\chocolatey\bin`
        - 检查 `系统` 的 `PATH` 环境变量
- [`系统环境变量` 设置方法](https://www.wikihow.com/Change-the-PATH-Environment-Variable-on-Windows)

---

# 综合用法

## `mdbook-lang install` 会修改 `book.toml`
**在 `book.toml` 所在目录安装 `mdbook-lang`**

```bash
$ mdbook-lang install
```

安装后，会新增或修改 `[output.html]` 和 `[preprocessor.lang]` 两个部分：

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

`server = "http://127.0.0.1:3333/api/v1/build-code"` 用于部署本地/远程 `playground`服务器运行代码块。

`cpp-enable = true`：mdbook-lang 支持 C/C++，`cpp-enable = false`：不支持。

`disable-devtool-auto` 用于 `disable-devtool.js` 工具自动启用/禁用。

`editable = true` 启用 `ACE` 编辑器代码块编辑，`editable = false` 禁用。

`ace-strict = true` 禁用 `ACE` 编辑器中的复制/剪切/粘贴。

以上值均为默认值。

## 启动 mdbook 并开始体验

在示例 `mdbook` 目录下安装 `mdbook-lang` 插件，启动 `mdbook` 和编程语言服务器：
```bash
# 安装 mdbook-lang 插件
$ mdbook-lang install
# 启动 mdbook
$ mdbook serve -n 127.0.0.1 -p 2000
# 启动 mdbook-lang 服务器，默认 127.0.0.1:3333
$ mdbook-lang server start
# 或指定主机和端口启动 mdbook-lang 服务器
$ mdbook-lang server start --hostname 127.0.0.1 --port 3333
```

## 注意事项

### 类 Unix 系统
#### 编译服务器以守护进程方式运行
##### 启动
启动编译服务器以支持多语言 `playground`，作为守护进程运行，`stdin/stdout/stderr` 和 `pid` 文件在 `/tmp/mdbook-lang-server.[pid/log/out]`

```shell
# 默认主机和端口：127.0.0.1:3333
$ mdbook-lang server start
# 监听 127.0.0.1:9876
$ mdbook-lang server start -n 127.0.0.1 -p 9876
```

- 注意，windows系统主编译服务ip地址和端口号，在安装服务时指定

```bash
# windows
C:\Windows\System32> mdbook-lang server install --hostname 127.0.0.1 --port 3333
```

##### 停止
停止编译服务器
```shell
$ mdbook-lang server stop
```
##### 重启
使用上次启动命令的配置重启编译服务器。
```shell
$ mdbook-lang server restart
```
##### 状态
显示编译服务器状态
```shell
$ mdbook-lang server status
```

### Windows
mdbook-lang playground 服务器作为 Windows 服务安装，可用以下命令启动/停止/重启服务。

- 以管理员身份打开命令提示符，运行：

```bash
C:\Windows\system32> mdbook-lang server install --hostname 127.0.0.1 --port 3333
```

服务安装后，可用 `系统服务管理器` 或命令行启动/停止/重启服务（需管理员权限）：
```bash
C:\Windows\system32> mdbook-lang server start # start 子命令无参数
C:\Windows\system32> mdbook-lang server stop
C:\Windows\system32> mdbook-lang server restart
```

- 随时可通过以下命令卸载 `mdbook-lang` 服务：
```bash
C:\Windows\system32> mdbook-lang uninstall
```

## 全局部署并对外服务

需要有 `ipv4/ipv6` 地址的主机，注意安全性。

### 安全性：编译服务器支持 firejail 沙箱
- 设置两个环境变量启用 firejail
```bash
export MDBOOKLANG_SERVER_SANDBOX_CMD="firejail"
export MDBOOKLANG_SERVER_SANDBOX_ARGS="--profile=/etc/firejail/mdbook-lang-server.profile:--quiet"
```
- firejail 配置
```bash
# /etc/firejail/mdbook-lang-server.profile
include disable-common.inc
# include disable-exec.inc
# noexec ${HOME}
noexec ${RUNUSER}
noexec /dev/shm
noexec /var

# 必须取消注释以允许 c/c++ 在 /tmp 执行 output.exe
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
# 调试用
# private-bin ls

# 允许 c/c++ 工具链
private-bin mdbook-lang
private-bin clang++
private-bin ld

# 允许 java 工具链
private-bin java
private-bin javac

# 允许 python 工具链
private-bin python2
private-bin python3

# 允许 go 工具链
private-bin go

# 允许 javascript/typescript 工具链
private-bin node
private-bin tsc

# 允许 lisp/scheme 工具链
private-bin scheme-r5rs

# 允许 c/c++ 输出可执行文件
private-bin output.exe

# 必须为 java/javac 取消注释
# private-lib

blacklist /opt
blacklist /etc/nginx
blacklist /etc/firejail
blacklist /etc

blacklist /sbin
# java、c/c++ 检查
blacklist /bin
# python 解释器
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

### 通过nginx部署多本mdbook电子书

```conf
# /etc/nginx/conf.d/mdbook.conf
map $http_upgrade $connection_upgrade {
        default upgrade;
        '' close;
}

server {
        listen 3000;
        server_name 0.0.0.0;

        # 编译服务器
        location /playground/{
            proxy_pass http://127.0.0.1:3333/;
        }
        # java 面向对象编程 mdbook
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

### nginx 反向代理下 mdbook 刷新页面变化

#### 需手动修改 `index.hbs`

请注意 index.hb 中的 `/joop/` 字符串，WebSocket 需启用 nginx 反向代理以支持多 mdbook 部署时,`*.md`文档更新而重新生成静态`html`文件时，通知浏览器刷新。其中`joop`是通过nginx部署的电子书对应的`route`路径，参考nginx配置。

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

