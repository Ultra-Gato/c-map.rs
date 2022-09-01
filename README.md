This crate is inspired by the [dashmap](https://github.com/xacrimon/dashmap). which is a concurrent hashmap.

But [dashmap](https://github.com/xacrimon/dashmap) has some poor api design (In my opinion). 

Whenever It needs to do some operation, it has to recompute the same hash key. which is cheap but redundant, and unnecessary locking and unlocking also creates extra overhead. 

Moreover, It's very easy to get a deadlock while using a `dashmap`.

So this crate is designed to resolve those problems, Also performance should be better then `dashmap`, since no code is faster then no code. It has less then 150 lines of code...

# How does it work?

Instead of using a giant `RwLock` on a hashmap. It use multiple hashmaps for better performance. 
