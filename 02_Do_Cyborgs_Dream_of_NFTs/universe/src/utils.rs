use crate::species::SapienceScale;

pub fn sapience_value(level: &SapienceScale) -> u8 {
  match level {
    SapienceScale::None => 0,
    SapienceScale::Low => 1,
    SapienceScale::Medium => 2,
    SapienceScale::High => 3,
  }
}
