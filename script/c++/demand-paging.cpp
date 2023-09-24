#include <unistd.h>
#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <string.h>
#include <err.h>

#define ALLOCATION_SIZE 100 * 1024
#define CYCLE 10
#define PAGE_SIZE 2048

int main()
{
    char *p, *s;
    // 获取当前时间，并格式化保存
    time_t t = time(NULL);
    s = ctime(&t);
    printf("%.*s:before allcation , please press Enter key:\n", (int(strlen(s) - 1)), s);
    // 等待用户输入字符并以回车结束
    getchar();

    // 分配内存
    p = (char *)malloc(ALLOCATION_SIZE);
    if (p == NULL)
    {
        err(EXIT_FAILURE, "malloc failed!");
    }

    t = time(NULL);
    s = ctime(&t);
    printf("%.*s:allocated 100KB , please press Enter key:\n", (int(strlen(s) - 1)), s);
    getchar();

    for (int i = 0; i <= ALLOCATION_SIZE; i += PAGE_SIZE)
    {
        p[i] = 0;
        if (i / (ALLOCATION_SIZE / CYCLE) != 0 && i % (ALLOCATION_SIZE / CYCLE) == 0)
        {
            t = time(NULL);
            s = ctime(&t);
            printf("%.*s:allocated %dKB\n", (int(strlen(s) - 1)), s, i / 1024);
            sleep(1);
        }
    }
    exit(EXIT_SUCCESS);
}
