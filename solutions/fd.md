This challenge has the source code:
```
char buf[32];
int main(int argc, char* argv[], char* envp[]){
        if(argc<2){
                printf("pass argv[1] a number\n");
                return 0;
        }
        int fd = atoi( argv[1] ) - 0x1234;
        int len = 0;
        len = read(fd, buf, 32);
        if(!strcmp("LETMEWIN\n", buf)){
                printf("good job :)\n");
                system("/bin/cat flag");
                exit(0);
        }
        printf("learn about Linux file IO\n");
        return 0;
}
```
This program has the line `len = read(fd, buf, 32);` which when looking at the [man page for read](https://www.man7.org/linux/man-pages/man2/read.2.html)
the function signature for read is: `ssize_t read(int fd, void buf[.count], size_t count);` We can see that
`fd` is defined as `int fd = atoi( argv[1] ) - 0x1234;` If we can make `fd` to equal 0, we can just read from
standard input and write to buf. This means the first arguement to our program need to make fd end up being 0.

To do this run the shell `./fd 4660`, as `0x1234 = 4660` in decimal. Now just type "LETMEIN" and hit return and you
will have your flag.