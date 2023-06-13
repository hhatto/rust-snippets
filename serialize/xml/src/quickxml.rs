extern crate quick_xml;

use quick_xml::reader::Reader;
use quick_xml::events::Event;

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

    let mut buf = Vec::new();
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                println!("tag-name: {:?} attr: {:?}", std::str::from_utf8(e.name().as_ref()).expect("convert error"),
                        e.attributes().map(|a| {
                            let v = a.unwrap();
                            format!("{}={}",
                                    std::str::from_utf8(v.key.as_ref()).expect("convert error"),
                                    std::str::from_utf8(v.value.as_ref()).expect("convert error"))
                        }).collect::<Vec<_>>());
            },
            Ok(Event::Text(e)) => println!("    text: {}", e.unescape().unwrap()),
            Ok(Event::Eof) => break,
            Err(e) => panic!("error: {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}
