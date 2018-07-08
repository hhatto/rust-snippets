#![feature(test)]
extern crate test;
extern crate quick_xml;
extern crate xml;

use xml::reader::{EventReader, XmlEvent};
use quick_xml::reader::Reader;
use quick_xml::events::Event;

const XML_STRING: &'static str = r#"<?xml version='1.0' encoding='UTF-8'?>
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

fn _bench_xmlrs() {
    let reader = EventReader::from_str(XML_STRING);

    for e in reader {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                format!("tag-name: {:?} attr: {:?}",
                         name.local_name,
                         attributes.into_iter().map(|a| {
                             format!("{}={}", a.name.local_name, a.value)
                         }).collect::<Vec<_>>());
            },
            Ok(XmlEvent::EndElement { .. }) => {
                //println!("tag-name(e): {:?}", name);
            }
            Ok(XmlEvent::Characters(s)) => {
                let _ = format!("    text: {}", s);
            },
            Err(e) => panic!("error: {:?}", e),
            _ => (),
        }
    }
}

fn _bench_quickxml() {
    let mut buf = Vec::new();
    let mut reader = Reader::from_str(XML_STRING);
    reader.trim_text(true);

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                let _ = format!("tag-name: {:?} attr: {:?}", std::str::from_utf8(e.name()).expect("convert error"),
                        e.attributes().map(|a| {
                            let v = a.unwrap();
                            format!("{}={}",
                                    std::str::from_utf8(v.key).expect("convert error"),
                                    std::str::from_utf8(v.value).expect("convert error"))
                        }).collect::<Vec<_>>());
            },
            Ok(Event::Text(e)) => {
                let _ = format!("    text: {}", e.unescape_and_decode(&reader).unwrap());
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("error: {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_xmlrs(b: &mut Bencher) {
        b.iter(|| _bench_xmlrs());
    }

    #[bench]
    fn bench_quickxml(b: &mut Bencher) {
        b.iter(|| _bench_quickxml());
    }
}
