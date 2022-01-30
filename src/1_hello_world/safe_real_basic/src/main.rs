use rkyv::{Serialize, Deserialize, Archive};

use bytecheck::CheckBytes;

#[derive(Serialize, Deserialize, Archive)]
#[archive_attr(derive(CheckBytes))]          // <- derive validation from checkbytes

struct  Example {
    a: i32,
    b: f64,
    c: String,
}

fn main() {
    let ex1 = Example {
        a: 1,
        b: 3.14,
        c: String::from("Hello world"),
    };

    let bytes = rkyv::to_bytes::<_, 256>(&ex1).expect("some ser error");
    let bytes = bytes.as_ref();

    let d_ex1: Example = rkyv::from_bytes(bytes).expect("some deser error");      // so that we no longer have to do unsafe from_bytes_unchecked
    println!("d_ext1.c = {}", d_ex1.c);



}
