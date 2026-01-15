use crate::error::{Result, UrlParseSnafu};
use crate::models::*;
use crate::{serialize_comma_separated, ArcGISSharingClient};
use serde::Serialize;
use snafu::ResultExt;

// TODO: add doc comments

pub struct CreateGroupHandler<'a> {
    client: &'a ArcGISSharingClient,
}

impl<'a> CreateGroupHandler<'a> {
    pub(crate) fn new(client: &'a ArcGISSharingClient) -> Self {
        Self { client }
    }

    pub fn create(&self, title: impl Into<String>) -> CreateGroupBuilder<'_, '_> {
        CreateGroupBuilder::new(self, title)
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGroupBuilder<'a, 'r> {
    #[serde(skip)]
    handler: &'r CreateGroupHandler<'a>,

    title: String,

    access: AccessLevel, // Who can view this group (private)

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "Vec::is_empty"
    )]
    type_keywords: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    snippet: Option<String>,

    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "Vec::is_empty"
    )]
    tags: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_field: Option<SortField>,

    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<SortOrder>,

    is_view_only: bool,

    is_invitation_only: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail: Option<String>,

    #[serde(
        serialize_with = "serialize_comma_separated",
        skip_serializing_if = "Vec::is_empty"
    )]
    capabilities: Vec<Capability>,

    leaving_disallowed: bool,

    hidden_members: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    membership_access: Option<MembershipAccess>,

    auto_join: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<AccessLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    contribute: Option<Contributors>,
}

impl<'a, 'r> CreateGroupBuilder<'a, 'r> {
    pub fn new(handler: &'r CreateGroupHandler<'a>, title: impl Into<String>) -> Self {
        Self {
            handler,
            title: title.into(),
            access: AccessLevel::default(),
            description: None,
            type_keywords: Vec::new(),
            snippet: None,
            tags: Vec::new(),
            phone: None,
            sort_field: None,
            sort_order: None,
            is_view_only: false,
            is_invitation_only: false,
            thumbnail: None,
            capabilities: Vec::new(),
            leaving_disallowed: false,
            hidden_members: false,
            membership_access: None,
            auto_join: false,
            privacy: None,
            contribute: None,
        }
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn type_keywords(mut self, keywords: Vec<String>) -> Self {
        self.type_keywords = keywords;
        self
    }

    pub fn snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }

    pub fn tags(mut self, tags: Vec<impl Into<String>>) -> Self {
        self.tags = tags.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    pub fn access(mut self, access: AccessLevel) -> Self {
        self.access = access;
        self
    }

    pub fn sort_field(mut self, sort_field: SortField) -> Self {
        self.sort_field = Some(sort_field);
        self
    }

    pub fn sort_order(mut self, sort_order: SortOrder) -> Self {
        self.sort_order = Some(sort_order);
        self
    }

    pub fn is_view_only(mut self, val: bool) -> Self {
        self.is_view_only = val;
        self
    }

    pub fn is_invitation_only(mut self, val: bool) -> Self {
        self.is_invitation_only = val;
        self
    }

    pub fn thumbnail(mut self, thumb: impl Into<String>) -> Self {
        self.thumbnail = Some(thumb.into());
        self
    }

    pub fn capabilities(mut self, caps: Vec<Capability>) -> Self {
        self.capabilities = caps;
        self
    }

    pub fn leaving_disallowed(mut self, val: bool) -> Self {
        self.leaving_disallowed = val;
        self
    }

    pub fn hidden_members(mut self, val: bool) -> Self {
        self.hidden_members = val;
        self
    }

    pub fn membership_access(mut self, access: MembershipAccess) -> Self {
        self.membership_access = Some(access);
        self
    }

    pub fn auto_join(mut self, val: bool) -> Self {
        self.auto_join = val;
        self
    }

    pub fn privacy(mut self, val: AccessLevel) -> Self {
        self.privacy = Some(val);
        self
    }

    pub fn contribute(mut self, val: Contributors) -> Self {
        self.contribute = Some(val);
        self
    }

    pub async fn send(&self) -> Result<CreateGroupResponse> {
        let url = self
            .handler
            .client
            .portal
            .join("sharing/rest/community/createGroup")
            .context(UrlParseSnafu)?;

        self.handler.client.post(url, Some(&self), None).await
    }
}
