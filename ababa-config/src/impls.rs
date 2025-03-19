use crate::{AbabaParseError, AbabaValue};

impl TryFrom<AbabaValue> for f64 {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        match value {
            AbabaValue::Number(value) => Ok(value),
            _ => Err(AbabaParseError::ValueTypeDidNotMatch {
                expected: "Number",
                got: value,
            }),
        }
    }
}

macro_rules! impl_from_number {
    ($t:ty) => {
        impl TryFrom<AbabaValue> for $t {
            type Error = AbabaParseError;

            fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
                let x = f64::try_from(value)?;
                if x > <$t>::MAX as f64 || x < <$t>::MIN as f64 {
                    Err(AbabaParseError::NumberOutOfBounds {
                        x,
                        target_type: stringify!($t),
                    })
                } else {
                    Ok(x as $t)
                }
            }
        }
    };
}

impl_from_number!(i64);
impl_from_number!(u64);
impl_from_number!(i32);
impl_from_number!(u32);
impl_from_number!(i16);
impl_from_number!(u16);
impl_from_number!(i8);
impl_from_number!(u8);
impl_from_number!(usize);
impl_from_number!(isize);

impl<T: TryFrom<AbabaValue, Error = AbabaParseError>> TryFrom<AbabaValue> for Vec<T> {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        match value {
            // Result implements FromIterator, the iterator will stop on the first Err
            // and return it, otherwise it will contain the collected structure (in this case Vec<T>)
            AbabaValue::List(v) => Ok(v.into_iter().map(T::try_from).collect::<Result<_, _>>()?),
            _ => Err(AbabaParseError::ValueTypeDidNotMatch {
                expected: "List",
                got: value,
            }),
        }
    }
}

impl<T: TryFrom<AbabaValue, Error = AbabaParseError>> TryFrom<AbabaValue> for (T, T) {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        match value {
            AbabaValue::Tuple(v) => {
                if v.len() < 2 {
                    Err(AbabaParseError::NotEnoughElements {
                        expected: 2,
                        got: v.len(),
                    })
                } else {
                    Ok((v[0].to_owned().try_into()?, v[1].to_owned().try_into()?))
                }
            }
            _ => Err(AbabaParseError::ValueTypeDidNotMatch {
                expected: "Tuple",
                got: value,
            }),
        }
    }
}

impl<T: TryFrom<AbabaValue, Error = AbabaParseError>> TryFrom<AbabaValue> for (T, T, T) {
    type Error = AbabaParseError;

    fn try_from(value: AbabaValue) -> Result<Self, Self::Error> {
        match value {
            AbabaValue::Tuple(v) => {
                if v.len() < 3 {
                    Err(AbabaParseError::NotEnoughElements {
                        expected: 3,
                        got: v.len(),
                    })
                } else {
                    Ok((
                        v[0].to_owned().try_into()?,
                        v[1].to_owned().try_into()?,
                        v[2].to_owned().try_into()?,
                    ))
                }
            }
            _ => Err(AbabaParseError::ValueTypeDidNotMatch {
                expected: "Tuple",
                got: value,
            }),
        }
    }
}
