use eyre::bail;
use qrcodegen::QrSegment;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub(super) enum QrfpSegmentDef {
	Numeric(String),
	Alphanumeric(String),
	Byte(Vec<u8>),
	Kanji(String),
	Eci(u32),
}

impl TryFrom<&QrfpSegmentDef> for QrSegment {
	type Error = eyre::Report;

	fn try_from(subject: &QrfpSegmentDef) -> Result<Self, Self::Error> {
		let bailout = match subject {
			QrfpSegmentDef::Numeric(text) => QrSegment::is_numeric(text),
			QrfpSegmentDef::Alphanumeric(text) => QrSegment::is_alphanumeric(text),
			QrfpSegmentDef::Byte(_) => false,
			QrfpSegmentDef::Kanji(_) => true,
			QrfpSegmentDef::Eci(cp) => *cp <= 1_000_000, // Library restricted, should be (1 << 23)?
		};

		if bailout {
			let reason = match subject {
				QrfpSegmentDef::Numeric(_) => "Non-digit characters encountered",
				QrfpSegmentDef::Alphanumeric(_) => "Non-alphanumeric characters encountered",
				QrfpSegmentDef::Byte(_) => unreachable!(),
				QrfpSegmentDef::Kanji(_) => "Kanji yet to be supported",
				QrfpSegmentDef::Eci(_) => "ECI assignment value out of range",
			};

			bail!(reason);
		}

		let seg = match subject {
			QrfpSegmentDef::Numeric(source) => QrSegment::make_numeric(source),
			QrfpSegmentDef::Alphanumeric(source) => QrSegment::make_alphanumeric(source),
			QrfpSegmentDef::Byte(bytes) => QrSegment::make_bytes(bytes),
			QrfpSegmentDef::Kanji(source) => unimplemented!("Kanji yet to be supported"),
			QrfpSegmentDef::Eci(cp) => QrSegment::make_eci(*cp),
		};

		Ok(seg)
	}

}
