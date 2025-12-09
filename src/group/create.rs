use std::collections::HashMap;

use reqwest::Client;
use serde::Deserialize;

use crate::parser::parse_response;

pub struct CreateGroupQuery {
    url: String,
    params: HashMap<String, String>,
}

#[derive(Default)]
pub struct CreateGroupQueryBuilder {
    url: String,
    title: String,
    access: AccessLevel, // Who can view this group (private)
    description: Option<String>,
    type_keywords: Vec<String>,
    snippet: Option<String>,
    tags: Vec<String>,
    phone: Option<String>,
    sort_field: Option<SortField>,
    sort_order: Option<SortOrder>,
    is_view_only: bool,
    is_invitation_only: bool,
    thumbnail: Option<String>,
    capabilities: Vec<Capability>,
    leaving_disallowed: bool,
    hidden_members: bool,
    membership_access: Option<MembershipAccess>,
    auto_join: bool,
    privacy: Option<AccessLevel>,
    contribute: Option<Contributors>,
}

impl CreateGroupQuery {
    pub fn builder(root: impl Into<String>, title: impl Into<String>) -> CreateGroupQueryBuilder {
        CreateGroupQueryBuilder::new(root, title)
    }

    pub async fn send(&self, client: &Client) -> anyhow::Result<GroupResponse> {
        let response = client.post(&self.url).form(&self.params).send().await?;
        let body = parse_response::<GroupResponse>(response).await?;
        Ok(body)
    }
}

impl CreateGroupQueryBuilder {
    pub fn new(root: impl Into<String>, title: impl Into<String>) -> Self {
        let url = format!("{}/community/createGroup", root.into());
        // TODO: validtate url
        Self {
            url,
            title: title.into(),
            ..Default::default()
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

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
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

    pub fn build(self) -> CreateGroupQuery {
        let mut params = HashMap::new();
        params.insert("title".into(), self.title);
        params.insert("access".into(), format!("{:?}", self.access).to_lowercase());

        if let Some(desc) = self.description {
            params.insert("description".into(), desc);
        }

        if !self.type_keywords.is_empty() {
            params.insert("typeKeywords".into(), self.type_keywords.join(","));
        }

        if let Some(snippet) = self.snippet {
            params.insert("snippet".into(), snippet);
        }

        if !self.tags.is_empty() {
            params.insert("tags".into(), self.tags.join(","));
        }

        if let Some(phone) = self.phone {
            params.insert("phone".into(), phone);
        }

        if let Some(sf) = self.sort_field {
            params.insert("sortField".into(), format!("{:?}", sf).to_lowercase());
        }

        if let Some(so) = self.sort_order {
            params.insert("sortOrder".into(), format!("{:?}", so).to_lowercase());
        }

        if self.is_view_only {
            params.insert("isViewOnly".into(), "true".into());
        }

        if self.is_invitation_only {
            params.insert("isInvitationOnly".into(), "true".into());
        }

        if let Some(thumb) = self.thumbnail {
            params.insert("thumbnail".into(), thumb);
        }

        if !self.capabilities.is_empty() {
            let caps = self
                .capabilities
                .into_iter()
                .map(|c| format!("{:?}", c))
                .collect::<Vec<_>>()
                .join(",");
            params.insert("capabilities".into(), caps);
        }

        if self.leaving_disallowed {
            params.insert("leavingDisallowed".into(), "true".into());
        }

        if self.hidden_members {
            params.insert("hiddenMembers".into(), "true".into());
        }

        if let Some(mem_access) = self.membership_access {
            params.insert(
                "membershipAccess".into(),
                format!("{:?}", mem_access).to_lowercase(),
            );
        }

        if self.auto_join {
            params.insert("autoJoin".into(), "true".into());
        }

        params.insert("f".into(), "json".into());

        CreateGroupQuery {
            url: self.url,
            params,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GroupResponse {
    pub success: bool,
    pub group: Group,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    pub id: String,
    pub title: String,

    #[serde(rename = "isInvitationOnly")]
    pub is_invitation_only: bool,

    #[serde(rename = "isViewOnly")]
    pub is_view_only: bool,

    #[serde(rename = "isOrganization")]
    pub is_organization: Option<bool>,

    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,

    #[serde(rename = "isReadOnly")]
    pub is_read_only: bool,

    #[serde(rename = "isFav")]
    pub is_fav: bool,

    #[serde(rename = "autoJoin")]
    pub auto_join: bool,

    #[serde(rename = "leavingDisallowed")]
    pub leaving_disallowed: bool,

    #[serde(rename = "hiddenMembers")]
    pub hidden_members: bool,

    #[serde(rename = "membershipAccess")]
    pub membership_access: String,

    pub access: String,
    pub owner: String,

    pub description: Option<String>,
    pub snippet: Option<String>,
    pub phone: Option<String>,
    pub thumbnail: Option<String>,

    #[serde(rename = "sortField")]
    pub sort_field: Option<String>,

    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,

    pub tags: Vec<String>,

    #[serde(rename = "typeKeywords")]
    pub type_keywords: Vec<String>,

    pub capabilities: Vec<String>, // assuming strings in the array

    pub created: i64,
    pub modified: i64,

    #[serde(rename = "notificationsEnabled")]
    pub notifications_enabled: bool,

    pub provider: Option<String>,
    pub protected: bool,

    #[serde(rename = "providerGroupName")]
    pub provider_group_name: Option<String>,

    pub properties: Option<serde_json::Value>,

    #[serde(rename = "featuredItemsId")]
    pub featured_items_id: Option<String>,

    #[serde(rename = "displaySettings")]
    pub display_settings: Option<DisplaySettings>,
}

#[derive(Debug, Deserialize)]
pub struct DisplaySettings {
    #[serde(rename = "itemTypes")]
    pub item_types: String,
}

#[derive(Debug, Deserialize)]
pub struct GroupInput {
    pub title: String,
    pub access: AccessLevel,

    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(rename = "typeKeywords")]
    pub type_keywords: Vec<String>,

    #[serde(default)]
    pub snippet: Option<String>,

    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub phone: Option<String>,

    #[serde(default)]
    #[serde(rename = "sortField")]
    pub sort_field: Option<SortField>,

    #[serde(default)]
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<SortOrder>,

    #[serde(default)]
    #[serde(rename = "isViewOnly")]
    pub is_view_only: bool,

    #[serde(default)]
    #[serde(rename = "isInvitationOnly")]
    pub is_invitation_only: bool,

    #[serde(default)]
    pub thumbnail: Option<String>,

    #[serde(default)]
    pub capabilities: Vec<Capability>,

    #[serde(default)]
    #[serde(rename = "leavingDisallowed")]
    pub leaving_disallowed: bool,

    #[serde(default)]
    #[serde(rename = "hiddenMembers")]
    pub hidden_members: bool,

    #[serde(default)]
    #[serde(rename = "membershipAccess")]
    pub membership_access: Option<MembershipAccess>,

    #[serde(default)]
    #[serde(rename = "autoJoin")]
    pub auto_join: bool,

    #[serde(default)]
    pub privacy: Option<AccessLevel>,

    #[serde(default)]
    pub contribute: Option<Contributors>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessLevel {
    Private,
    Org,
    Public,
}

impl Default for AccessLevel {
    fn default() -> Self {
        AccessLevel::Private
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortField {
    Title,
    Owner,
    Avgrating,
    Numviews,
    Created,
    Modified,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Capability {
    UpdateItemControl,
    Distributed,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MembershipAccess {
    Org,
    Collaboration,
    None,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Contributors {
    Members,
    Owners,
}
