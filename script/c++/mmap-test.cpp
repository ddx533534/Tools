#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/mman.h>
#include <err.h>

#define BUFFER_SIZE 1000
#define ALLOC_SIZE 1 * 1024 * 1024
static char command[BUFFER_SIZE];
int main()
{
    // 单独测试mmap内存分配的系统调用次数，注意会造成内存泄露，建议运行完毕后重启系统
    void *new_momery;
    new_momery = mmap(NULL, ALLOC_SIZE, PROT_READ | PROT_WRITE,
                      // 私有匿名映射，每个进程都有自己独立的拷贝，并且修改不可见
                      MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
}