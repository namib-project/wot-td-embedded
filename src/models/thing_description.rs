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
    pub properties: Option<Map<'a, Property<'a>>>,
    pub actions: Option<Map<'a, Action<'a>>>,
    pub events: Option<Map<'a, Event<'a>>>,
    pub links: Option<Array<'a, Link<'a>>>,
    pub forms: Option<Array<'a, Form<'a>>>,
    pub security: Option<Array<'a, &'a str>>,
    pub security_definitions: Option<Map<'a, SecurityScheme<'a>>>,
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
    pub properties: Option<Map<'a, Property<'a>>>,
    pub actions: Option<Map<'a, Action<'a>>>,
    pub events: Option<Map<'a, Event<'a>>>,
    pub links: Option<Array<'a, Link<'a>>>,
    pub forms: Option<Array<'a, Form<'a>>>,
    pub security: Option<Array<'a, &'a str>>,
    pub security_definitions: Option<Map<'a, SecurityScheme<'a>>>,
    pub profile: Option<Array<'a, &'a str>>,
    pub schema_definitions: Option<Map<'a, DataSchema<'a>>>,
    pub uri_variables: Option<Map<'a, DataSchema<'a>>>,
}

impl<'a> ThingDescriptionBuilder<'a> {
    fn default() -> Self {
        //let mut context_entry = Array::new_entry(ContextEntry::default());
        let context = Array::new(); //.add(&mut context_entry);

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
        // let mut context_entry = Array::new_entry(ContextEntry::default());
        let context = Array::new(); //.add(&mut context_entry);
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

    pub fn titles(mut self, titles: Map<'a, &'a str>) -> ThingDescriptionBuilder<'a> {
        self.titles = Some(titles);
        self
    }

    pub fn description(mut self, description: &'a str) -> ThingDescriptionBuilder<'a> {
        self.description = Some(description);
        self
    }

    pub fn descriptions(mut self, descriptions: Map<'a, &'a str>) -> ThingDescriptionBuilder<'a> {
        self.descriptions = Some(descriptions);
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

    pub fn created(mut self, created: &'a str) -> ThingDescriptionBuilder<'a> {
        self.created = Some(created);
        self
    }

    pub fn modified(mut self, modified: &'a str) -> ThingDescriptionBuilder<'a> {
        self.modified = Some(modified);
        self
    }

    pub fn properties(mut self, properties: Map<'a, Property<'a>>) -> ThingDescriptionBuilder<'a> {
        self.properties = Some(properties);
        self
    }

    pub fn actions(mut self, actions: Map<'a, Action<'a>>) -> ThingDescriptionBuilder<'a> {
        self.actions = Some(actions);
        self
    }

    pub fn events(mut self, events: Map<'a, Event<'a>>) -> ThingDescriptionBuilder<'a> {
        self.events = Some(events);
        self
    }

    pub fn forms(mut self, forms: Array<'a, Form<'a>>) -> ThingDescriptionBuilder<'a> {
        self.forms = Some(forms);
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
        security_definitions: Map<'a, SecurityScheme<'a>>,
    ) -> ThingDescriptionBuilder<'a> {
        self.security_definitions = Some(security_definitions);
        self
    }

    pub fn uri_variables(
        mut self,
        uri_variables: Map<'a, DataSchema<'a>>,
    ) -> ThingDescriptionBuilder<'a> {
        self.uri_variables = Some(uri_variables);
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
            event::Event,
            form::{Form, FormBuilder, OperationType},
            link::Link,
            property::Property,
            security_definition::{SecurityScheme, SecuritySchemeType},
            thing_description::{ContextEntry, ThingDescription},
            version_info::VersionInfo,
        },
    };
    use serde_json_core::{heapless::String, ser::Error, to_string};

    #[test]
    fn serialize_thing_description() -> Result<(), Error> {
        let mut action1_form =
            Array::new_entry(FormBuilder::new("coaps://example.org/toggle").build());
        let action1_forms = Array::new().add(&mut action1_form);

        let mut action2_form =
            Array::new_entry(FormBuilder::new("coaps://example.org/toggle2").build());
        let action2_forms = Array::new().add(&mut action2_form);

        let mut action1 = Map::new_entry(
            "toggle",
            ActionBuilder::new(action1_forms)
                .input(DataSchema::builder().title("Toggle Data").build())
                .build(),
        );
        let mut action2 = Map::new_entry("toggle2", ActionBuilder::new(action2_forms).build());

        let actions = Map::new().add(&mut action1).add(&mut action2);

        let mut link_entry = Array::new_entry(Link::new("https://example.org"));
        let links = Array::new().add(&mut link_entry);

        let mut json_ld_type_entry = Array::new_entry("saref:LightSwitch");
        let json_ld_type = Array::new().add(&mut json_ld_type_entry);

        const NO_SEC_KEY: &str = "nosec_sc";
        let mut security_entry = Array::new_entry(NO_SEC_KEY);
        let security = Array::new().add(&mut security_entry);

        let mut security_definitions = Map::new_entry(
            NO_SEC_KEY,
            SecurityScheme::builder(SecuritySchemeType::Nosec).build(),
        );
        let security_definitions = Map::new().add(&mut security_definitions);

        let mut property_form =
            Array::new_entry(FormBuilder::new("coaps://example.org/status").build());
        let mut property_forms = Array::new_entry(&mut property_form);
        let mut property = Map::new_entry(
            "status",
            Property::builder(
                Array::new(),
                DataSchema::builder()
                    .title("Status")
                    .data_type(DataType::Boolean)
                    .build(),
            )
            .build(),
        );
        let properties = Map::new().add(&mut property);

        let mut default_context = Array::new_entry(ContextEntry::default());

        let mut context_map_entry = Map::new_entry("cov", "http://www.example.org/coap-binding#");
        let mut context_map = Array::new_entry(ContextEntry::MapEntry(
            Map::new().add(&mut context_map_entry),
        ));

        let context = Array::new().add(&mut default_context).add(&mut context_map);

        let mut event_form =
            Array::new_entry(FormBuilder::new("coaps://example.org/overheating").build());
        let event_forms = Array::new().add(&mut event_form);
        let mut event = Map::new_entry(
            "overheating",
            Event::builder(event_forms)
                .data(
                    DataSchema::builder()
                        .data_type(DataType::Integer(None))
                        .build(),
                )
                .build(),
        );
        let events = Map::new().add(&mut event);

        let thing_description = ThingDescription::builder()
            .context(context)
            .title("Test TD")
            .description("Description for the Test TD")
            .json_ld_type(json_ld_type)
            .properties(properties)
            .actions(actions)
            .events(events)
            .links(links)
            .security(security)
            .security_definitions(security_definitions)
            .version(VersionInfo::new("v1.0.0"))
            .build();

        let expected_result = r#"{"@context":["https://www.w3.org/2022/wot/td/v1.1",{"cov":"http://www.example.org/coap-binding#"}],"@type":["saref:LightSwitch"],"title":"Test TD","description":"Description for the Test TD","version":{"instance":"v1.0.0"},"properties":{"status":{"forms":[{"href":"coaps://example.org/status"}],"title":"Status","type":"boolean"}},"actions":{"toggle":{"forms":[{"href":"coaps://example.org/toggle","op":["invokeaction"]}],"input":{"title":"Toggle Data"}},"toggle2":{"forms":[{"href":"coaps://example.org/toggle2"}]}},"events":{"overheating":{"forms":[{"href":"coaps://example.org/overheating"}],"data":{"type":"integer"}}},"links":[{"href":"https://example.org"}],"security":["nosec_sc"],"securityDefinitions":{"nosec_sc":{"scheme":"nosec"}}}"#;
        let actual_result: String<800> = to_string(&thing_description)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
