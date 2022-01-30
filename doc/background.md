rkyv is the best serialaization-deserialization library for Rust. It provides for unparalleled performance gains with it's use of zero-copy-deserialization (aka ZCD) to enable different use cases. The motivation and the inner working of the rkyv are beautifully described in the "rkvy book" here (https://rkyv.org/). 

This book provides a practical approach to using rkyv in different settings; taking the user from basic to the advanced use cases. Obviously not all uses are covered; but enough to enable the committed user for common use cases. 


rkyv has fundamentally two APIs for 'deserialization: 

1. Traditional Deserialization 
This API lets you work with the same types; but at the cost of copy

2. Zero Copy Deserialization 
This API requires the use of archived variants of your types; but provides access and mutability with much more performance.

In this book, we provide techniques to use both as well as how to intermingle the two approaches at times.