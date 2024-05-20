use qrcodegen::{Mask, QrCode, QrCodeEcc, QrSegment, Version};
use eyre::Result as eResult;

mod segment;
mod serde_compat;
mod unchecked;

mod checks;

use unchecked::QrfpSnapshotUnchecked;

struct QrfpSnapshot {
  version: Option<Version>,
  ec_level: Option<QrCodeEcc>,
  mask: Option<Mask>,
  segments: Vec<QrSegment>
}

impl TryFrom<QrfpSnapshotUnchecked> for QrfpSnapshot {
  type Error = eyre::Report;

  fn try_from(source: QrfpSnapshotUnchecked) -> Result<Self, Self::Error> /* = eResult<Self> */ {
    let version = source.try_version()?;
    let ec_level = source.get_eclevel();
    let mask = source.try_mask()?;
    let segments = source.try_blocks()?;

    Ok(QrfpSnapshot{
      version,
      ec_level,
      mask,
      segments
    })
  }
}

impl QrfpSnapshot {
  pub fn make(&self) -> eResult<QrCode> {
    use qrcodegen::Version;

    let ecl = self.ec_level.unwrap_or(QrCodeEcc::Quartile);
    let (vmax, vmin) = Option::zip(self.version, self.version).unwrap_or((Version::MIN, Version::MAX));

    let result = QrCode::encode_segments_advanced(
      &self.segments, ecl,
      vmin, vmax,
      self.mask,
      false
    );

    Ok(result?)
  }
}

fn get_ron_spec() -> ron::Options {
  use ron::{Options, extensions::Extensions};

  macro_rules! extlist {
      ( $( $e:ident )* ) => { $(Extensions::$e) |* };
  }

  let exts = extlist!{
    UNWRAP_NEWTYPES
    IMPLICIT_SOME
    UNWRAP_VARIANT_NEWTYPES
  };

  Options::default()
  .with_default_extension(exts)
}
