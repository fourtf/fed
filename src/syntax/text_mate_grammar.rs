use std::collections::HashMap;

/*
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub name: String,
    pub scope_name: String,
    pub patterns: Vec<Pattern>,
    pub repository: Repository,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern {
    pub comment: Option<String>,
    pub begin: Option<String>,
    pub end: Option<String>,
    pub begin_captures: Option<Captures>,
    //#[serde(default)]
    pub patterns: Option<Vec<Pattern2>>,
    pub include: Option<String>,
    pub name: Option<String>,
    pub captures: Option<Captures>,
    #[serde(rename = "match")]
    pub match_field: Option<String>,
}

pub type Captures = HashMap<String, N1>;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N1 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern2 {
    pub include: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "match")]
    pub match_field: Option<String>,
    pub name: Option<String>,
}

pub type Repository = HashMap<String, RepositoryElement>;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryElement {
    pub comment: Option<String>,
    pub name: Option<String>,
    pub begin: Option<String>,
    pub end: Option<String>,
    pub patterns: Option<Vec<RepositoryElementReference>>,
    #[serde(rename = "match")]
    pub match_field: Option<String>,
    pub captures: Option<Captures>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryElementReference {
    pub include: String,
}
*/
