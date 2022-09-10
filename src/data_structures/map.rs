use serde::{ser::SerializeMap, Serialize};

#[derive(Debug)]
pub struct MapEntry<'a, T>
where
    T: Serialize,
{
    key: &'a str,
    value: T,
    next: Option<&'a mut MapEntry<'a, T>>,
}

impl<'a, T: Serialize> MapEntry<'a, T> {
    pub fn new(key: &'a str, value: T) -> MapEntry<T> {
        MapEntry {
            key,
            value,
            next: None,
        }
    }

    fn get_last_entry(&mut self) -> &mut MapEntry<'a, T> {
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

    pub fn set_next(&mut self, next: &'a mut MapEntry<'a, T>) -> &MapEntry<'a, T> {
        self.next = Some(next);
        self
    }

    pub fn add_entry(&mut self, next: &'a mut MapEntry<'a, T>) -> &mut MapEntry<'a, T> {
        self.get_last_entry().set_next(next);
        self
    }
}

impl<'a, T: Serialize> Serialize for MapEntry<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;

        let mut current_entry = self;

        loop {
            map.serialize_entry(current_entry.key, &current_entry.value)?;

            if current_entry.next.is_none() {
                break;
            }

            current_entry = current_entry
                .next
                .as_ref()
                .expect("Expected a next element");
        }

        map.end()
    }
}
