use qrcodegen::{QrCodeEcc, QrSegment};
use eyre::{eyre, Result as eResult};
use serde::{Serialize, Deserialize};

use super::segment::QrfpSegmentDef;
pub(self) use super::serde_compat::QrCodeEccCompat;

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct QrfpSnapshotUnchecked {
  pub version: Option<u8>,

  // #[serde(with = "option_serde")]
  pub ec_level: Option<QrCodeEccCompat>,
  pub mask: Option<u8>,

  pub blocks: Vec<QrfpSegmentDef>
}

impl QrfpSnapshotUnchecked {
  pub fn try_version(&self) -> eResult<Option<qrcodegen::Version>> {
    use qrcodegen::Version;
    let valid = Version::MIN.value() ..= Version::MAX.value();

    match self.version {
      None => Ok(None),
      Some(v) if !valid.contains(&v) => Err(eyre!("Invalid Version")),
      Some(v) => Ok(Some(Version::new(v)))
    }
  }

  pub fn get_eclevel(&self) -> Option<QrCodeEcc> {
    self.ec_level.map(<_ as Into<QrCodeEcc>>::into)
  }

  pub fn try_mask(&self) -> eResult<Option<qrcodegen::Mask>>  {
    use qrcodegen::Mask;
    let valid = 0 ..= 7;

    match self.mask {
      None => Ok(None),
      Some(v) if !valid.contains(&v) => Err(eyre!("Invalid Version")),
      Some(v) => Ok(Some(Mask::new(v)))
    }
  }

  pub fn try_blocks(&self) -> eResult<Vec<QrSegment>> {
    self
    .blocks
    .iter()
    .map(|def| def.try_into())
    .collect::<Result<Vec<_>, _>>()
  }
}
