use crate::math::Vector3;
use crate::ppm::Pixel;
use ababa_config::{AbabaParseError, AbabaValue};

impl TryFrom<AbabaValue> for Vector3 {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        match value {
            AbabaValue::Tuple(t) => {
                if t.len() < 3 {
                    Err(AbabaParseError::NotEnoughElements {
                        expected: 3,
                        got: t.len(),
                    })
                } else {
                    let x = t[0].clone().try_into()?;
                    let y = t[1].clone().try_into()?;
                    let z = t[2].clone().try_into()?;
                    Ok(Vector3::new(x, y, z))
                }
            }
            _ => Err(AbabaParseError::ValueTypeDidNotMatch {
                expected: "String",
                got: value,
            }),
        }
    }
}

impl TryFrom<AbabaValue> for Pixel {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        match value {
            AbabaValue::Tuple(t) => {
                if t.len() < 3 {
                    Err(AbabaParseError::NotEnoughElements {
                        expected: 3,
                        got: t.len(),
                    })
                } else {
                    let x = t[0].clone().try_into()?;
                    let y = t[1].clone().try_into()?;
                    let z = t[2].clone().try_into()?;
                    Ok(Pixel::new(x, y, z))
                }
            }
            _ => Err(AbabaParseError::ValueTypeDidNotMatch {
                expected: "String",
                got: value,
            }),
        }
    }
}
