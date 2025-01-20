We are given the code:
```
int filter(char* cmd){
        int r=0;
        r += strstr(cmd, "flag")!=0;
        r += strstr(cmd, "sh")!=0;
        r += strstr(cmd, "tmp")!=0;
        return r;
}
int main(int argc, char* argv[], char** envp){
        putenv("PATH=/thankyouverymuch");
        if(filter(argv[1])) return 0;
        system( argv[1] );
        return 0;
}
```
As we see the filter function will prevent "flag", "sh", "tmp" from showing up
in our argument, or it will exit early. We will have to use some trickery to cat
the flag without actually typing the "flag" filename out. 

There are tons of solutions to this problem, you can even start a python session and do whatever with that.
The solution I found is to use the [printf](https://linuxize.com/post/bash-printf-command/) shell command to trick
the program to give us the flag. Using the `%s` specifier, we can split up "flag" into multiple parts to get through the filter.

If we do: `printf "/bin/cat %s%s" "fla" "g"` and execute it with `$()`, the flag is easy to get.
Surround it with a single quote so the current shell doesn't try to resolve it, and you get the flag:

```
cmd1@pwnable:~./cmd1 '$(printf "/bin/cat %s%s" "fla" "g")'
(Flag Redacted)
```