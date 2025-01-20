This challenge requires us to look at a couple of ARM assembly and see what they will return.
The two registers you will need to learn about are `pc` (program counter) and `lr` (link register). 

Source code:
```
int main(){
	int key=0;
	printf("Daddy has very strong arm! : ");
	scanf("%d", &key);
	if( (key1()+key2()+key3()) == key ){
		printf("Congratz!\n");
		int fd = open("flag", O_RDONLY);
		char buf[100];
		int r = read(fd, buf, 100);
		write(0, buf, r);
	}
	else{
		printf("I have strong leg :P\n");
	}
	return 0;
}
```
We have to figure out what the three keys will evaluate to and add them all. Thankfully we are handed
the assembly with the addresses we need. Let's figure out each key:

### key1 
This is returning the program counter, which is the current instruction address + 0x8, that means the value is 0x8ce4
```
   0x00008cdc <+8>:	mov	r3, pc
   0x00008ce0 <+12>:	mov	r0, r3
   0x00008ce4 <+16>:	sub	sp, r11, #0
```

### key2
A little crazier, we have a swap to thumb mode (16-bit instructions). We add the program counter into r3 = 0x8d0a (instruction addr + 0x4), then 4 is added to r3 = 0x8d0c
```
int key2(){
	asm(
	"push	{r6}\n"
	"add	r6, pc, $1\n"
	"bx	r6\n"
	".code   16\n"
	"mov	r3, pc\n"    //    0x00008d04
	"add	r3, $0x4\n"  //    0x00008d06
	"push	{r3}\n"      //    0x00008d08
	"pop	{pc}\n"
	".code	32\n"
	"pop	{r6}\n"
	);
}
```

### key3 
This is returning the link register, which is the return address for when it's
finished.
```
int key3(){
	asm("mov r3, lr\n");
}
```
We make our call to key3 here in main: the link register is equal to 0x8d80 as that's the instruction we return to.
```
   0x00008d7c <+64>:	bl	0x8d20 <key3>
   0x00008d80 <+68>:	mov	r3, r0
```

- key1 = 0x8ce4
- key2 = 0x8d0c
- key3 = 0x8d80
- key1 + key2 + key3 = 0x1A770

Once converted to a decimal you can get the flag:
```
/ $ ./leg
Daddy has very strong arm! : 108400
Congratz!
(Flag Redacted)
```