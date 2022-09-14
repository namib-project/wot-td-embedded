use crate::data_structures::map::Map;

#[derive(Debug)]
pub struct CommonFields<'a> {
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
}
impl<'a> CommonFields<'a> {
    pub fn builder() -> CommonFieldsBuilder<'a> {
        CommonFieldsBuilder::default()
    }
}

#[derive(Default)]
pub struct CommonFieldsBuilder<'a> {
    pub title: Option<&'a str>,
    pub titles: Option<&'a Map<'a, &'a str>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a Map<'a, &'a str>>,
}

impl<'a> CommonFieldsBuilder<'a> {
    pub fn new() -> CommonFieldsBuilder<'a> {
        CommonFieldsBuilder::default()
    }

    pub fn title(mut self, title: &'a str) -> CommonFieldsBuilder<'a> {
        self.title = Some(title);
        self
    }

    pub fn titles(mut self, titles: &'a Map<'a, &'a str>) -> CommonFieldsBuilder<'a> {
        self.titles = Some(titles);
        self
    }

    pub fn description(mut self, description: &'a str) -> CommonFieldsBuilder<'a> {
        self.description = Some(description);
        self
    }

    pub fn descriptions(mut self, descriptions: &'a Map<'a, &'a str>) -> CommonFieldsBuilder<'a> {
        self.descriptions = Some(descriptions);
        self
    }

    pub fn build(self) -> CommonFields<'a> {
        CommonFields {
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
        }
    }
}
