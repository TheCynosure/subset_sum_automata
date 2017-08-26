# subset\_sum
This is the sub problem to the subset\_sum\_automata. The idea here is that given a list of numbers we can find if the set has any non-zero subset that sums to the value given. This program was on r/dailyprogrammer before subset\_sum\_automata but is needed in its solution. 

I solved the question in O(2^n n) time. But that is not that great. One day I will implement a better solution. There is a second function in main.rs called check\_subset. This function will check for a 2 length subset and solves that problem in O(n).
