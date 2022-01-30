# practical_rkyv

Table of Contents

## [Background](./doc/background.md)

## 1. Hello world

### Traditional Deserialization
- [Most Basic Case](./doc/hello_world/real_basic.md)
- [Safe case](./doc/hello_world/safe_real_basic.md)
### Zero Copy Deserialization
- [Max Performance](./doc/hello_world/zcd_max_perf.md)
- [Safe ZCD](./doc/hello_world/zcd_safe.md)

## 2. Serializing struct of structs
    - Transparent Types
    - Opaque Types

## 3. Serializing through Serializers
    - Base Use Case 
    - Using CompositeSerializer
        - Writer
        - Scratch Space
        - Shared Pointer Map

## 4. Alternate Writers
    - File, 
    - ByteBuffer
    - ...

## 5. Extending Serializers

## 6. Deserializing Subobjects 
    - Harmonical use of zero-copy-deserialization with traditional deserialization

## 7. Wrapper Types 
    - When to use 
    - How to use them effectively
        - Customizable serialization
        - Non-default behavior
        - Transparent Foreign Type Support

## 8. Archived vs original types and impl duplication

## 9. Streaming in rkyv  
    - Fixed-size types 
    - Two-file solution 
    - Manual framing

## 10. Validation 
    - bytecheck 
    - object subtrees 
    - the default validator
    - adding validation capabilities

## 11. Basic Schema evolution
    -  Enums of boxed data

## 12. Advanced Schema Evolution 
    - No comprehensive proto solution yet

## 13. Copy optimization and nightly features

## 14. Derive macro features: 
    - archive_attr 
    - omit_bounds/archive(bound(...)) 
    - archive(compare(PartialEq, PartialOrd))
