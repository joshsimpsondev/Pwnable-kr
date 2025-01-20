When looking at the binary in ghidra you see a main function that contains
two functions of interest: `init_ABCDEFG` and `ropme`

Inside init_ABCDEFG:
```
  iVar2 = rand();
  a = iVar2 * -0x21524111 + (uint)(0xcafebabd < (uint)(iVar2 * -0x21524111)) * 0x35014542;
  iVar2 = rand();
  b = iVar2 * -0x21524111 + (uint)(0xcafebabd < (uint)(iVar2 * -0x21524111)) * 0x35014542;
  iVar2 = rand();
  c = iVar2 * -0x21524111 + (uint)(0xcafebabd < (uint)(iVar2 * -0x21524111)) * 0x35014542;
  iVar2 = rand();
  d = iVar2 * -0x21524111 + (uint)(0xcafebabd < (uint)(iVar2 * -0x21524111)) * 0x35014542;
  iVar2 = rand();
  e = iVar2 * -0x21524111 + (uint)(0xcafebabd < (uint)(iVar2 * -0x21524111)) * 0x35014542;
  iVar2 = rand();
  f = iVar2 * -0x21524111 + (uint)(0xcafebabd < (uint)(iVar2 * -0x21524111)) * 0x35014542;
  iVar2 = rand();
  g = iVar2 * -0x21524111 + (uint)(0xcafebabd < (uint)(iVar2 * -0x21524111)) * 0x35014542;
  sum = g + a + b + c + d + e + f;
```
There isn't a predictable way to guess sum, so lets move on to ropme:
```
  printf("Select Menu:");
  __isoc99_scanf(&DAT_080a0519,&local_14);
  getchar();
  if (local_14 == a) {
    A();
  }
  else if (local_14 == b) {
    B();
  }
  else if (local_14 == c) {
    C();
  }
  else if (local_14 == d) {
    D();
  }
  else if (local_14 == e) {
    E();
  }
  else if (local_14 == f) {
    F();
  }
  else if (local_14 == g) {
    G();
  }
  else {
    printf("How many EXP did you earned? : ");
                    /* gets == vuln, look around here */
    gets(xp_input_char);
    xp_input = atoi(xp_input_char);
    if (xp_input == sum) {
      flag_file = open("flag",0);
      bytes_read = read(flag_file,xp_input_char,100);
      xp_input_char[bytes_read] = '\0';
      puts(xp_input_char);
      close(flag_file);
                    /* WARNING: Subroutine does not return */
      exit(0);
    }
    puts("You\'d better get more experience to kill Voldemort");
  }
  return 0;
```
Ok, so this challenge is going to be just a simple buffer overflow to the stack pointer to
go the read the flag instruction, when looking at the stack we can figure out that we have
120 bytes of data to write, and then we can write to the stack pointer.
```
                 undefined ropme()
 undefined         AL:1           <RETURN>                                XREF[2]:     080a00f8(W), 
                                                                                       080a012c(W)  
 undefined4        EAX:4          xp_input                                XREF[1]:     080a00f8(W)  
 undefined4        EAX:4          bytes_read                              XREF[1]:     080a012c(W)  
 undefined4        Stack[-0x10]:4 flag_file                               XREF[3]:     080a011d(W), 
 undefined4        Stack[-0x14]:4 local_14                                XREF[8]:     080a0022(*), 
 undefined1[100]   Stack[-0x78]   xp_input_char                           XREF[4]:     080a00e5(*), 
                                                                                       080a00f4(*), 
                                                                                       080a0125(*), 
```
So lets try to jump to the address 0x080a010b to go back to printing the flag. But there is a problem with this idea:
the range the ropme function is 0x080a0XXX. The unicode character for "0x0A" is a linefeed, so trying to input
this will only cut off our input.

Instead, we will have to jump to each function that exists in `A()`, `B()`, `C()`, `D()`, `E()`, `F()`, `G()`, and get them to print what
we want. For example the function `A()` contains:
```
void A(void) {
  printf("You found \"Tom Riddle\'s Diary\" (EXP +%d)\n",a);
  return;
}
```
How do we do the jump? We first need to write our stack pointer to a RET instruction, I've chosen the one in `A()`
(0x0809fe69) and then write to the instruction we want to jump to, and repeat. At the end we can jump to the ropme call in
`main` (0x0809fffc) and put the proper sum in to get our flag. Don't forget the little endian notation when writing these to the
stack. The stack looks like this:

```
0809fe69 --> 0809fe4b --> 0809fe69 --> 0809fe6a --> 0809fe69 --> 0809fe89 --> 0809fe69 
--> 0809fea8 --> 0809fe69 --> 0809fec7 --> 0809fe69 --> 0809fee6 --> 0809fe69 --> 0809ff05
--> 0809fe69 --> 0809fffc
```
You should have all the xp values you need to input the correct sum. You can see my solution here: [horcruxes.rs](../src/horcruxes/horcruxes.rs) 