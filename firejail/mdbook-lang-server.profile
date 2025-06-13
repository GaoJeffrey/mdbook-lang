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

# ipc-namespace
# machine-id

net none
# netfilter
# no3d
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
