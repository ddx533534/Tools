#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <string.h>
#include <err.h>
#define BUFFER_SIZE 1000
#define ALLOCATION_SIZE 10 * 1024

static char command[BUFFER_SIZE];

/**
 * 释放内存，关闭文件映射、文件符和指令内存
 * @param fpd 文件描述符
 * @param file_contents 文件映射起始地址
 * @param command 指令
 * @param ret 释放内存时当前程序的状态
 */
void release(int fpd, void *file_contents, char *command, int ret)
{
   if (fpd == -1)
   {
      err(EXIT_FAILURE, "file open failed");
   }
   else
   {
      close(fpd);
   }
   if (file_contents != NULL)
   {
      munmap(file_contents, ALLOCATION_SIZE);
   }
   if (command != NULL)
   {
      free(command);
   }
   exit(ret);
}

int main()
{
   pid_t pid = getpid();
   snprintf(command, BUFFER_SIZE, "cat /proc/%d/maps", pid);
   puts("----- before map file -----");
   system(command);
   fflush(stdout);

   // 以可读可写模式打开文件
   int fpd = open("testmmap", O_RDWR);
   if (fpd == -1)
   {
      release(fpd, NULL, command, EXIT_FAILURE);
   }
   char *file_contents;
   file_contents = (char *)mmap(NULL, ALLOCATION_SIZE,
                                PROT_READ | PROT_WRITE, MAP_SHARED, fpd, 0);
   if (file_contents == (char *)-1)
   {
      release(fpd, file_contents, command, EXIT_FAILURE);
   }
   puts("----- after map file -----");
   system(command);
   fflush(stdout);

   puts("----- map content is -----");
   printf("%c\n", file_contents);

   puts("----- write map content -----");
   char new_content[] = "hello world!";
   memcpy(file_contents, new_content, strlen(new_content));

   puts("----- after writing content is -----");
   printf("%c\n", file_contents);
   release(fpd, file_contents, command, EXIT_SUCCESS);
}