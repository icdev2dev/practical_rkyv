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
