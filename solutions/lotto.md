Relevant source code:
```
unsigned char submit[6];

void play(){

        int i;
        printf("Submit your 6 lotto bytes : ");
        fflush(stdout);

        int r;
        r = read(0, submit, 6);

        printf("Lotto Start!\n");
        //sleep(1);

        // generate lotto numbers
        int fd = open("/dev/urandom", O_RDONLY);
        if(fd==-1){
                printf("error. tell admin\n");
                exit(-1);
        }
        unsigned char lotto[6];
        if(read(fd, lotto, 6) != 6){
                printf("error2. tell admin\n");
                exit(-1);
        }
        for(i=0; i<6; i++){
                lotto[i] = (lotto[i] % 45) + 1;         // 1 ~ 45
        }
        close(fd);

        // calculate lotto score
        int match = 0, j = 0;
        for(i=0; i<6; i++){
                for(j=0; j<6; j++){
                        if(lotto[i] == submit[j]){
                                match++;
                        }
                }
        }

        // win!
        if(match == 6){
                system("/bin/cat flag");
        }
        else{
                printf("bad luck...\n");
        }

}
```
Normally the chances that we would win the intended game would be very low, and most of the code is good,
the only thing that's wrong is how we are checking the score:

```
int match = 0, j = 0;
for(i=0; i<6; i++){
        for(j=0; j<6; j++){
                if(lotto[i] == submit[j]){
                        match++;
                }
        }
}
```
In this case, the check won't work as we check for matches too often without checking duplicates,
and our chances of winning are actually much greater than what was intended. All
we have to do is enter chars that are between 1-45 in decimal (most of ASCII punctuation symbols are in this range) and 
we have a decent chance of winning.

Getting the flag:
```
lotto@pwnable:~$ ./lotto
- Select Menu -
1. Play Lotto
2. Help
3. Exit
1
Submit your 6 lotto bytes : !!!!!!
Lotto Start!
bad luck...
.......
(A couple more attempts)
.......
- Select Menu -
1. Play Lotto
2. Help
3. Exit
Submit your 6 lotto bytes : !!!!!!!
Lotto Start!
(Flag Redacted)
```