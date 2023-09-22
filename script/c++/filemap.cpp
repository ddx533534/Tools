#include<stdlib.h>
#include<stdio.h>
#Last login: Tue Sep 19 16:14:11 on ttys008
 ddx@dudongxu  ~  adb devices
List of devices attached
aca193e7d630	device

 ddx@dudongxu  ~  adb install /Users/ddx/Desktop/new_workspace/pay-demo/samples/build/outputs/apk/demo/debug/samples-demo-debug.apk
Performing Push Install
/Users/ddx/Desktop/new_workspace/pay-d...d. 5.3 MB/s (46323317 bytes in 8.342s)
	pkg: /data/local/tmp/samples-demo-debug.apk
Success
 ddx@dudongxu  ~  adb install /Users/ddx/Desktop/new_workspace/pay-demo/samples/build/outputs/apk/demo/debug/samples-demo-debug.apk
^C
 ✘ ddx@dudongxu  ~  adb devices
List of devices attached
^C
 ✘ ddx@dudongxu  ~  adb devices
adb: failed to check server version: protocol fault (couldn't read status): Connection reset by peer
 ✘ ddx@dudongxu  ~  adb devices
List of devices attached
aca193e7d630	device

 ddx@dudongxu  ~  adb devices
List of devices attached
aca193e7d630	device

 ddx@dudongxu  ~  adb install /Users/ddx/Desktop/new_workspace/pay-demo/samples/build/outputs/apk/demo/debug/samples-demo-debug.apk
Performing Push Install
/Users/ddx/Desktop/new_workspace/pay-d...d. 4.5 MB/s (46324025 bytes in 9.923s)
	pkg: /data/local/tmp/samples-demo-debug.apk
Failure [INSTALL_FAILED_ALREADY_EXISTS]
 ddx@dudongxu  ~  adb install /Users/ddx/Desktop/new_workspace/pay-demo/samples/build/outputs/apk/demo/debug/samples-demo-debug.apk
Performing Push Install
/Users/ddx/Desktop/new_workspace/pay-d...d. 5.2 MB/s (46324025 bytes in 8.439s)
	pkg: /data/local/tmp/samples-demo-debug.apk
Success
 ddx@dudongxu  ~  cd Desktop/obsidian
 ddx@dudongxu  ~/Desktop/obsidian   master  git pull
remote: Enumerating objects: 36, done.
remote: Counting objects: 100% (36/36), done.
remote: Compressing objects: 100% (13/13), done.
remote: Total 26 (delta 16), reused 22 (delta 13), pack-reused 0
Unpacking objects: 100% (26/26), done.
From github.com:ddx533534/obsidian
   700107f..2b78063  master     -> origin/master
Updating 700107f..2b78063
Fast-forward
 .obsidian/workspace.json                           | 115 +++++++++++----------
 .../Service/Binder \345\210\235\350\257\206.md"    |   2 +-
 {Computer System => ComputerSystem}/IPC.md         |   0
 .../Linux \346\216\242\347\264\242.md"             |  11 +-
 {Computer System => ComputerSystem}/fork.md        |   0
 {Computer System => ComputerSystem}/mkfifo.md      |   0
 {Computer System => ComputerSystem}/pipe.md        |   0
 README.md                                          |  23 +++--
 ...216\257\345\242\203\351\205\215\347\275\256.md" |   9 +-
 9 files changed, 91 insertions(+), 69 deletions(-)
 rename {Computer System => ComputerSystem}/IPC.md (100%)
 rename "Computer System/Linux \345\221\275\344\273\244\346\216\242\347\264\242.md" => "ComputerSystem/Linux \346\216\242\347\264\242.md" (91%)
 rename {Computer System => ComputerSystem}/fork.md (100%)
 rename {Computer System => ComputerSystem}/mkfifo.md (100%)
 rename {Computer System => ComputerSystem}/pipe.md (100%)
 ddx@dudongxu  ~/Desktop/obsidian   master  docker exec -it 682ca971b3cea089f0f3288b2e595544a589a7b97cedef167a080a70dd354457 /bin/bash
root@682ca971b3ce:/# ls
bin   etc   lib32   media  proc  sbin             sys  var
boot  home  lib64   mnt    root  sleep.info.text  tmp  work_space
dev   lib   libx32  opt    run   srv              usr
root@682ca971b3ce:/# cd work_space/
root@682ca971b3ce:/work_space# ls
Tools
root@682ca971b3ce:/work_space# cd Tools/
root@682ca971b3ce:/work_space/Tools# git pull]
git: 'pull]' is not a git command. See 'git --help'.

The most similar command is
	pull
root@682ca971b3ce:/work_space/Tools# git pull
remote: Enumerating objects: 24, done.
remote: Counting objects: 100% (24/24), done.

~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
"segv.c" [New]                                                0,0-1         All
#include<stdio.h>
remote: Compressing objects: 100% (12/12), done.
remote: Total 19 (delta 9), reused 16 (delta 6), pack-reused 0
Unpacking objects: 100% (19/19), 7.84 KiB | 501.00 KiB/s, done.
From github.com:ddx533534/Tools
   477e499..11894bc  main       -> origin/main
Updating 477e499..11894bc
Fast-forward
 README.md                            |  10 ++
 script/1core/1core-2process-nice.txt | 209 +++++++++++++++++++++++++++++++++++
 script/c++/output/shedule_nice       | Bin 0 -> 16776 bytes
 script/c++/shedule_nice.cpp          | 178 +++++++++++++++++++++++++++++
 4 files changed, 397 insertions(+)
 create mode 100644 script/1core/1core-2process-nice.txt
 create mode 100755 script/c++/output/shedule_nice
 create mode 100644 script/c++/shedule_nice.cpp
root@682ca971b3ce:/work_space/Tools# ls
README.md  script
root@682ca971b3ce:/work_space/Tools# cd script/
root@682ca971b3ce:/work_space/Tools/script# ls
1core  2core  c++  data  java  python  sh
root@682ca971b3ce:/work_space/Tools/script# cd c++/
root@682ca971b3ce:/work_space/Tools/script/c++# ls
fork.c  loop.cpp  output  shedule.cpp  shedule_nice.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# vi segv.c
root@682ca971b3ce:/work_space/Tools/script/c++# g++ segv.c -o segv
root@682ca971b3ce:/work_space/Tools/script/c++# ./segv
before invalid access
Segmentation fault
root@682ca971b3ce:/work_space/Tools/script/c++# git
usage: git [--version] [--help] [-C <path>] [-c <name>=<value>]
           [--exec-path[=<path>]] [--html-path] [--man-path] [--info-path]
           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare]
           [--git-dir=<path>] [--work-tree=<path>] [--namespace=<name>]
           [--super-prefix=<path>] [--config-env=<name>=<envvar>]
           <command> [<args>]

These are common Git commands used in various situations:

start a working area (see also: git help tutorial)
   clone     Clone a repository into a new directory
   init      Create an empty Git repository or reinitialize an existing one

work on the current change (see also: git help everyday)
   add       Add file contents to the index
   mv        Move or rename a file, a directory, or a symlink
   restore   Restore working tree files
   rm        Remove files from the working tree and from the index

examine the history and state (see also: git help revisions)
   bisect    Use binary search to find the commit that introduced a bug
   diff      Show changes between commits, commit and working tree, etc
   grep      Print lines matching a pattern
   log       Show commit logs
   show      Show various types of objects
   status    Show the working tree status

grow, mark and tweak your common history
   branch    List, create, or delete branches
   commit    Record changes to the repository
   merge     Join two or more development histories together
   rebase    Reapply commits on top of another base tip
   reset     Reset current HEAD to the specified state
   switch    Switch branches
   tag       Create, list, delete or verify a tag object signed with GPG

collaborate (see also: git help workflows)
   fetch     Download objects and refs from another repository
   pull      Fetch from and integrate with another repository or a local branch
   push      Update remote refs along with associated objects

'git help -a' and 'git help -g' list available subcommands and some
concept guides. See 'git help <command>' or 'git help <concept>'
to read about a specific subcommand or concept.
See 'git help git' for an overview of the system.
root@682ca971b3ce:/work_space/Tools/script/c++# git st
On branch main
Your branch is up to date with 'origin/main'.

Untracked files:
  (use "git add <file>..." to include in what will be committed)
	segv
	segv.c

#include<stdio.h>
#include<stdlib.h>
int main(){
  int *p = NULL;
  puts("before invalid access");
  *p = 0;
    puts("after invalid access");
    exit(EXIT_SUCCESS);
}
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
"segv.c" 9L, 169B                                             8,23          All
nothing added to commit but untracked files present (use "git add" to track)
root@682ca971b3ce:/work_space/Tools/script/c++# git add .
root@682ca971b3ce:/work_space/Tools/script/c++# mv segv output/segv
root@682ca971b3ce:/work_space/Tools/script/c++# git add .
root@682ca971b3ce:/work_space/Tools/script/c++# git cm -m "test segment fault"
[main 0a2fd8a] test segment fault
 2 files changed, 9 insertions(+)
 create mode 100755 script/c++/output/segv
 create mode 100644 script/c++/segv.c
root@682ca971b3ce:/work_space/Tools/script/c++# git push
Enumerating objects: 11, done.
Counting objects: 100% (11/11), done.
Delta compression using up to 6 threads
Compressing objects: 100% (7/7), done.
Writing objects: 100% (7/7), 2.82 KiB | 1.41 MiB/s, done.
Total 7 (delta 3), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (3/3), completed with 3 local objects.
To github.com:ddx533534/Tools.git
   11894bc..0a2fd8a  main -> main
root@682ca971b3ce:/work_space/Tools/script/c++# ls
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <err.h>

#define BUFFER_SIZE 1000
#define ALLOC_SIZE 100 * 1024 * 1024
static char command[BUFFER_SIZE];
int main()
{
    pid_t pid = getpid();
    // snprintf �~G��~U��~Z~D工�~\�~V��~O类似�~N printf �~G��~U��~L�~F�~~
X��~C�~M�~Z�~[��~N��~F�~S�~^~\�~S�~G��~H��| ~G�~G~F�~S�~G��~A�~L�~@~L�~~
X��~F�~S�~^~\�| ��~O�~L~V�~P~N�~X�~B��~H��~L~G�~Z�~Z~D�~W符�~U��~D中�~@@
~B
    snprintf(command, BUFFER_SIZE, "cat /proc/%d/maps", pid);
    puts("command has been generated");
    puts("memory map before memory locate");
    system(command);
    void *new_memory;
    new_memory = mmap()
}
~
"mmap.cpp" [noeol] 20L, 633B                                  1,1           All
fork.c  loop.cpp  output  segv.c  shedule.cpp  shedule_nice.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# ls
fork.c  loop.cpp  output  segv.c  shedule.cpp  shedule_nice.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# vim segv.c
root@682ca971b3ce:/work_space/Tools/script/c++# ls
fork.c  loop.cpp  output  segv.c  shedule.cpp  shedule_nice.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# git pull
remote: Enumerating objects: 11, done.
remote: Counting objects: 100% (11/11), done.
remote: Compressing objects: 100% (4/4), done.
remote: Total 7 (delta 3), reused 7 (delta 3), pack-reused 0
Unpacking objects: 100% (7/7), 1.90 KiB | 139.00 KiB/s, done.
From github.com:ddx533534/Tools
   0a2fd8a..33bc3a4  main       -> origin/main
Updating 0a2fd8a..33bc3a4
Fast-forward
 script/c++/mmap.cpp    |  20 ++++++++++++++++++++
 script/c++/output/mmap | Bin 0 -> 49504 bytes
 2 files changed, 20 insertions(+)
 create mode 100644 script/c++/mmap.cpp
 create mode 100755 script/c++/output/mmap
root@682ca971b3ce:/work_space/Tools/script/c++# ls
fork.c  loop.cpp  mmap.cpp  output  segv.c  shedule.cpp  shedule_nice.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# vim mmap.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# vim -h
VIM - Vi IMproved 8.2 (2019 Dec 12, compiled Aug 18 2023 04:12:26)

Usage: vim [arguments] [file ..]       edit specified file(s)
   or: vim [arguments] -               read text from stdin
   or: vim [arguments] -t tag          edit file where tag is defined
   or: vim [arguments] -q [errorfile]  edit file with first error

Arguments:
   --			Only file names after this
   -v			Vi mode (like "vi")
   -e			Ex mode (like "ex")
   -E			Improved Ex mode
   -s			Silent (batch) mode (only for "ex")
   -d			Diff mode (like "vimdiff")
   -y			Easy mode (like "evim", modeless)
   -R			Readonly mode (like "view")
   -Z			Restricted mode (like "rvim")
   -m			Modifications (writing files) not allowed
   -M			Modifications in text not allowed
   -b			Binary mode
   -l			Lisp mode
   -C			Compatible with Vi: 'compatible'
   -N			Not fully Vi compatible: 'nocompatible'
   -V[N][fname]		Be verbose [level N] [log messages to fname]
   -D			Debugging mode
   -n			No swap file, use memory only
   -r			List swap files and exit
   -r (with file name)	Recover crashed session
   -L			Same as -r
   -A			Start in Arabic mode
   -H			Start in Hebrew mode
   -T <terminal>	Set terminal type to <terminal>
























"mmap.cpp" [noeol] 20L, 633B
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <err.h>

#define BUFFER_SIZE 1000
#define ALLOC_SIZE 100 * 1024 * 1024
static char command[BUFFER_SIZE];
int main()
{
    pid_t pid = getpid();
    // snprintf 函数的工作方式类似于 printf 函数，但是它不会直接将结果输出到标准输出流，而是将结果格式化后存储到指定的字符数>组中。
    snprintf(command, BUFFER_SIZE, "cat /proc/%d/maps", pid);
    puts("command has been generated");
    puts("memory map before memory locate");
    system(command);
    void *new_memory;
    new_memory = mmap()
}
~
~
~
                                                                                                           17,1          All
   --not-a-term		Skip warning for input/output not being a terminal
# ~/.bashrc: executed by bash(1) for non-login shells.
# see /usr/share/doc/bash/examples/startup-files (in the package bash-doc)
# for examples

# If not running interactively, don't do anything
[ -z "$PS1" ] && return

# don't put duplicate lines in the history. See bash(1) for more options
# ... or force ignoredups and ignorespace
HISTCONTROL=ignoredups:ignorespace

# append to the history file, don't overwrite it
shopt -s histappend

# for setting history length see HISTSIZE and HISTFILESIZE in bash(1)
HISTSIZE=1000
HISTFILESIZE=2000

# check the window size after each command and, if necessary,
# update the values of LINES and COLUMNS.
shopt -s checkwinsize

# make less more friendly for non-text input files, see lesspipe(1)
[ -x /usr/bin/lesspipe ] && eval "$(SHELL=/bin/sh lesspipe)"
"~/.bashrc" 99L, 3106B                                                                                     1,1           Top
    alias fgrep='fgrep --color=auto'
   --ttyfail		Exit if input or output is not a terminal
   -u <vimrc>		Use <vimrc> instead of any .vimrc
























"mmap.cpp" [noeol] 20L, 633B
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <err.h>

#define BUFFER_SIZE 1000
#define ALLOC_SIZE 100 * 1024 * 1024
static char command[BUFFER_SIZE];
int main()
{
    pid_t pid = getpid();
    // snprintf 函数的工作方式类似于 printf 函数，但是它不会直接将结果输出到标准输出流，而是将结果格式化后存储到指定的字符数>组中。
    snprintf(command, BUFFER_SIZE, "cat /proc/%d/maps", pid);
    puts("command has been generated");
    puts("memory map before memory locate");
    system(command);
    void *new_memory;
    new_memory = mmap()
}
~
~
~
                                                                                                           17,1          All
#include <stdio.h>
   --noplugin		Don't load plugin scripts
   -p[N]		Open N tab pages (default: one for each file)
   -o[N]		Open N windows (default: one for each file)
   -O[N]		Like -o but split vertically
   +			Start at end of file
























"mmap.cpp" 20L, 634B
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <err.h>

#define BUFFER_SIZE 1000
#define ALLOC_SIZE 100 * 1024 * 1024
static char command[BUFFER_SIZE];
int main()
{
    pid_t pid = getpid();
    // snprintf 函数的工作方式类似于 printf 函数，但是它不会直接将结果输出到标准输出流，而是将结果格式化后存储到指定的字符数>组中。
    snprintf(command, BUFFER_SIZE, "cat /proc/%d/maps", pid);
    puts("command has been generated");
    puts("memory map before memory locate");
    system(command);
    void *new_memory;
    new_memory = mmap()
}
~
~
~
                                                                                                           17,1          All
#include <stdio.h>
   +<lnum>		Start at line <lnum>
   --cmd <command>	Execute <command> before loading any vimrc file
   -c <command>		Execute <command> after loading the first file
   -S <session>		Source file <session> after loading the first file
   -s <scriptin>	Read Normal mode commands from file <scriptin>
   -w <scriptout>	Append all typed commands to file <scriptout>
   -W <scriptout>	Write all typed commands to file <scriptout>
   -x			Edit encrypted files
   --startuptime <file>	Write startup timing messages to <file>
   -i <viminfo>		Use <viminfo> instead of .viminfo
   --clean		'nocompatible', Vim defaults, no plugins, no viminfo
   -h  or  --help	Print Help (this message) and exit
   --version		Print version information and exit
root@682ca971b3ce:/work_space/Tools/script/c++#    -c <command>         Execute <command> after loading the first file
bash: command: No such file or directory
root@682ca971b3ce:/work_space/Tools/script/c++# vim -c 'set encoding=utf-8' mmap.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# vim ~/.bashrc
root@682ca971b3ce:/work_space/Tools/script/c++# source ~/.bashrc
root@682ca971b3ce:/work_space/Tools/script/c++# vim mmap.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# which vim
/usr/bin/vim
root@682ca971b3ce:/work_space/Tools/script/c++# ls
fork.c  loop.cpp  mmap.cpp  output  segv.c  shedule.cpp  shedule_nice.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# vim mmap.cpp
root@682ca971b3ce:/work_space/Tools/script/c++# git st
On branch main
Your branch is up to date with 'origin/main'.

Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
	modified:   mmap.cpp

no changes added to commit (use "git add" and/or "git commit -a")
root@682ca971b3ce:/work_space/Tools/script/c++# git co .
git: 'co' is not a git command. See 'git --help'.

The most similar commands are
	commit
	cm
root@682ca971b3ce:/work_space/Tools/script/c++# git checkout .
Updated 1 path from the index
root@682ca971b3ce:/work_space/Tools/script/c++# git pull
remote: Enumerating objects: 35, done.
remote: Counting objects: 100% (35/35), done.
remote: Compressing objects: 100% (14/14), done.
remote: Total 25 (delta 15), reused 21 (delta 11), pack-reused 0
Unpacking objects: 100% (25/25), 7.90 KiB | 218.00 KiB/s, done.
From github.com:ddx533534/Tools
   33bc3a4..0ade864  main       -> origin/main
Updating 33bc3a4..0ade864
Fast-forward
 script/1core/1core-2process-nice.txt |   9 ---------
 script/c++/mmap-test.cpp             |  17 +++++++++++++++++
 script/c++/mmap-test.log             |  35 +++++++++++++++++++++++++++++++++++
 script/c++/mmap.cpp                  |  22 ++++++++++++++++++++--
 script/c++/output/mmap               | Bin 49504 -> 16320 bytes
 script/c++/output/mmap-test          | Bin 0 -> 16000 bytes
























"filemap.cpp" [New]

~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
                                                                                                           0,0-1         All
#include<stdlib.h>
#include<stdio.h>
#
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
~
-- INSERT --                                                                                               3,2           All
