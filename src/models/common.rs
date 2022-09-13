use heapless::FnvIndexMap;

#[derive(Debug)]
pub struct CommonFields<'a, const TITLES: usize = 0, const DESCRIPTIONS: usize = 0> {
    pub title: Option<&'a str>,
    pub titles: Option<&'a FnvIndexMap<&'a str, &'a str, TITLES>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a FnvIndexMap<&'a str, &'a str, DESCRIPTIONS>>,
}
impl<'a, const TITLES: usize, const DESCRIPTIONS: usize> CommonFields<'a, TITLES, DESCRIPTIONS> {
    pub fn builder() -> CommonFieldsBuilder<'a, TITLES, DESCRIPTIONS> {
        CommonFieldsBuilder::default()
    }
}

#[derive(Default)]
pub struct CommonFieldsBuilder<'a, const TITLES: usize = 0, const DESCRIPTIONS: usize = 0> {
    pub title: Option<&'a str>,
    pub titles: Option<&'a FnvIndexMap<&'a str, &'a str, TITLES>>,
    pub description: Option<&'a str>,
    pub descriptions: Option<&'a FnvIndexMap<&'a str, &'a str, DESCRIPTIONS>>,
}

impl<'a, const TITLES: usize, const DESCRIPTIONS: usize>
    CommonFieldsBuilder<'a, TITLES, DESCRIPTIONS>
{
    pub fn new() -> CommonFieldsBuilder<'a, TITLES, DESCRIPTIONS> {
        CommonFieldsBuilder::default()
    }

    pub fn title(&mut self, title: &'a str) -> &mut CommonFieldsBuilder<'a, TITLES, DESCRIPTIONS> {
        self.title = Some(title);
        self
    }

    pub fn titles(
        &mut self,
        titles: &'a FnvIndexMap<&'a str, &'a str, TITLES>,
    ) -> &mut CommonFieldsBuilder<'a, TITLES, DESCRIPTIONS> {
        self.titles = Some(titles);
        self
    }

    pub fn description(
        &mut self,
        description: &'a str,
    ) -> &mut CommonFieldsBuilder<'a, TITLES, DESCRIPTIONS> {
        self.description = Some(description);
        self
    }

    pub fn descriptions(
        &mut self,
        descriptions: &'a FnvIndexMap<&'a str, &'a str, DESCRIPTIONS>,
    ) -> &mut CommonFieldsBuilder<'a, TITLES, DESCRIPTIONS> {
        self.descriptions = Some(descriptions);
        self
    }

    pub fn build(&mut self) -> CommonFields<'a, TITLES, DESCRIPTIONS> {
        CommonFields {
            title: self.title,
            titles: self.titles,
            description: self.description,
            descriptions: self.descriptions,
        }
    }
}
