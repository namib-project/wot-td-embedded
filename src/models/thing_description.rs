use heapless::FnvIndexMap;
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
pub struct ThingDescription<
    'a,
    const ACTIONS: usize = 0,
    const PROPERTIES: usize = 0,
    const EVENTS: usize = 0,
> {
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
    pub properties: Option<FnvIndexMap<&'a str, Property<'a>, PROPERTIES>>,
    pub actions: Option<FnvIndexMap<&'a str, Action<'a>, ACTIONS>>,
    pub events: Option<FnvIndexMap<&'a str, Event<'a>, EVENTS>>,
    pub links: Option<ArrayEntry<'a, Link<'a>>>,
    // TODO: Add forms
    // TODO: Add security
    // TODO: Add securityDefinitions
    // TODO: Add profile
    // TODO: Add schemaDefinitions
    // TODO: Add uriVariables
}

#[derive(Default)]
pub struct ThingDescriptionBuilder<
    'a,
    const ACTIONS: usize = 0,
    const PROPERTIES: usize = 0,
    const EVENTS: usize = 0,
> {
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
    pub properties: Option<FnvIndexMap<&'a str, Property<'a>, PROPERTIES>>,
    pub actions: Option<FnvIndexMap<&'a str, Action<'a>, ACTIONS>>,
    pub events: Option<FnvIndexMap<&'a str, Event<'a>, EVENTS>>,
    pub links: Option<ArrayEntry<'a, Link<'a>>>,
    // TODO: Add forms
    // TODO: Add security
    // TODO: Add securityDefinitions
    // TODO: Add profile
    // TODO: Add schemaDefinitions
    // TODO: Add uriVariables
}

impl<'a, const ACTIONS: usize, const PROPERTIES: usize, const EVENTS: usize>
    ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES, EVENTS>
{
    pub fn new(title: &'a str) -> ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES> {
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

    pub fn base(
        mut self,
        base: &'a str,
    ) -> ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES, EVENTS> {
        self.base = Some(base);
        self
    }
    pub fn properties(
        mut self,
        properties: FnvIndexMap<&'a str, Property<'a>, PROPERTIES>,
    ) -> ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES, EVENTS> {
        self.properties = Some(properties);
        self
    }

    pub fn actions(
        mut self,
        actions: FnvIndexMap<&'a str, Action<'a>, ACTIONS>,
    ) -> ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES, EVENTS> {
        self.actions = Some(actions);
        self
    }

    pub fn events(
        mut self,
        events: FnvIndexMap<&'a str, Event<'a>, EVENTS>,
    ) -> ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES, EVENTS> {
        self.events = Some(events);
        self
    }

    pub fn links(
        mut self,
        links: ArrayEntry<'a, Link<'a>>,
    ) -> ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES, EVENTS> {
        self.links = Some(links);
        self
    }

    pub fn json_ld_type(
        mut self,
        json_ld_type: ArrayEntry<'a, &'a str>,
    ) -> ThingDescriptionBuilder<'a, ACTIONS, PROPERTIES, EVENTS> {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn build(self) -> ThingDescription<'a, ACTIONS, PROPERTIES, EVENTS> {
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
        data_structures::array::ArrayEntry,
        models::{
            action::Action,
            data_schema::{DataSchema, DataType},
            link::Link,
            property::Property,
            thing_description::ThingDescriptionBuilder,
        },
    };
    use heapless::FnvIndexMap;
    use serde_json_core::{heapless::String, ser::Error, to_string};

    #[test]
    fn serialize_thing_description() -> Result<(), Error> {
        // FIXME: For some reason the size needs to be 2 here, otherwise an overflow occurs.
        //        See https://github.com/japaric/heapless/issues/216
        let mut properties = FnvIndexMap::<&str, Property, 2>::new();

        let first_property = Property {
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
                data_type: Some(DataType::Boolean),
            },
        };

        properties.insert("status", first_property).unwrap();

        let mut actions = FnvIndexMap::<&str, Action, 2>::new();

        let first_action = Action {
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
        };

        let second_action = Action {
            input: None,
            output: None,
            safe: None,
            idempotent: None,
            synchronous: None,
        };
        actions.insert("toggle", first_action).unwrap();
        actions.insert("toggle2", second_action).unwrap();

        let links = ArrayEntry::new(Link {
            href: "https://example.org",
        });

        let thing_description = ThingDescriptionBuilder::<2, 2>::new("Test TD")
            .json_ld_type(ArrayEntry::new("saref:LightSwitch"))
            .properties(properties)
            .actions(actions)
            .links(links)
            .build();

        let expected_result = r#"{"@context":"https://www.w3.org/2022/wot/td/v1.1","@type":["saref:LightSwitch"],"title":"Test TD","properties":{"status":{"title":"Status","type":"boolean"}},"actions":{"toggle":{"input":{"title":"Toggle Data"}},"toggle2":{}},"links":[{"href":"https://example.org"}]}"#;
        let actual_result: String<300> = to_string(&thing_description)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
