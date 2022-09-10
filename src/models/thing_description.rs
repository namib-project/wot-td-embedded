use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::data_structures::{array::ArrayEntry, map::MapEntry};

use super::{action::Action, event::Event, link::Link, property::Property};

pub const WOT_TD_11_CONTEXT: &str = "https://www.w3.org/2022/wot/td/v1.1";

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ContextEntry<'a> {
    StringEntry(&'a str),
    MapEntry(&'a str, &'a str),
}

impl<'a> Default for ContextEntry<'a> {
    fn default() -> Self {
        ContextEntry::StringEntry(WOT_TD_11_CONTEXT)
    }
}

#[skip_serializing_none]
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThingDescription<'a> {
    #[serde(rename = "@context")]
    pub context: ContextEntry<'a>,
    #[serde(rename = "@type")]
    pub json_ld_type: Option<ArrayEntry<'a, &'a str>>,
    pub id: Option<&'a str>,
    pub title: &'a str,
    // TODO: Add titles
    pub description: Option<&'a str>,
    // TODO: Add descriptions
    // TODO: Add version
    pub created: Option<&'a str>,
    pub modified: Option<&'a str>,
    pub support: Option<&'a str>,
    pub base: Option<&'a str>,
    pub properties: Option<MapEntry<'a, Property<'a>>>,
    pub actions: Option<MapEntry<'a, Action<'a>>>,
    pub events: Option<MapEntry<'a, Event<'a>>>,
    pub links: Option<ArrayEntry<'a, Link<'a>>>,
    // TODO: Add forms
    // TODO: Add security
    // TODO: Add securityDefinitions
    // TODO: Add profile
    // TODO: Add schemaDefinitions
    // TODO: Add uriVariables
}

#[derive(Default)]
pub struct ThingDescriptionBuilder<'a> {
    pub context: ContextEntry<'a>,
    pub json_ld_type: Option<ArrayEntry<'a, &'a str>>,
    pub id: Option<&'a str>,
    pub title: &'a str,
    pub titles: Option<MapEntry<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<MapEntry<'a, &'a str>>,
    // TODO: Add version
    pub created: Option<&'a str>,
    pub modified: Option<&'a str>,
    pub support: Option<&'a str>,
    pub base: Option<&'a str>,
    pub properties: Option<MapEntry<'a, Property<'a>>>,
    pub actions: Option<MapEntry<'a, Action<'a>>>,
    pub events: Option<MapEntry<'a, Event<'a>>>,
    pub links: Option<ArrayEntry<'a, Link<'a>>>,
    // TODO: Add forms
    // TODO: Add security
    // TODO: Add securityDefinitions
    // TODO: Add profile
    // TODO: Add schemaDefinitions
    // TODO: Add uriVariables
}

impl<'a> ThingDescriptionBuilder<'a> {
    pub fn new(title: &'a str) -> ThingDescriptionBuilder {
        ThingDescriptionBuilder {
            title,
            titles: None,
            context: ContextEntry::default(),
            json_ld_type: None,
            id: None,
            description: None,
            descriptions: None,
            created: None,
            modified: None,
            support: None,
            base: None,
            properties: None,
            actions: None,
            events: None,
            links: None,
        }
    }

    pub fn base(mut self, base: &'a str) -> ThingDescriptionBuilder {
        self.base = Some(base);
        self
    }
    pub fn properties(
        mut self,
        properties: MapEntry<'a, Property<'a>>,
    ) -> ThingDescriptionBuilder<'a> {
        self.properties = Some(properties);
        self
    }

    pub fn actions(mut self, actions: MapEntry<'a, Action<'a>>) -> ThingDescriptionBuilder<'a> {
        self.actions = Some(actions);
        self
    }

    pub fn links(mut self, links: ArrayEntry<'a, Link<'a>>) -> ThingDescriptionBuilder<'a> {
        self.links = Some(links);
        self
    }

    pub fn json_ld_type(
        mut self,
        json_ld_type: ArrayEntry<'a, &'a str>,
    ) -> ThingDescriptionBuilder<'a> {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn build(self) -> ThingDescription<'a> {
        ThingDescription {
            context: self.context,
            json_ld_type: self.json_ld_type,
            id: self.id,
            title: self.title,
            description: self.description,
            created: self.created,
            modified: self.modified,
            support: self.support,
            base: self.base,
            properties: self.properties,
            actions: self.actions,
            events: self.events,
            links: self.links,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        data_structures::{array::ArrayEntry, map::MapEntry},
        models::{
            action::Action, data_schema::DataSchema, link::Link, property::Property,
            thing_description::ThingDescriptionBuilder,
        },
    };
    use serde_json_core::{heapless::String, ser::Error, to_string};

    #[test]
    fn serialize_thing_description() -> Result<(), Error> {
        let mut actions = MapEntry::new(
            "toggle",
            Action {
                input: Some(DataSchema {
                    json_ld_type: None,
                    title: Some("Toggle Data"),
                    titles: None,
                    description: None,
                    descriptions: None,
                    constant: None,
                    default: None,
                    unit: None,
                    one_of: None,
                    enumeration: None,
                    read_only: None,
                    write_only: None,
                    format: None,
                    data_type: None,
                }),
                output: None,
                safe: None,
                idempotent: None,
                synchronous: None,
            },
        );
        let mut second_action = MapEntry::new(
            "toggle2",
            Action {
                input: None,
                output: None,
                safe: None,
                idempotent: None,
                synchronous: None,
            },
        );
        actions.add_entry(&mut second_action);

        let properties = MapEntry::new(
            "status",
            Property {
                observable: None,
                data_schema: DataSchema {
                    json_ld_type: None,
                    title: Some("Status"),
                    titles: None,
                    description: None,
                    descriptions: None,
                    constant: None,
                    default: None,
                    unit: None,
                    one_of: None,
                    enumeration: None,
                    read_only: None,
                    write_only: None,
                    format: None,
                    data_type: None,
                },
            },
        );

        let links = ArrayEntry::new(Link {
            href: "https://example.org",
        });

        let thing_description = ThingDescriptionBuilder::new("Test TD")
            .json_ld_type(ArrayEntry::new("saref:LightSwitch"))
            .properties(properties)
            .actions(actions)
            .links(links)
            .build();

        let expected_result = r#"{"@context":"https://www.w3.org/2022/wot/td/v1.1","@type":["saref:LightSwitch"],"title":"Test TD","properties":{"status":{"title":"Status"}},"actions":{"toggle":{"input":{"title":"Toggle Data"}},"toggle2":{}},"links":[{"href":"https://example.org"}]}"#;
        let actual_result: String<300> = to_string(&thing_description)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
