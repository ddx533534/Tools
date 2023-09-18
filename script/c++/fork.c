#include<unistd.h>
#include<stdio.h>
#include<stdlib.h>

static void child(){
	printf("I am the child! And my pid is %d !\n",getpid());
	exit(EXIT_SUCCESS);
}

static void parent(pid_t pid_c){
	printf("I am the parent! And my pid is %d !And the child pid is %d\n",getpid(),pid_c);
	exit(EXIT_SUCCESS);
}

int main(){
	pid_t pid;
	pid = fork();
	if(pid == -1)
		err(EXIT_FAILURE,"fork() system call failed!");
	if(pid == 0){
		child();
	} else{
		parent(pid);
	}
	err(EXIT_FAILURE,"function error!");
}

