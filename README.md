This crate is inspired by [dashmap](https://github.com/xacrimon/dashmap). which is a concurrent hashmap.

But [dashmap](https://github.com/xacrimon/dashmap) has some poor api design (In my opinion). 

Whenever It need to do some operation, every time it compute the same hash key. which is cheap but redundant, and unnecessary locking and unlocking also has some extra overhead. 

Moreover, Its very easy to get deadlock in `dashmap`.

So this crate is to resolve those problems, Also performance should be better then `dashmap`, as no code is faster any less code. It has less then 150 lines of code...

# How it works ?

Instead of a giant rwlock on a hashmap. It use multiple hashmap for better performance. 
