extern crate xml;

use xml::reader::{EventReader, XmlEvent};

fn main() {
let xml = r#"<?xml version='1.0' encoding='UTF-8'?>
<com.example_-Tag plugin="t1@1.23">
  <node class="com.example.node" plugin="t2@9.87">
    <Ids>
      <string>9</string>
    </Ids>
    <id>10</id>
    <descId>description</descId>
  </node>
  <actions>
    <com.example.action1 plugin="t1@1.23"/>
    <com.example.action2 plugin="t1@1.23">
      <startTime>1502953063279</startTime>
    </com.example.action2>
  </actions>
</com.example_-Tag>"#;

    let reader = EventReader::from_str(xml);

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                println!("tag-name: {:?} attr: {:?}",
                         name.local_name,
                         attributes.into_iter().map(|a| {
                             format!("{}={}", a.name.local_name, a.value)
                         }).collect::<Vec<_>>());
            },
            Ok(XmlEvent::EndElement { .. }) => {
                //println!("tag-name(e): {:?}", name);
            }
            Ok(XmlEvent::Characters(s)) => println!("    text: {}", s),
            Err(e) => panic!("error: {:?}", e),
            _ => (),
        }
    }
}
