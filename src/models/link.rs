use heapless::Vec;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Link<'a, const HREF_LANG: usize = 0> {
    pub href: &'a str,
    #[serde(rename = "type")]
    pub link_type: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub anchor: Option<&'a str>,
    pub sizes: Option<&'a str>,
    pub hreflang: Option<&'a Vec<&'a str, HREF_LANG>>,
}

impl<'a, const HREF_LANG: usize> Link<'a, HREF_LANG> {
    pub fn builder() -> LinkBuilder<'a, HREF_LANG> {
        LinkBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct LinkBuilder<'a, const HREF_LANG: usize = 0> {
    pub href: &'a str,
    pub link_type: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub anchor: Option<&'a str>,
    pub sizes: Option<&'a str>,
    pub hreflang: Option<&'a Vec<&'a str, HREF_LANG>>,
}

impl<'a, const HREF_LANG: usize> LinkBuilder<'a, HREF_LANG> {
    pub fn new(href: &'a str) -> LinkBuilder<'a, HREF_LANG> {
        let mut link_builder = LinkBuilder::default();
        link_builder.href = href;
        link_builder
    }

    pub fn href(&mut self, href: &'a str) -> &LinkBuilder<'a, HREF_LANG> {
        self.href = href;
        self
    }

    pub fn link_type(&mut self, link_type: &'a str) -> &LinkBuilder<'a, HREF_LANG> {
        self.link_type = Some(link_type);
        self
    }

    pub fn rel(&mut self, rel: &'a str) -> &LinkBuilder<'a, HREF_LANG> {
        self.rel = Some(rel);
        self
    }

    pub fn anchor(&mut self, anchor: &'a str) -> &LinkBuilder<'a, HREF_LANG> {
        self.anchor = Some(anchor);
        self
    }

    pub fn hreflang(
        &mut self,
        hreflang: &'a Vec<&'a str, HREF_LANG>,
    ) -> &LinkBuilder<'a, HREF_LANG> {
        self.hreflang = Some(hreflang);
        self
    }

    pub fn build(&self) -> Link<'a, HREF_LANG> {
        Link {
            href: self.href,
            link_type: self.link_type,
            rel: self.rel,
            anchor: self.anchor,
            sizes: self.sizes,
            hreflang: self.hreflang,
        }
    }
}
