#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <err.h>
#include <time.h>
#include <sys/mman.h>
#include <signal.h>
#define NSECS_PER_SCE 1000000000UL
#define CACHE_LINE_SIZE 64
#define NLOOP (4 * 1024UL * 1024 * 1024)

static inline long diff_nsec(struct timespec before, struct timespec after)
{
    return (after.tv_sec * NSECS_PER_SCE + after.tv_nsec) -
           (before.tv_sec * NSECS_PER_SCE + before.tv_nsec);
}

int main(int argc, char *argv[])
{
    char *progressname = argv[0];
    if (argc < 2)
    {
        err(EXIT_FAILURE, "At least one parameter needed!");
    }
    register int size;
    size = atoi(argv[1]) * 1024;
    if (!size)
    {
        err(EXIT_FAILURE, "Size should be less than 0");
    }

    char *buffer;
    buffer = (char *)mmap(NULL, size, PROT_READ | PROT_WRITE, MAP_PRIVATE | MAP_SHARED, -1, 0);
    if (buffer == (char *)-1)
    {
        err(EXIT_FAILURE, "mmap failed");
    }

    struct timespec before, after;
    clock_gettime(CLOCK_MONOTONIC, &before);

    for (long i = 0; i < NLOOP / (size / CACHE_LINE_SIZE); i++)
    {
        for (long j = 0; j < size; j += CACHE_LINE_SIZE)
        {
            buffer[j] = 0;
        }
    }

    clock_gettime(CLOCK_MONOTONIC, &after);
    printf("%f\n", (double)diff_nsec(before, after) / NLOOP);

    if (munmap(buffer, size) == -1)
    {
        err(EXIT_FAILURE, "munmap failed!");
    }
    exit(EXIT_SUCCESS);
}