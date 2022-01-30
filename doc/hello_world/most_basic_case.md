 The finished code is checked into the (../../src/1_hello_world).

We take the quintessential case. Serialize an instance of a given sttuct. We will use this format through out the book. We will define what we want to serialize/deseralize as the Example struct and create an instance of that struct and then proceed to serialize/deserialize that instance. Obviously the examples will become more complex; but we will follow this basic format.


In this toy example, we are using the following struct and instance. 

<code>

struct Example {

    a:i32,
    b:f64,
    c:String
}

let ex1 = Example {a:1, b:3.14, c:String::from("Hello World")};

</code>

# Steps : 

## Create New Project
- cargo new **project_name**

## Add Dependency
- Add dependency of rkyv="0.7.10" into cargo.toml
## Code

<code>

// Bring rkyv into scope

use rkyv::{Serialize, Deserialize, Archive, AlignedVec};

// Annotate Example with derives

#[derive(Serialize, Deserialize, Archive)]

struct Example {

    a: i32,
    b: f64,
    c: String,
}


fn main() {

    // create an instance ex1 of Example

    let ex1 = Example {
        a: 1,
        b: 3.14,
        c: String::from("Hello world"),
    };

    // Serialize ex1 to bytes

    let bytes: AlignedVec = rkyv::to_bytes::<_, 256> (&ex1).expect("some serialization error");
    let bytes:&[u8] = bytes.as_ref();

    // Deserialize Example from bytes (unsafe)

    let d_ex1:Example = unsafe {rkyv::from_bytes_unchecked(bytes).expect("some deser error")};

    // print a value from the deserialized Example
    println!("d_ext.c = {}", d_ex1.c );

}
</code>


## Run
- cargo run
