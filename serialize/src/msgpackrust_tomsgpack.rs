extern crate rmp;

trait M {
    fn to_msgpack(&self) -> rmp::Value;
}

impl M for i32 {
    fn to_msgpack(&self) -> rmp::Value {
        rmp::Value::from(*self)
    }
}

#[derive(Debug)]
struct MyStruct {
    id: i32,
    name: String,
}

impl M for MyStruct {
    fn to_msgpack(&self) -> rmp::Value {
        let id = self.id;
        let name = self.name.to_string();
        rmp::Value::Array(vec![rmp::Value::from(id), rmp::Value::String(name.to_owned())])
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
