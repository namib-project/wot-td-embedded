use heapless::{FnvIndexMap, Vec};
use serde::Serialize;
use serde_with::skip_serializing_none;

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
    const TITLES: usize = 0,
    const DESCRIPTIONS: usize = 0,
    const JSON_LD_TYPE: usize = 0,
    const LINKS: usize = 0,
> {
    #[serde(rename = "@context")]
    pub context: ContextEntry<'a>,
    #[serde(rename = "@type")]
    pub json_ld_type: Option<Vec<&'a str, JSON_LD_TYPE>>,
    pub id: Option<&'a str>,
    pub title: &'a str,
    pub titles: Option<FnvIndexMap<&'a str, &'a str, TITLES>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<FnvIndexMap<&'a str, &'a str, DESCRIPTIONS>>,
    // TODO: Add version
    pub created: Option<&'a str>,
    pub modified: Option<&'a str>,
    pub support: Option<&'a str>,
    pub base: Option<&'a str>,
    pub properties: Option<FnvIndexMap<&'a str, Property<'a>, PROPERTIES>>,
    pub actions: Option<FnvIndexMap<&'a str, Action<'a>, ACTIONS>>,
    pub events: Option<FnvIndexMap<&'a str, Event<'a>, EVENTS>>,
    pub links: Option<Vec<Link<'a>, LINKS>>,
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
    const TITLES: usize = 0,
    const DESCRIPTIONS: usize = 0,
    const JSON_LD_TYPE: usize = 0,
    const LINKS: usize = 0,
> {
    pub context: ContextEntry<'a>,
    pub json_ld_type: Option<Vec<&'a str, JSON_LD_TYPE>>,
    pub id: Option<&'a str>,
    pub title: &'a str,
    pub titles: Option<FnvIndexMap<&'a str, &'a str, TITLES>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<FnvIndexMap<&'a str, &'a str, DESCRIPTIONS>>,
    // TODO: Add version
    pub created: Option<&'a str>,
    pub modified: Option<&'a str>,
    pub support: Option<&'a str>,
    pub base: Option<&'a str>,
    pub properties: Option<FnvIndexMap<&'a str, Property<'a>, PROPERTIES>>,
    pub actions: Option<FnvIndexMap<&'a str, Action<'a>, ACTIONS>>,
    pub events: Option<FnvIndexMap<&'a str, Event<'a>, EVENTS>>,
    pub links: Option<Vec<Link<'a>, LINKS>>,
    // TODO: Add forms
    // TODO: Add security
    // TODO: Add securityDefinitions
    // TODO: Add profile
    // TODO: Add schemaDefinitions
    // TODO: Add uriVariables
}

impl<
        'a,
        const ACTIONS: usize,
        const PROPERTIES: usize,
        const EVENTS: usize,
        const TITLES: usize,
        const DESCRIPTIONS: usize,
        const JSON_LD_TYPE: usize,
        const LINKS: usize,
    >
    ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    >
{
    pub fn new(
        title: &'a str,
    ) -> ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    > {
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
    ) -> ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    > {
        self.base = Some(base);
        self
    }
    pub fn properties(
        mut self,
        properties: FnvIndexMap<&'a str, Property<'a>, PROPERTIES>,
    ) -> ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    > {
        self.properties = Some(properties);
        self
    }

    pub fn actions(
        mut self,
        actions: FnvIndexMap<&'a str, Action<'a>, ACTIONS>,
    ) -> ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    > {
        self.actions = Some(actions);
        self
    }

    pub fn events(
        mut self,
        events: FnvIndexMap<&'a str, Event<'a>, EVENTS>,
    ) -> ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    > {
        self.events = Some(events);
        self
    }

    pub fn links(
        mut self,
        links: Vec<Link<'a>, LINKS>,
    ) -> ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    > {
        self.links = Some(links);
        self
    }

    pub fn json_ld_type(
        mut self,
        json_ld_type: Vec<&'a str, JSON_LD_TYPE>,
    ) -> ThingDescriptionBuilder<
        'a,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
    > {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn build(
        self,
    ) -> ThingDescription<'a, ACTIONS, PROPERTIES, EVENTS, TITLES, DESCRIPTIONS, JSON_LD_TYPE, LINKS>
    {
        ThingDescription {
            context: self.context,
            json_ld_type: self.json_ld_type,
            id: self.id,
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
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

    use crate::models::{
        action::Action,
        data_schema::{DataSchema, DataType},
        link::Link,
        property::Property,
        thing_description::ThingDescriptionBuilder,
    };
    use heapless::{FnvIndexMap, Vec};
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

        const LINKS_LENGTH: usize = 1;

        let mut links = Vec::<Link, LINKS_LENGTH>::new();
        links
            .push(Link {
                href: "https://example.org",
            })
            .unwrap();

        const JSON_LD_TYPE_LENGTH: usize = 2;

        let mut json_ld_type = Vec::<&str, JSON_LD_TYPE_LENGTH>::new();
        json_ld_type.push("saref:LightSwitch").unwrap();

        let thing_description =
            ThingDescriptionBuilder::<2, 2, 0, 0, 0, JSON_LD_TYPE_LENGTH, LINKS_LENGTH>::new(
                "Test TD",
            )
            .json_ld_type(json_ld_type)
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
