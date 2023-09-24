#include <unistd.h>
#include <sys/wait.h>
#include <stdio.h>
#include <time.h>
#include <stdlib.h>
#include <string.h>
#include <err.h>

#define ALLOCATION_SIZE 100 * 1024 * 1024
#define PAGE_SIZE 4096

#define BUFFER_SIZE 100
static char command[BUFFER_SIZE];
static char *p;
static void child_fn(char *p)
{
    printf("-- child ps info before memory access \n");
    fflush(stdout);

    snprintf(command, BUFFER_SIZE, "ps -o pid, comm,vsz,rss,min_flt,maj_flt | grep %d", getpid());
    system(command);

    printf("-- free memory info before memory access \n");
    fflush(stdout);
    system("free");

    for (int i = 0; i < ALLOCATION_SIZE; i += PAGE_SIZE)
    {
        p[i] = 0;
    }
    printf("-- child ps info after memory access \n");
    fflush(stdout);
    system(command);

    printf("-- free memory info before memory access \n");
    fflush(stdout);
    system("free");
    exit(EXIT_SUCCESS);
}

static void parent_fn()
{
    wait(NULL);
    exit(EXIT_SUCCESS);
}

int main()
{
    char *buf;
    p = (char *)malloc(ALLOCATION_SIZE);
    if (p == NULL)
    {
        err(EXIT_FAILURE, "malloc failed!");
    }
    int i;
    for (i = 0; i < ALLOCATION_SIZE; i += PAGE_SIZE)
    {
        p[i] = 0;
    }
    printf("-- free memory info before fork \n");
    fflush(stdout);
    system("free");
    pid_t pid = fork();
    if (pid == 0)
    {
        child_fn(p);
    }
    else if (pid > 0)
    {
        parent_fn();
    }
    else
    {
        err(EXIT_FAILURE, "fork failed!");
    }
}