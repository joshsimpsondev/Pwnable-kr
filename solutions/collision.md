The challenge source code:
```
unsigned long hashcode = 0x21DD09EC;
unsigned long check_password(const char* p){
        int* ip = (int*)p;
        int i;
        int res=0;
        for(i=0; i<5; i++){
                res += ip[i];
        }
        return res;
}

int main(int argc, char* argv[]){
        if(argc<2){
                printf("usage : %s [passcode]\n", argv[0]);
                return 0;
        }
        if(strlen(argv[1]) != 20){
                printf("passcode length should be 20 bytes\n");
                return 0;
        }

        if(hashcode == check_password( argv[1] )){
                system("/bin/cat flag");
                return 0;
        }
        else
                printf("wrong passcode.\n");
        return 0;
}
```
The goal of this challenge is to pass 20 bytes of characters to the program such that
it will hash out to be `0x21DD09EC`, each 4 byte block we put in will be treated as an int and added
to the return value in `check_password`.

So if we can pass five integers in char form to equal the hash we can get the flag easily. To construct what we want to send
through lets send "0x01010101" and send the remainder that we need. (We can't send 0x00000000 as that's the null character and would be interpreted as the end of our string.) 

Then: 0x21dd09ec - (0x01010101 * 4) = 0x1dd905e8. (You can convert to decimal to confirm for yourself).

Using what we learned, and using python to pass the bytes we want:
```
col@pwnable:~$ ./col $(python -c "print '\x01\x01\x01\x01'*4+'\xe8\x05\xd9\x1d'")
(Flag Redacted)
```
I'm using the shell notation of `$(<command goes here>)` which is a command substitution, you can read about it [here](https://superuser.com/questions/935374/difference-between-and-in-a-shell-script).
