#include <stdio.h>
#include <unistd.h>

int main(int argc, char **argv) {
    pid_t pid = fork();

    if (pid == 0) { // child process
        printf("This is in child process\n");
    } else if (pid == -1) { // fork failed
        perror("Fork failed");
    } else { // parent process
        printf("This is parent process and child pid is: %d\n", pid);
    }

    return 0;
}
