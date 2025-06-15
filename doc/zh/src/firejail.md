# 沙箱配置

## 添加白名单

如果所部署的python编程环境不在系统路径，而是在`${HOME}`目录，除了将其添加到`$HOME/.bash`的`PATH`环境变量之外，需要使用命令安装相应包：

- 采用系统`python`环境，用非超管账户安装包，则额外包安装路径为：`${HOME}/.local/lib/python3.10/site-packages`

```bash
$ python -m pip install numpy
```

则需要在firejail配置尾部添加语句，注意`python`版本号：
```bash
whitelist ${HOME}/.local/lib/python3.10/site-packages
```

## 添加编译器白名单

如果安装了其他编译器，需要在firejail配置文件中添加：
```bash
private-bin your-compiler-file-name
```