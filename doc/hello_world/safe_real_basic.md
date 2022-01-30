From this section onwards, the reader is expected to be familiar with the overall flow of rkyv development and therefore only key elements will covered. All complete code is checked in into (../../src/1_hello_world/safe_real_basic).

Because rkyv is a serialization-deserialization protocol, it must take in incoming bytes at some point in the lifecycles of types. For some types, providing invalid values for that type results immediately in undefined behaviour. bytecheck (a sister crate)  provides a framework for performing byte-level validations so that rkyv can rely on these bytechecks apriori (and hence be safe). Additionally bytecheck provides implementations for most basic types. Further it provides a derive macro to implement validation for enums and custom structs. 

Obviously you must declare the bytecheck dependency in Cargo.toml. 

<code>
bytecheck="0.6"
</code>


Additionally you must enable the validation feature in Cargo.toml for safe features to appear in rkyv.

<code>
rkyv= {version= "0.7.10", features=["validation"]}
</code>

After this, in the code you would use the safe function from_bytes. 

<code>
    let d_ex1: Example = rkyv::from_bytes(bytes).expect("some deser error");      // so that we no longer have to do unsafe from_bytes_unchecked

</code>

But from_bytes (being safe) requires a CheckBytes trait to be implemented. 

This is done through 

<code>
#[archive_attr(derive(CheckBytes))]
</code>

The attribute archive_attr (along with archive, omit_bounds and with ) beccome available through derive(Archive)
