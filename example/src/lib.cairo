trait NameTrait<T> {
    fn name(self: @T) -> ByteArray;
}

#[dojo::model]
#[derive(Drop, Debug)]
struct MyStruct {
    x: u8,
    y: u16,
    z: u32,
}

#[derive(Drop, Introspect)]
struct MyOtherStruct {
    x: u8,
    y: u8
}

fn main() {

    // dojo::model attribute test => replace member types by u128
    let s = MyStruct { x: 0xFFFF, y: 0xFFFFFF, z: 0xFFFFFFFFFF};
    println!("s: {:?}", s);

    // Introspect derive => generate name() method for a struct
    let x = MyOtherStruct {x: 1, y: 2};
    println!("struct name: {}", x.name());

    // bytearray_hash inline macro => compute string hash
    let hash = bytearray_hash!("hello world");
    println!("hash: {}", hash);
}
