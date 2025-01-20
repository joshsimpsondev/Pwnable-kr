We get a hint on operator priority, so this is something to look out for.


Here is the source code to this challenge:
```
#define PW_LEN 10
#define XORKEY 1

void xor(char* s, int len){
        int i;
        for(i=0; i<len; i++){
                s[i] ^= XORKEY;
        }
}

int main(int argc, char* argv[]){

        int fd;
        if(fd=open("/home/mistake/password",O_RDONLY,0400) < 0){
                printf("can't open password %d\n", fd);
                return 0;
        }

        printf("do not bruteforce...\n");
        sleep(time(0)%20);

        char pw_buf[PW_LEN+1];
        int len;
        if(!(len=read(fd,pw_buf,PW_LEN) > 0)){
                printf("read error\n");
                close(fd);
                return 0;
        }

        char pw_buf2[PW_LEN+1];
        printf("input password : ");
        scanf("%10s", pw_buf2);

        // xor your input
        xor(pw_buf2, 10);

        if(!strncmp(pw_buf, pw_buf2, PW_LEN)){
                printf("Password OK\n");
                system("/bin/cat flag\n");
        }
        else{
                printf("Wrong Password\n");
        }

        close(fd);
        return 0;
}
```

If we look at the line `if(fd=open("/home/mistake/password",O_RDONLY,0400) < 0)` and look at it intensely,
we can see the mistake that occurred. The portion: `open("/home/mistake/password",O_RDONLY,0400) < 0` is 
evaluated before the assignment to fd ([read more](https://en.cppreference.com/w/c/language/operator_precedence)). 
The [open](https://www.man7.org/linux/man-pages/man2/open.2.html) function
returns an integer that is negative, and we are left with `postive int < 0` which evaluates to zero.

This means when we later read with `if(!(len=read(fd,pw_buf,PW_LEN) > 0))`, the file descriptor is zero, which is stdin.
The password we wrote is then xor'ed with 1. That means if we put in the character 'A' the least significant
bit is flipped and turns into '@'.

Let's use this knowledge now:
```
mistake@pwnable:~$ ./mistake
do not bruteforce...
AAAAAAAAAA
input password : @@@@@@@@@@
Password OK
(Flag Redacted)
```