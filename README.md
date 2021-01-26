# rustyfun

Starting up my Rusty journey once again! This repo is for anything rust related that I am currently learning.

## Iteration


>In Rust, we can’t directly use map/filter/etc over vectors, we need to follow these steps:
>
>1. Convert the vector into an iterator using iter, into_iter or iter_mut methods
>2. Chain adapters such as map/filter/etc on the iterator
>3. Finally convert the iterator back to a vector using consumers such as collect, find, sum etc
