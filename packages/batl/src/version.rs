#![allow(clippy::module_name_repetitions)]

use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{self, Serialize};


#[derive(Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct Version0_2_2;

#[allow(clippy::missing_trait_methods)]
impl<'de> Deserialize<'de> for Version0_2_2 {
	#[inline]
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>
	{
		/// Visitor for Version 0.2.2
		struct VersionVisitor;

		impl<'de> Visitor<'de> for VersionVisitor {
			type Value = Version0_2_2;

			fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
				formatter.write_str("`0.2.2`")
			}

			fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
			where
				E: de::Error
			{
				match v {
					"0.2.2" => Ok(Version0_2_2),
					_ => Err(de::Error::invalid_value(de::Unexpected::Str(v), &"0.2.2"))
				}	
			}
		}

		deserializer.deserialize_str(VersionVisitor)
	}
}

impl Serialize for Version0_2_2 {
	#[inline]
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: ser::Serializer
	{
		serializer.serialize_str("0.2.2")	
	}
}
