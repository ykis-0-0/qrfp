use qrcodegen::QrCodeEcc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Serialize, Deserialize)]
pub enum QrCodeEccCompat {
  Low,
  Medium,
  Quartile,
  High
}

impl From<&QrCodeEccCompat> for QrCodeEcc {
  fn from(value: &QrCodeEccCompat) -> Self {
    use QrCodeEccCompat as QCEc;
    match value {
      QCEc::Low => Self::Low,
      QCEc::Medium => Self::Medium,
      QCEc::Quartile => Self::Quartile,
      QCEc::High => Self::High,
    }
  }
}

impl From<&QrCodeEcc> for QrCodeEccCompat {
  fn from(value: &QrCodeEcc) -> Self {
    use QrCodeEcc as QCE;
    match value {
      QCE::Low => Self::Low,
      QCE::Medium => Self::Medium,
      QCE::Quartile => Self::Quartile,
      QCE::High => Self::High,
    }
  }
}

impl From<QrCodeEccCompat> for QrCodeEcc {
  fn from(value: QrCodeEccCompat) -> Self {
    Self::from(&value)
  }
}

impl From<QrCodeEcc> for QrCodeEccCompat {
  fn from(value: QrCodeEcc) -> Self {
    Self::from(&value)
  }
}
