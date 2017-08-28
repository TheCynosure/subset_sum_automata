# subset_sum_automata

Solved the Subset Sum Automata problem on Reddit r/dailyprogrammer. 
https://www.reddit.com/r/dailyprogrammer/comments/6vyihu/20170825_challenge_328_hard_subset_sum_automata/

All finished here. Works nicely and as intended. If you find any bugs please open an issue.

### What is this?

This program mixes Conway's game of life and the Subset Sum problem as mentioned in the problem on Reddit. The point is that a cell takes the subset sum of it's 8 neighbor cells and if they can sum to the __target value__, then the cell is rewarding by having its value incremented by the __reward value__. If not then it is penalized by having its value decremented by the __penalty value__.

### Usage:

__Basic Usage__ is as follows:
```bash
./subset_sum_automata 5/1/-1
```
5 Here is the __Target value__.

1 is the __Reward value__ or Increment as I call it.

-1 is the __Penalty value__ or the Decrement as I call it.


__Argument Usage__ is as follows:

```bash
./subset_sum_automata --help
```
Prints the help screen obviously.

```bash
./subset_sum_automata 5/1/-1 -w 100
or
./subset_sum_automata 5/1/-1 --width 100
```
Sets the boards width to 100. In terms of characters it is actually double this amount as there is a space between each of the block characters.

__Default Value for Width:__ 10

```bash
./subset_sum_automata 5/1/-1 -h 100
or
./subset_sum_automata 5/1/-1 --height 100
```
Sets the boards height to 100. This is its actual height unlike the width argument.

__Default Value for Height:__ 10

```bash
./subset_sum_automata 5/1/-1 -i 0
or
./subset_sum_automata 5/1/-1 --min 0
```
Sets the minimum value for the random values generated on the board on start.

__Default Value for Minimum:__ 0

```bash
./subset_sum_automata 5/1/-1 --a 1000
or
./subset_sum_automata 5/1/-1 --max 1000
```
Sets the maximum value for the random values generated on the board on start.

__Default Value for Maximum:__ 100

```bash
./subset_sum_automata 5/1/-1 -f
or
./subset_sum_automata 5/1/-1 --fullscreen
```
Sets the board to fullscreen

__Default value for fullscreen:__ false