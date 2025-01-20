We are given the source code in the form of a link:
[source](http://cboard.cprogramming.com/c-programming/114023-simple-blackjack-program.html)

When we select to play the game and get into the game itself, we are presented with:
```
Cash: $500
-------
|D    |
|  2  |
|    D|
-------

Your Total is 2

The Dealer Has a Total of 10

Enter Bet: $
```
We can enter a bet and depending on if we win we get money. We have to reach a million dollars to win,
so we can play all night and day for the flag or try to find a shortcut. Let's take a  look at how the game handles
the bet we put in:
```
int bet;
....
int betting() //Asks user amount to bet
{
 printf("\n\nEnter Bet: $");
 scanf("%d", &bet);
 
 if (bet > cash) //If player tries to bet more money than player has
 {
        printf("\nYou cannot bet more money than you have.");
        printf("\nEnter Bet: ");
        scanf("%d", &bet);
        return bet;
 }
 else return bet;
} // End Function
```
We can just enter a negative bet, and if we look at how the game handles a loss:
```
if(dealer_total==21) //Is dealer total is 21, loss
{
  printf("\nDealer Has the Better Hand. You Lose.\n");
  loss = loss+1;
  cash = cash - bet;
  printf("\nYou have %d Wins and %d Losses. Awesome!\n", won, loss);
  dealer_total=0;
  askover();
} 
```
All it does is: `cash = cash - bet`, so if we enter a negative bet and play the worst blackjack game
of our life, we will be millionaires.

So let's do that:
```
Enter Bet: $-11000000


Would You Like to Hit or Stay?
Please Enter H to Hit or S to Stay.
s

You Have Chosen to Stay at 2. Wise Decision!

The Dealer Has a Total of 14
The Dealer Has a Total of 18
Dealer Has the Better Hand. You Lose.

You have 0 Wins and 1 Losses. Awesome!

Would You Like To Play Again?
Please Enter Y for Yes or N for No
 Y
(Flag Redacted)

Cash: $11000500
-------
|H    |
|  A  |
|    H|
-------

Your Total is 11

The Dealer Has a Total of 7

Enter Bet: $
```