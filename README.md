# practical_rkyv
How to use rkyv

Table of Contents

- [Background](./doc/background.md)
- [Hello world]
    - Most Basic Case
    - Safe case
    - Zero Copy Deserialization
        - Max Performance
        - Safe ZCD

- [Serializing struct of structs]
    - Transparent Types
    - Opaque Types

- [Serializing through Serializers](./hello world)
    - Base Use Case 
    - Using CompositeSerializer
        - Writer
        - Scratch Space
        - Shared Pointer Map

- [Alternate Writers]
    - File, 
    - ByteBuffer
    - ...


- [Extending Serialization]

- Deserializing Subobjects 
    - Harmonical use of zero-copy-deserialization with traditional deserialization

- Wrapper Types 
    - When to use 
    - How to use them effectively
        - Customizable serialization
        - Non-default behavior
        - Transparent Foreign Type Support

- Archived vs original types and impl duplication

- Streaming in rkyv  
    - Fixed-size types 
    - Two-file solution 
    - Manual framing

- Validation 
    - bytecheck 
    - object subtrees 
    - the default validator
    - adding validation capabilities

- Basic Schema evolution
    -  Enums of boxed data

- Advanced Schema Evolution 
    - No comprehensive proto solution yet

- Copy optimization and nightly features

- Derive macro features: 
    - archive_attr 
    - omit_bounds/archive(bound(...)) 
    - archive(compare(PartialEq, PartialOrd))
