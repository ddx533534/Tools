#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <err.h>
#include <time.h>
#include <signal.h>
#define NLOOP_FOR_EXTIMATION 1000000000UL
#define NSECS_PER_MSEC 1000000UL
#define NSECS_PER_SCE 1000000000UL

static inline long diff_nsec(struct timespec before, struct timespec after)
{
    return (after.tv_sec * NSECS_PER_SCE + after.tv_nsec) -
           (before.tv_sec * NSECS_PER_SCE + before.tv_nsec);
}

static unsigned long loops_per_msec()
{
    struct timespec before, after;
    clock_gettime(CLOCK_MONOTONIC, &before);
    unsigned long i;
    for (i = 0; i < NLOOP_FOR_EXTIMATION; i++)
        ;
    clock_gettime(CLOCK_MONOTONIC, &after);
    // return loop times per msecond!
    return NLOOP_FOR_EXTIMATION * NSECS_PER_MSEC / diff_nsec(before, after);
}

static inline void load(unsigned long nloop)
{
    unsigned long i;
    for (i = 0; i < nloop; i++)
        ;
}

static void child(int id, struct timespec *buf,
                  int nrecord, unsigned long nloop_per_resol, struct timespec start)
{
    int i;
    for (i = 0; i < nrecord; i++)
    {
        struct timespec ts;
        load(nloop_per_resol);
        clock_gettime(CLOCK_MONOTONIC, &ts);
        buf[i] = ts;
    }

    for (i = 0; i < nrecord; i++)
    {
        printf("%d\t%ld\t%d\n",
               id,
               diff_nsec(start, buf[i]) / NSECS_PER_MSEC,
               (i + 1) * 100 / nrecord);
    }
    exit(EXIT_SUCCESS);
}

static void parent(int nproc)
{
    int i = 0;
    for (i = 0; i < nproc; i++)
    {
        /**
         * 等待所有子进程结束
         */
        wait(NULL);
    }
}

void check_parameters(int argc, char *argv[])
{
    if (argc < 4)
    {
        printf("You need input at least 3  extra parameters!");
        exit(EXIT_FAILURE);
    }
    int processCount = atoi(argv[1]);
    int processTime = atoi(argv[2]);
    int processInterval = atoi(argv[3]);
    if (processCount < 1 || processTime < 1 || processInterval < 1)
    {
        printf("Any of parameters should't be less than 1!");
        exit(EXIT_FAILURE);
    }
    if (processTime % processInterval)
    {
        printf("Time shuold be multiple of Interval");
        exit(EXIT_FAILURE);
    }
}
static pid_t *pids;

int main(int argc, char *argv[])
{
    check_parameters(argc, argv);
    int processCount = atoi(argv[1]);
    int processTime = atoi(argv[2]);
    int processInterval = atoi(argv[3]);
    int recordTimes = processTime % processInterval;
    struct timespec *logbuf = (timespec *)malloc(recordTimes * sizeof(struct timespec));
    if (!logbuf)
    {
        warn("logbuf allocation failed!");
        free(logbuf);
        exit(EXIT_FAILURE);
    }
    puts("estimating workload which takes just one milisecond!");
    unsigned long nloop_per_resol = loops_per_msec() * processInterval;
    printf("estimation continue %ld\n", nloop_per_resol / processInterval);
    puts("end estimation! ");
    pids = (pid_t *)malloc(processCount * sizeof(pid_t));
    if (pids == NULL)
    {
        warn("pids allocation failed!");
        free(logbuf);
        free(pids);
        exit(EXIT_FAILURE);
    }
    // holly shit , what a lot tired preparation!

    int ret = EXIT_FAILURE;
    struct timespec start;
    clock_gettime(CLOCK_MONOTONIC, &start);
    int i, ncreated;
    printf("start came into the main process");
    for (i = 0, ncreated = 0; i < processCount; i++, ncreated++)
    {
        // record  every subprocess id;
        pids[i] = fork();
        if (pids[i] < 0)
        {
            printf("fork failed!");
            for (size_t i = 0; i < ncreated; i++)
            {
                if (kill(pids[i], SIGINT) < 0)
                {
                    warn("killing %d subprocess fialed!", pids[i]);
                }
            }
            free(logbuf);
            free(pids);
            exit(EXIT_FAILURE);
        }
        else if (pids[i] == 0)
        {
            child(i, logbuf, recordTimes, nloop_per_resol, start);
        }
        else
        {
        }
    }
    ret = EXIT_SUCCESS;
    if (ret == EXIT_FAILURE)
    {
        for (size_t i = 0; i < ncreated; i++)
        {
            if (wait(NULL) < 0)
            {
                warn("killing %d subprocess fialed!", pids[i]);
            }
        }
    }
    free(logbuf);
    free(pids);
    exit(ret);
}
