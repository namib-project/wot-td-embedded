use heapless::FnvIndexMap;
use serde::ser::SerializeMap;

use crate::serialize_field;

use super::{action::Action, event::Event, property::Property};

pub struct InteractionAffordances<
    'a,
    const PROPERTIES: usize = 0,
    const ACTIONS: usize = 0,
    const EVENTS: usize = 0,
> {
    pub properties: Option<FnvIndexMap<&'a str, Property<'a>, PROPERTIES>>,
    pub actions: Option<FnvIndexMap<&'a str, Action<'a>, ACTIONS>>,
    pub events: Option<FnvIndexMap<&'a str, Event<'a>, EVENTS>>,
}

impl<'a, const PROPERTIES: usize, const ACTIONS: usize, const EVENTS: usize>
    InteractionAffordances<'a, PROPERTIES, ACTIONS, EVENTS>
{
    pub fn serialize_to_map<S>(&self, mut map: S::SerializeMap) -> Result<S::SerializeMap, S::Error>
    where
        S: serde::Serializer,
    {
        serialize_field!("properties", &self.properties, map);
        serialize_field!("actions", &self.actions, map);
        serialize_field!("events", &self.events, map);

        Ok(map)
    }
}

#[derive(Default, Debug)]
pub struct InteractionAffordancesBuilder<
    'a,
    const PROPERTIES: usize = 0,
    const ACTIONS: usize = 0,
    const EVENTS: usize = 0,
> {
    pub properties: Option<FnvIndexMap<&'a str, Property<'a>, PROPERTIES>>,
    pub actions: Option<FnvIndexMap<&'a str, Action<'a>, ACTIONS>>,
    pub events: Option<FnvIndexMap<&'a str, Event<'a>, EVENTS>>,
}

impl<'a, const PROPERTIES: usize, const ACTIONS: usize, const EVENTS: usize>
    InteractionAffordancesBuilder<'a, PROPERTIES, ACTIONS, EVENTS>
{
    pub fn new() -> InteractionAffordancesBuilder<'a, PROPERTIES, ACTIONS, EVENTS> {
        InteractionAffordancesBuilder::default()
    }

    pub fn properties(
        mut self,
        properties: FnvIndexMap<&'a str, Property<'a>, PROPERTIES>,
    ) -> InteractionAffordancesBuilder<'a, PROPERTIES, ACTIONS, EVENTS> {
        self.properties = Some(properties);
        self
    }

    pub fn actions(
        mut self,
        actions: FnvIndexMap<&'a str, Action<'a>, ACTIONS>,
    ) -> InteractionAffordancesBuilder<'a, PROPERTIES, ACTIONS, EVENTS> {
        self.actions = Some(actions);
        self
    }

    pub fn events(
        mut self,
        events: FnvIndexMap<&'a str, Event<'a>, EVENTS>,
    ) -> InteractionAffordancesBuilder<'a, PROPERTIES, ACTIONS, EVENTS> {
        self.events = Some(events);
        self
    }

    pub fn build(self) -> InteractionAffordances<'a, PROPERTIES, ACTIONS, EVENTS> {
        InteractionAffordances {
            properties: self.properties,
            actions: self.actions,
            events: self.events,
        }
    }
}
