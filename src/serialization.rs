#[derive(Debug)]
pub struct SerializationError;

pub trait ToJson {
    fn to_json(self, buf: &mut [u8]) -> Result<usize, SerializationError>;
}

pub(crate) trait JsonKey {
    fn to_json_key(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>;
}

pub(crate) trait JsonValue {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>;
}

pub(crate) trait JsonString {
    fn to_json_string(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>;
}

impl JsonValue for &str {
    fn to_json_value(&self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        let mut new_index = "\"".to_json_string(buf, index)?;

        new_index = self.to_json_string(buf, new_index)?;

        new_index = "\"".to_json_string(buf, new_index)?;

        Ok(new_index)
    }
}

impl JsonKey for &str {
    fn to_json_key(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError>
    where
        Self: Sized,
    {
        let mut new_index = index;

        new_index = self.to_json_value(buf, new_index)?;

        new_index = ":".to_json_string(buf, new_index)?;

        Ok(new_index)
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
        let mut new_index = index;

        if has_previous {
            new_index = ",".to_json_string(buf, index)?;
        }

        new_index = key.to_json_key(buf, new_index)?;
        new_index = self.to_json_value(buf, new_index)?;

        Ok(new_index)
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
        let mut new_index = index;

        if let Some(value) = self {
            if has_previous {
                new_index = ",".to_json_string(buf, index)?;
            }

            new_index = key.to_json_key(buf, new_index)?;
            new_index = value.to_json_value(buf, new_index)?;
        }

        Ok(new_index)
    }
}

impl JsonString for &str {
    fn to_json_string(self, buf: &mut [u8], index: usize) -> Result<usize, SerializationError> {
        if buf.len() - index - self.len() <= 0 {
            return Err(SerializationError {});
        }

        let mut new_index = index;

        for code_unit in self.as_bytes().into_iter() {
            buf[new_index] = *code_unit;
            new_index = new_index + 1;
        }

        Ok(new_index)
    }
}
