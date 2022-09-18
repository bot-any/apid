macro_rules! impl_serde_bool_const {
    ($val:ident, $unit_struct:ident) => {
        impl serde::Serialize for $unit_struct {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_bool($val)
            }
        }

        impl<'de> serde::Deserialize<'de> for $unit_struct {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                if bool::deserialize(deserializer)? == $val {
                    Ok($unit_struct)
                } else {
                    Err(serde::de::Error::custom(concat!(
                        "expected ",
                        stringify!($val)
                    )))
                }
            }
        }
    };
}

#[derive(Debug)]
pub struct True;
impl_serde_bool_const!(true, True);

#[derive(Debug)]
pub struct False;
impl_serde_bool_const!(false, False);
