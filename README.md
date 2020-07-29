# my-grep
Simple implementation of grep. Goals: learn more rust error management, try implementing parallelism.

The idea would be to implement some form of grepping and do it in parallel for each file using `Rayon`. 

## Notes:
Timing it hasn't shown a substantial increase in my case in speed from using more threads. 
