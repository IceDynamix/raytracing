use crate::math::Vector3;
use crate::ppm::Pixel;
use ababa_config::{AbabaParseError, AbabaValue};

impl TryFrom<AbabaValue> for Vector3 {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        let (x, y, z) = value.try_into()?;
        Ok(Vector3::new(x, y, z))
    }
}

impl TryFrom<AbabaValue> for Pixel {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        let (r, g, b) = value.try_into()?;
        Ok(Pixel::new(r, g, b))
    }
}
