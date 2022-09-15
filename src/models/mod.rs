pub mod action;
pub mod data_schema;
pub mod event;
pub mod form;
pub mod link;
pub mod property;
pub mod security_definition;
pub mod thing_description;

#[macro_export]
macro_rules! serialize_field {
    ($key:expr, $field:expr, $map:expr) => {
        if $field.is_some() {
            $map.serialize_entry($key, &$field)?;
        }
    };
}
