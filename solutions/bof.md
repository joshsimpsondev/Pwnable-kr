The source code to this problem is short and simple: 
```
void func(int key){
	char overflowme[32];
	printf("overflow me : ");
	gets(overflowme);	// smash me!
	if(key == 0xcafebabe){
		system("/bin/sh");
	}
	else{
		printf("Nah..\n");
	}
}
int main(int argc, char* argv[]){
	func(0xdeadbeef);
	return 0;
}
```
We see the call to gets which is very dangerous to call as it doesn't check
how much we are writing to our buffer. We can check the man page as well,
[gets](https://www.man7.org/linux/man-pages/man3/gets.3.html) here says in the
description: *Never use this function*.

When we look inside ghidra we see the stack of func
```
undefined func(undefined4 param_1)
undefined         AL:1           <RETURN>
undefined4        Stack[0x4]:4   param_1                                 XREF[1]:     00010654(R)  
undefined4        Stack[-0x10]:4 local_10                                XREF[2]:     00010638(W),
00010677(R)  
undefined1[32]    Stack[-0x30]   overflowme                              XREF[1]:     00010649(*)  
undefined4        Stack[-0x4c]:4 local_4c                                XREF[4]:     0001063d(*),
```
Since overflowme is at -0x30 and param_1 (key) is at 0x4 and 0x30 + 0x4 = 52 bytes all
we have to do is write 52 bytes and then the key value we wish to have. We know this to be
the value 0xcafebabe, though keep in mind that this will be stored as little endian. Check out
the source for a simple solution: [bof.rs](../src/bof/bof.rs)