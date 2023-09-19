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
    // snprintf 函数的工作方式类似于 printf 函数，但是它不会直接将结果输出到标准输出流，而是将结果格式化后存储到指定的字符数组中。
    snprintf(command, BUFFER_SIZE, "cat /proc/%d/maps", pid);
    puts("command has been generated");
    puts("memory map before memory locate");
    system(command);
    void *new_memory;
    new_memory = mmap()
}