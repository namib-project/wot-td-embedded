/*
 * Copyright (c) 2022 The NAMIB Project Developers.
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 *
 * SPDX-License-Identifier: MIT OR Apache-2.0
 */

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    constants::WOT_TD_11_CONTEXT,
    data_structures::{array::Array, map::Map},
};

use super::{
    action::Action, data_schema::DataSchema, event::Event, form::Form, link::Link,
    property::Property, security_definition::SecurityScheme, version_info::VersionInfo,
};

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ContextEntry<'a> {
    StringEntry(&'a str),
    MapEntry(Map<'a, &'a str>),
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
    pub context: Array<'a, ContextEntry<'a>>,
    #[serde(rename = "@type")]
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub id: Option<&'a str>,
    pub title: &'a str,
    pub titles: Option<Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<Map<'a, &'a str>>,
    pub version: Option<VersionInfo<'a>>,
    pub created: Option<&'a str>,
    pub modified: Option<&'a str>,
    pub support: Option<&'a str>,
    pub base: Option<&'a str>,
    pub properties: Option<&'a Map<'a, Property<'a>>>,
    pub actions: Option<&'a mut Map<'a, Action<'a>>>,
    pub events: Option<Map<'a, Event<'a>>>,
    pub links: Option<Array<'a, Link<'a>>>,
    pub forms: Option<Array<'a, Form<'a>>>,
    pub security: Option<Array<'a, &'a str>>,
    pub security_definitions: Option<&'a mut Map<'a, SecurityScheme<'a>>>,
    pub profile: Option<Array<'a, &'a str>>,
    pub schema_definitions: Option<Map<'a, DataSchema<'a>>>,
    pub uri_variables: Option<Map<'a, DataSchema<'a>>>,
}

impl<'a> ThingDescription<'a> {
    pub fn builder() -> ThingDescriptionBuilder<'a> {
        ThingDescriptionBuilder::default()
    }
}

#[derive(Debug)]
pub struct ThingDescriptionBuilder<'a> {
    pub context: Array<'a, ContextEntry<'a>>,
    pub json_ld_type: Option<Array<'a, &'a str>>,
    pub id: Option<&'a str>,
    pub title: &'a str,
    pub titles: Option<Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<Map<'a, &'a str>>,
    pub version: Option<VersionInfo<'a>>,
    pub created: Option<&'a str>,
    pub modified: Option<&'a str>,
    pub support: Option<&'a str>,
    pub base: Option<&'a str>,
    pub properties: Option<&'a Map<'a, Property<'a>>>,
    pub actions: Option<&'a mut Map<'a, Action<'a>>>,
    pub events: Option<Map<'a, Event<'a>>>,
    pub links: Option<Array<'a, Link<'a>>>,
    pub forms: Option<Array<'a, Form<'a>>>,
    pub security: Option<Array<'a, &'a str>>,
    pub security_definitions: Option<&'a mut Map<'a, SecurityScheme<'a>>>,
    pub profile: Option<Array<'a, &'a str>>,
    pub schema_definitions: Option<Map<'a, DataSchema<'a>>>,
    pub uri_variables: Option<Map<'a, DataSchema<'a>>>,
}

impl<'a> ThingDescriptionBuilder<'a> {
    fn default() -> Self {
        let context = Array::new(ContextEntry::default());

        Self {
            context,
            json_ld_type: Default::default(),
            id: Default::default(),
            title: Default::default(),
            titles: Default::default(),
            description: Default::default(),
            descriptions: Default::default(),
            version: Default::default(),
            created: Default::default(),
            modified: Default::default(),
            support: Default::default(),
            base: Default::default(),
            properties: Default::default(),
            actions: Default::default(),
            events: Default::default(),
            links: Default::default(),
            forms: Default::default(),
            security: Default::default(),
            security_definitions: Default::default(),
            profile: Default::default(),
            schema_definitions: Default::default(),
            uri_variables: Default::default(),
        }
    }
}

impl<'a> ThingDescriptionBuilder<'a> {
    pub fn new(title: &'a str) -> ThingDescriptionBuilder<'a> {
        let context = Array::new(ContextEntry::default());
        ThingDescriptionBuilder {
            title,
            context,
            ..ThingDescriptionBuilder::default()
        }
    }

    pub fn context(mut self, context: Array<'a, ContextEntry<'a>>) -> ThingDescriptionBuilder<'a> {
        self.context = context;
        self
    }

    pub fn title(mut self, title: &'a str) -> ThingDescriptionBuilder<'a> {
        self.title = title;
        self
    }

    pub fn base(mut self, base: &'a str) -> ThingDescriptionBuilder<'a> {
        self.base = Some(base);
        self
    }

    pub fn version(mut self, version: VersionInfo<'a>) -> ThingDescriptionBuilder<'a> {
        self.version = Some(version);
        self
    }

    pub fn properties(
        mut self,
        properties: &'a Map<'a, Property<'a>>,
    ) -> ThingDescriptionBuilder<'a> {
        self.properties = Some(properties);
        self
    }

    pub fn actions(mut self, actions: &'a mut Map<'a, Action<'a>>) -> ThingDescriptionBuilder<'a> {
        self.actions = Some(actions);
        self
    }

    pub fn events(mut self, events: Map<'a, Event<'a>>) -> ThingDescriptionBuilder<'a> {
        self.events = Some(events);
        self
    }

    pub fn links(mut self, links: Array<'a, Link<'a>>) -> ThingDescriptionBuilder<'a> {
        self.links = Some(links);
        self
    }

    pub fn json_ld_type(mut self, json_ld_type: Array<'a, &'a str>) -> ThingDescriptionBuilder<'a> {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn security(mut self, security: Array<'a, &'a str>) -> ThingDescriptionBuilder<'a> {
        self.security = Some(security);
        self
    }

    pub fn security_definitions(
        mut self,
        security_definitions: &'a mut Map<'a, SecurityScheme<'a>>,
    ) -> ThingDescriptionBuilder<'a> {
        self.security_definitions = Some(security_definitions);
        self
    }

    pub fn build(self) -> ThingDescription<'a> {
        ThingDescription {
            context: self.context,
            json_ld_type: self.json_ld_type,
            id: self.id,
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
            version: self.version,
            created: self.created,
            modified: self.modified,
            support: self.support,
            base: self.base,
            properties: self.properties,
            actions: self.actions,
            events: self.events,
            links: self.links,
            forms: self.forms,
            security: self.security,
            profile: self.profile,
            schema_definitions: self.schema_definitions,
            uri_variables: self.uri_variables,
            security_definitions: self.security_definitions,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        data_structures::{
            array::{Array, ArrayEntry},
            map::{Map, MapEntry},
        },
        models::{
            action::{Action, ActionBuilder},
            data_schema::{DataSchema, DataType},
            form::{Form, FormBuilder, OperationType},
            link::Link,
            property::{Property, PropertyBuilder},
            security_definition::{SecurityScheme, SecuritySchemeType},
            thing_description::{ContextEntry, ThingDescription},
        },
    };
    use serde_json_core::{heapless::String, ser::Error, to_string};

    #[test]
    fn serialize_thing_description() -> Result<(), Error> {
        let action_input = DataSchema {
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
        };

        let mut first_action = MapEntry::<Action>::new(
            "toggle",
            ActionBuilder::new(Array::<Form>::new(
                FormBuilder::new("coaps://example.org/toggle")
                    .op(Array::<OperationType>::new(OperationType::Invokeaction))
                    .build(),
            ))
            .input(&action_input)
            .build(),
        );

        let mut second_action = MapEntry::<Action>::new(
            "toggle2",
            ActionBuilder::new(Array::<Form>::new(
                FormBuilder::new("coaps://example.org/toggle2").build(),
            ))
            .build(),
        );

        let mut actions = Map::<Action>::new()
            .insert(&mut second_action)
            .insert(&mut first_action);

        let links = Array::new(Link::builder().href("https://example.org").build());
        let json_ld_type = Array::new("saref:LightSwitch");

        const NO_SEC_KEY: &str = "nosec_sc";
        let security = Array::new(NO_SEC_KEY);

        let mut no_security = MapEntry::new(
            NO_SEC_KEY,
            SecurityScheme {
                json_ld_type: None,
                description: None,
                descriptions: None,
                proxy: None,
                scheme: SecuritySchemeType::Nosec,
            },
        );
        let mut security_definitions = Map::<SecurityScheme>::new().insert(&mut no_security);

        let mut first_property = MapEntry::<Property>::new(
            "status",
            PropertyBuilder::new(
                Array::<Form>::new(FormBuilder::new("coaps://example.org/status").build()),
                DataSchema {
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
            )
            .build(),
        );

        let mut properties = Map::<Property>::new().insert(&mut first_property);

        let mut coap_context = MapEntry::new("cov", "http://www.example.org/coap-binding#");
        let context_entries = Map::<&str>::new().insert(&mut coap_context);
        let mut context_map = ArrayEntry::new(ContextEntry::MapEntry(context_entries));
        let context =
            Array::<ContextEntry>::new(ContextEntry::default()).add_entry(&mut context_map);

        let thing_description = ThingDescription::builder()
            .context(context)
            .title("Test TD")
            .json_ld_type(json_ld_type)
            .actions(&mut actions)
            .links(links)
            .security(security)
            .security_definitions(&mut security_definitions)
            .properties(&mut properties)
            .build();

        let expected_result = r#"{"@context":["https://www.w3.org/2022/wot/td/v1.1",{"cov":"http://www.example.org/coap-binding#"}],"@type":["saref:LightSwitch"],"title":"Test TD","properties":{"status":{"forms":[{"href":"coaps://example.org/status"}],"title":"Status","type":"boolean"}},"actions":{"toggle":{"forms":[{"href":"coaps://example.org/toggle","op":["invokeaction"]}],"input":{"title":"Toggle Data"}},"toggle2":{"forms":[{"href":"coaps://example.org/toggle2"}]}},"links":[{"href":"https://example.org"}],"security":["nosec_sc"],"securityDefinitions":{"nosec_sc":{"scheme":"nosec"}}}"#;
        let actual_result: String<600> = to_string(&thing_description)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
