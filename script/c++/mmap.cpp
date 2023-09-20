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
    puts("memory map before memory located");
    system(command);
    void *new_memory;
    new_memory = mmap(NULL, ALLOC_SIZE, PROT_READ | PROT_WRITE,
                      // 私有匿名映射，每个进程都有自己独立的拷贝，并且修改不可见
                      MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (new_memory == (void *)-1)
    {
        err(EXIT_FAILURE, "mmap() failed");
    };
    printf("succeeded to allocate memory:address = %p; size = 0x%x\n", new_memory, ALLOC_SIZE);
    puts("memory map after memory located");
    system(command);
    // 使用完后及时释放空间
    if (munmap(new_memory, ALLOC_SIZE) == -1)
    {
        err(EXIT_FAILURE, "munmap() failed");
    }
    else
    {
        exit(EXIT_SUCCESS);
    }
}