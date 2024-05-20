#![cfg(test)]

#[allow(unused_imports)]
use super::*;

use self::serde_compat::QrCodeEccCompat;
use checks::segment::QrfpSegmentDef as Q_SegDef;

#[test]
fn test_serialize() {
  use ron::ser::PrettyConfig;

  let snap = QrfpSnapshotUnchecked {
    version: Some(10),
    ec_level: Some(QrCodeEccCompat::Quartile),
    mask: None,
    blocks: vec![Q_SegDef::Alphanumeric("HELLO WORLD".to_string())],
  };

  let ropts = super::get_ron_spec();

  let strout = ropts
    .to_string_pretty(&snap, PrettyConfig::new())
    .expect("Serialization Failed")
  ;

  println!("{}", strout);
}

#[test]
fn test_deserialize() {
  let snap = {
    r#"(
        version: 10,
        ec_level: High,
        mask: 4,
        blocks: [
          Numeric("123123123"),
          Alphanumeric("WHAT THE HELL")
        ]
      )"#
  };

  let ropts = super::get_ron_spec();

  let result = ropts.from_str::<QrfpSnapshotUnchecked>(snap);

  println!("{:?}", result);
}
