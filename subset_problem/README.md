##Subset Sum Problem
Link to problem [here.](https://www.reddit.com/r/dailyprogrammer/comments/68oda5/20170501_challenge_313_easy_subset_sum/?sort=confidence)

The main.rs file in the src folder solves this problem. There are two functions inside this module. 
The __check\_subset__ function solves the first part of this problem in O(n) time. 
The second function __check\_no\_empty\_subset__ solves the bonus questions in O(n n<sup>2</sup>) time (Not so great). 
But hey if you compile it with optimizations than it runs in rust pretty quickly! And for an NP-Complete