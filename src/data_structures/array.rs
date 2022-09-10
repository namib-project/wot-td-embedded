use serde::{ser::SerializeSeq, Serialize};

#[derive(Debug)]
pub struct ArrayEntry<'a, T: Serialize>
where
    T: Serialize,
{
    value: T,
    next: Option<&'a mut ArrayEntry<'a, T>>,
}

impl<'a, T: Serialize> ArrayEntry<'a, T> {
    pub fn new(value: T) -> ArrayEntry<'a, T> {
        ArrayEntry { value, next: None }
    }

    fn get_last_entry(&mut self) -> &mut ArrayEntry<'a, T> {
        let mut current_entry = self;

        loop {
            if current_entry.next.is_none() {
                break;
            }

            current_entry = current_entry
                .next
                .as_mut()
                .expect("Expected a next element");
        }

        current_entry
    }

    pub fn set_next(&mut self, next: &'a mut ArrayEntry<'a, T>) -> &ArrayEntry<'a, T> {
        self.next = Some(next);
        self
    }

    pub fn add_entry(&mut self, next: &'a mut ArrayEntry<'a, T>) -> &mut ArrayEntry<'a, T> {
        self.get_last_entry().set_next(next);
        self
    }
}

impl<'a, T: Serialize> Serialize for ArrayEntry<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // TODO: Allow single values for certain data types
        //
        // if self.next.is_none() {
        //     return serializer.serialize_some(&self.value);
        // }

        let mut sequence = serializer.serialize_seq(None)?;

        let mut current_entry = self;

        loop {
            sequence.serialize_element(&current_entry.value)?;

            if current_entry.next.is_none() {
                break;
            }

            current_entry = current_entry
                .next
                .as_ref()
                .expect("Expected a next element");
        }

        sequence.end()
    }
}
