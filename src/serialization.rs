#[derive(Debug)]
pub struct SerializationError;

pub trait ToJson {
    fn to_json(&self, buf: &mut [u8]) -> Result<usize, SerializationError>;
}

pub(crate) trait JsonKey {
    fn to_json_key(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>;
}

pub(crate) trait JsonValue {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>;
}

pub(crate) trait NestedJsonValue {
    fn to_nested_json_value(
        &self,
        buf: &mut [u8],
        index: usize,
        has_previous: bool,
    ) -> Result<usize, SerializationError>;
}

pub(crate) trait JsonString {
    fn to_json_string(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>;
}

impl JsonValue for &str {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        let mut index = "\"".to_json_string(buf, index)?;

        index = self.to_json_string(buf, index)?;

        index = "\"".to_json_string(buf, index)?;

        Ok(index)
    }
}

impl JsonValue for i64 {
    fn to_json_value(&self, _buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        // TODO!

        Ok(index)
    }
}

impl JsonValue for f64 {
    fn to_json_value(&self, _buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        // TODO!

        Ok(index)
    }
}

impl JsonKey for &str {
    fn to_json_key(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>
    where
        Self: Sized,
    {
        let mut index = index;

        index = self.to_json_value(buf, index)?;

        index = ":".to_json_string(buf, index)?;

        Ok(index)
    }
}

pub(crate) trait SerializableField {
    fn serialize_field(
        &self,
        key: &str,
        buf: &mut [u8],
        index: usize,
        has_previous: bool,
    ) -> Result<usize, SerializationError>;
}

impl<T: JsonValue> SerializableField for T {
    fn serialize_field(
        &self,
        key: &str,
        buf: &mut [u8],
        index: usize,
        has_previous: bool,
    ) -> Result<usize, SerializationError> {
        let mut index = index;

        if has_previous {
            index = ",".to_json_string(buf, index)?;
        }

        index = key.to_json_key(buf, index)?;
        index = self.to_json_value(buf, index)?;

        Ok(index)
    }
}

impl<T: JsonValue> SerializableField for Option<T> {
    fn serialize_field(
        &self,
        key: &str,
        buf: &mut [u8],
        index: usize,
        has_previous: bool,
    ) -> Result<usize, SerializationError> {
        let mut index = index;

        if let Some(value) = self {
            if has_previous {
                index = ",".to_json_string(buf, index)?;
            }

            index = key.to_json_key(buf, index)?;
            index = value.to_json_value(buf, index)?;
        }

        Ok(index)
    }
}

impl JsonString for &str {
    fn to_json_string(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        if index + self.len() > buf.len() {
            return Err(SerializationError {});
        }

        let mut index = index;

        for code_unit in self.as_bytes().iter() {
            buf[index] = *code_unit;
            index += 1;
        }

        Ok(index)
    }
}

impl JsonValue for bool {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        if *self {
            "true".to_json_string(buf, index)
        } else {
            "false".to_json_string(buf, index)
        }
    }
}
