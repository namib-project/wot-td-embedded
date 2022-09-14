use heapless::{FnvIndexMap, Vec};
use serde::Serialize;
use serde_with::skip_serializing_none;

use super::{
    action::Action, data_schema::DataSchema, event::Event, link::Link, property::Property,
    security_definition::SecurityScheme,
};

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
    const CONTEXT: usize = 2,
    const ACTIONS: usize = 0,
    const PROPERTIES: usize = 0,
    const EVENTS: usize = 0,
    const TITLES: usize = 0,
    const DESCRIPTIONS: usize = 0,
    const JSON_LD_TYPE: usize = 0,
    const LINKS: usize = 0,
    const FORMS: usize = 0,
    const SECURITY: usize = 2,
    const SECURITY_DEFINITIONS: usize = 2,
    const PROFILE: usize = 0,
    const SCHEMA_DEFINITIONS: usize = 0,
    const URI_VARIABLES: usize = 0,
> {
    #[serde(rename = "@context")]
    pub context: Vec<ContextEntry<'a>, CONTEXT>,
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
    pub security: Option<Vec<&'a str, SECURITY>>,
    pub security_definitions:
        Option<FnvIndexMap<&'a str, SecurityScheme<'a>, SECURITY_DEFINITIONS>>,
    pub profile: Option<Vec<Link<'a>, PROFILE>>,
    pub schema_definitions: Option<FnvIndexMap<&'a str, DataSchema<'a>, SCHEMA_DEFINITIONS>>,
    pub uri_variables: Option<FnvIndexMap<&'a str, DataSchema<'a>, URI_VARIABLES>>,
}

impl<
        'a,
        const CONTEXT: usize,
        const ACTIONS: usize,
        const PROPERTIES: usize,
        const EVENTS: usize,
        const TITLES: usize,
        const DESCRIPTIONS: usize,
        const JSON_LD_TYPE: usize,
        const LINKS: usize,
        const FORMS: usize,
        const SECURITY: usize,
        const SECURITY_DEFINITIONS: usize,
        const PROFILE: usize,
        const SCHEMA_DEFINITIONS: usize,
        const URI_VARIABLES: usize,
    >
    ThingDescription<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    >
{
    pub fn builder() -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        ThingDescriptionBuilder::default()
    }
}

// #[derive(Default)]
pub struct ThingDescriptionBuilder<
    'a,
    const CONTEXT: usize = 2,
    const ACTIONS: usize = 0,
    const PROPERTIES: usize = 0,
    const EVENTS: usize = 0,
    const TITLES: usize = 0,
    const DESCRIPTIONS: usize = 0,
    const JSON_LD_TYPE: usize = 0,
    const LINKS: usize = 0,
    const FORMS: usize = 0,
    const SECURITY: usize = 0,
    const SECURITY_DEFINITIONS: usize = 0,
    const PROFILE: usize = 0,
    const SCHEMA_DEFINITIONS: usize = 0,
    const URI_VARIABLES: usize = 0,
> {
    pub context: Vec<ContextEntry<'a>, CONTEXT>,
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
    pub security: Option<Vec<&'a str, SECURITY>>,
    pub security_definitions:
        Option<FnvIndexMap<&'a str, SecurityScheme<'a>, SECURITY_DEFINITIONS>>,
    pub profile: Option<Vec<Link<'a>, PROFILE>>,
    pub schema_definitions: Option<FnvIndexMap<&'a str, DataSchema<'a>, SCHEMA_DEFINITIONS>>,
    pub uri_variables: Option<FnvIndexMap<&'a str, DataSchema<'a>, URI_VARIABLES>>,
}

impl<
        'a,
        const CONTEXT: usize,
        const ACTIONS: usize,
        const PROPERTIES: usize,
        const EVENTS: usize,
        const TITLES: usize,
        const DESCRIPTIONS: usize,
        const JSON_LD_TYPE: usize,
        const LINKS: usize,
        const FORMS: usize,
        const SECURITY: usize,
        const SECURITY_DEFINITIONS: usize,
        const PROFILE: usize,
        const SCHEMA_DEFINITIONS: usize,
        const URI_VARIABLES: usize,
    >
    ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    >
{
    fn default() -> Self {
        let mut blah = Vec::<ContextEntry, CONTEXT>::new();
        blah.push(ContextEntry::default()).unwrap();

        Self {
            context: blah,
            json_ld_type: Default::default(),
            id: Default::default(),
            title: Default::default(),
            titles: Default::default(),
            description: Default::default(),
            descriptions: Default::default(),
            created: Default::default(),
            modified: Default::default(),
            support: Default::default(),
            base: Default::default(),
            properties: Default::default(),
            actions: Default::default(),
            events: Default::default(),
            links: Default::default(),
            security: Default::default(),
            profile: Default::default(),
            schema_definitions: Default::default(),
            uri_variables: Default::default(),
            security_definitions: Default::default(),
        }
    }
}

impl<
        'a,
        const CONTEXT: usize,
        const ACTIONS: usize,
        const PROPERTIES: usize,
        const EVENTS: usize,
        const TITLES: usize,
        const DESCRIPTIONS: usize,
        const JSON_LD_TYPE: usize,
        const LINKS: usize,
        const FORMS: usize,
        const SECURITY: usize,
        const SECURITY_DEFINITIONS: usize,
        const PROFILE: usize,
        const SCHEMA_DEFINITIONS: usize,
        const URI_VARIABLES: usize,
    >
    ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    >
{
    pub fn new(
        title: &'a str,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        let mut context = Vec::<ContextEntry, CONTEXT>::new();
        context.push(ContextEntry::default()).unwrap();
        ThingDescriptionBuilder {
            title,
            titles: None,
            context,
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
            security: None,
            profile: None,
            schema_definitions: None,
            uri_variables: None,
            security_definitions: None,
        }
    }

    pub fn title(
        mut self,
        title: &'a str,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.title = title;
        self
    }

    pub fn base(
        mut self,
        base: &'a str,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.base = Some(base);
        self
    }

    pub fn properties(
        mut self,
        properties: FnvIndexMap<&'a str, Property<'a>, PROPERTIES>,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.properties = Some(properties);
        self
    }

    pub fn actions(
        mut self,
        actions: FnvIndexMap<&'a str, Action<'a>, ACTIONS>,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.actions = Some(actions);
        self
    }

    pub fn events(
        mut self,
        events: FnvIndexMap<&'a str, Event<'a>, EVENTS>,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.events = Some(events);
        self
    }

    pub fn links(
        mut self,
        links: Vec<Link<'a>, LINKS>,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.links = Some(links);
        self
    }

    pub fn json_ld_type(
        mut self,
        json_ld_type: Vec<&'a str, JSON_LD_TYPE>,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.json_ld_type = Some(json_ld_type);
        self
    }

    pub fn security(
        mut self,
        security: Vec<&'a str, SECURITY>,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.security = Some(security);
        self
    }

    pub fn security_definitions(
        mut self,
        security_definitions: FnvIndexMap<&'a str, SecurityScheme<'a>, SECURITY_DEFINITIONS>,
    ) -> ThingDescriptionBuilder<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
        self.security_definitions = Some(security_definitions);
        self
    }

    pub fn build(
        self,
    ) -> ThingDescription<
        'a,
        CONTEXT,
        ACTIONS,
        PROPERTIES,
        EVENTS,
        TITLES,
        DESCRIPTIONS,
        JSON_LD_TYPE,
        LINKS,
        FORMS,
        SECURITY,
        SECURITY_DEFINITIONS,
        PROFILE,
        SCHEMA_DEFINITIONS,
        URI_VARIABLES,
    > {
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

    use crate::models::{
        action::{Action, ActionBuilder},
        common::CommonFields,
        data_schema::{DataSchema, DataType},
        form::{Form, FormBuilder},
        link::Link,
        property::{Property, PropertyBuilder},
        security_definition::{SecurityScheme, SecuritySchemeType},
        thing_description::ThingDescription,
    };
    use heapless::{FnvIndexMap, Vec};
    use serde_json_core::{heapless::String, ser::Error, to_string};

    #[test]
    fn serialize_thing_description() -> Result<(), Error> {
        let mut property_forms = Vec::<Form, 2>::new();
        let property_form = FormBuilder::new("coaps://example.org/status").build();
        property_forms.push(property_form).unwrap();

        let property_data_schema = DataSchema {
            json_ld_type: None,
            common_fields: Some(CommonFields::builder().title("Status").build()),
            constant: None,
            default: None,
            unit: None,
            one_of: None,
            enumeration: None,
            read_only: None,
            write_only: None,
            format: None,
            data_type: Some(DataType::Boolean),
        };

        // FIXME: For some reason the size needs to be 2 here, otherwise an overflow occurs.
        //        See https://github.com/japaric/heapless/issues/216
        let mut properties = FnvIndexMap::<&str, Property, 2>::new();

        let first_property = PropertyBuilder::new(&property_forms, &property_data_schema).build();

        properties.insert("status", first_property).unwrap();

        let mut first_action_forms = Vec::<Form, 2>::new();
        first_action_forms
            .push(FormBuilder::new("coaps://example.org/toggle").build())
            .unwrap();
        let action_input = DataSchema {
            json_ld_type: None,
            common_fields: Some(CommonFields::builder().title("Toggle Data").build()),
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

        let mut second_action_forms = Vec::<Form, 2>::new();
        second_action_forms
            .push(FormBuilder::new("coaps://example.org/toggle2").build())
            .unwrap();

        let mut actions = FnvIndexMap::<&str, Action, 2>::new();
        actions
            .insert(
                "toggle",
                ActionBuilder::new(&first_action_forms)
                    .input(&action_input)
                    .build(),
            )
            .unwrap();
        actions
            .insert("toggle2", ActionBuilder::new(&second_action_forms).build())
            .unwrap();

        const LINKS_LENGTH: usize = 1;

        let mut links = Vec::<Link, LINKS_LENGTH>::new();
        links
            .push(Link::builder().href("https://example.org").build())
            .unwrap();

        const JSON_LD_TYPE_LENGTH: usize = 2;

        let mut json_ld_type = Vec::<&str, JSON_LD_TYPE_LENGTH>::new();
        json_ld_type.push("saref:LightSwitch").unwrap();

        const NO_SEC_KEY: &str = "nosec_sc";
        const SECURITY_LENGTH: usize = 2;
        let mut security = Vec::<&str, SECURITY_LENGTH>::new();
        security.push(NO_SEC_KEY).unwrap();

        const SECURITY_DEFINITIONS_LENGTH: usize = 2;
        let mut security_definitions =
            FnvIndexMap::<&str, SecurityScheme, SECURITY_DEFINITIONS_LENGTH>::new();
        security_definitions
            .insert(
                NO_SEC_KEY,
                SecurityScheme {
                    description: None,
                    scheme: SecuritySchemeType::Nosec,
                },
            )
            .unwrap();

        let thing_description = ThingDescription::<
            2,
            2,
            2,
            0,
            0,
            0,
            JSON_LD_TYPE_LENGTH,
            LINKS_LENGTH,
            0,
            SECURITY_LENGTH,
            SECURITY_DEFINITIONS_LENGTH,
        >::builder()
        .title("Test TD")
        .json_ld_type(json_ld_type)
        .properties(properties)
        .actions(actions)
        .links(links)
        .security(security)
        .security_definitions(security_definitions)
        .build();

        let expected_result = r#"{"@context":["https://www.w3.org/2022/wot/td/v1.1"],"@type":["saref:LightSwitch"],"title":"Test TD","properties":{"status":{"forms":[{"href":"coaps://example.org/status"}],"title":"Status","type":"boolean"}},"actions":{"toggle":{"forms":[{"href":"coaps://example.org/toggle"}],"input":{"title":"Toggle Data"}},"toggle2":{"forms":[{"href":"coaps://example.org/toggle2"}]}},"links":[{"href":"https://example.org"}],"security":["nosec_sc"],"securityDefinitions":{"nosec_sc":{"scheme":"nosec"}}}"#;
        let actual_result: String<600> = to_string(&thing_description)?;

        assert_eq!(expected_result, actual_result.as_str());
        Ok(())
    }
}
