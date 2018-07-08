extern crate rmpv;

trait M {
    fn to_msgpack(&self) -> rmpv::Value;
}

impl M for i32 {
    fn to_msgpack(&self) -> rmpv::Value {
        rmpv::Value::from(*self)
    }
}

#[derive(Debug)]
struct MyStruct {
    id: i32,
    name: String,
}

impl M for MyStruct {
    fn to_msgpack(&self) -> rmpv::Value {
        let id = self.id;
        let name = self.name.to_string();
        rmpv::Value::Array(vec![rmpv::Value::from(id),
                                rmpv::Value::String(name.into())])
    }
}

fn main() {
    let v: i32 = 100;
    println!("{:?}: {:?}", v, v.to_msgpack());
    let s = MyStruct {
        id: 22,
        name: "hoge".to_string(),
    };
    println!("{:?}: {:?}", s, s.to_msgpack());
}
