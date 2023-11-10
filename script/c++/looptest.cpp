#include <stdio.h>
#include <time.h>
#include <chrono>
#include <iostream>
#include <ctime>

int main()
{
    int LIMIT = 1000000;
    int LOOP = 100;
    int i = 0;
    int res = 0;
    // 获取当前时间
    std::time_t now = std::time(nullptr);

    // 转换为 long 类型
    long start = static_cast<long>(now);

    while (true)
    {
        if (i > LIMIT)
        {
            break;
        }
        i++;
        for (int j = 0; j < LOOP; j++)
        {
            res += j;
        }
        res = 0;
    }
    // 获取当前时间
    now = std::time(nullptr);

    // 转换为 long 类型
    long end = static_cast<long>(now);

    printf("time cost  %d ms", (end - start));
}