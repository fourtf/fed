use std::collections::HashMap;

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
    pub begin_captures: Option<BeginCaptures>,
    #[serde(default)]
    pub patterns: Vec<Pattern2>,
    pub include: Option<String>,
    pub name: Option<String>,
    pub captures: Option<Captures>,
    #[serde(rename = "match")]
    pub match_field: Option<String>,
}

//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct BeginCaptures {
//    #[serde(rename = "1")]
//    pub n1: N1,
//    #[serde(rename = "2")]
//    pub n2: Option<N2>,
//}

type BeginCaptures = HashMap<String, N1>;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N1 {
    pub name: String,
}

//#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
//#[serde(rename_all = "camelCase")]
//pub struct N2 {
//    pub name: String,
//}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern2 {
    pub include: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "match")]
    pub match_field: Option<String>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Captures {
    #[serde(rename = "1")]
    pub n1: N12,
    #[serde(rename = "2")]
    pub n2: Option<N22>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N12 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N22 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    #[serde(rename = "block_doc_comment")]
    pub block_doc_comment: BlockDocComment,
    #[serde(rename = "block_comment")]
    pub block_comment: BlockComment,
    #[serde(rename = "line_doc_comment")]
    pub line_doc_comment: LineDocComment,
    #[serde(rename = "line_comment")]
    pub line_comment: LineComment,
    #[serde(rename = "escaped_character")]
    pub escaped_character: EscapedCharacter,
    #[serde(rename = "string_literal")]
    pub string_literal: StringLiteral,
    #[serde(rename = "raw_string_literal")]
    pub raw_string_literal: RawStringLiteral,
    pub sigils: Sigils,
    #[serde(rename = "self")]
    pub self_field: Self_field,
    #[serde(rename = "mut")]
    pub mut_field: Mut,
    pub dyn: Dyn,
    #[serde(rename = "impl")]
    pub impl_field: Impl,
    #[serde(rename = "box")]
    pub box_field: Box,
    #[serde(rename = "const")]
    pub const_field: Const,
    #[serde(rename = "pub")]
    pub pub_field: Pub,
    #[serde(rename = "unsafe")]
    pub unsafe_field: Unsafe,
    #[serde(rename = "where")]
    pub where_field: Where,
    pub lifetime: Lifetime,
    #[serde(rename = "ref_lifetime")]
    pub ref_lifetime: RefLifetime,
    #[serde(rename = "core_types")]
    pub core_types: CoreTypes,
    #[serde(rename = "core_vars")]
    pub core_vars: CoreVars,
    #[serde(rename = "core_marker")]
    pub core_marker: CoreMarker,
    #[serde(rename = "core_traits")]
    pub core_traits: CoreTraits,
    #[serde(rename = "std_types")]
    pub std_types: StdTypes,
    #[serde(rename = "std_traits")]
    pub std_traits: StdTraits,
    #[serde(rename = "type")]
    pub type_field: Type,
    #[serde(rename = "type_params")]
    pub type_params: TypeParams,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockDocComment {
    pub comment: String,
    pub name: String,
    pub begin: String,
    pub end: String,
    pub patterns: Vec<Pattern3>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern3 {
    pub include: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockComment {
    pub comment: String,
    pub name: String,
    pub begin: String,
    pub end: String,
    pub patterns: Vec<Pattern4>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern4 {
    pub include: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineDocComment {
    pub comment: String,
    pub name: String,
    pub begin: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LineComment {
    pub comment: String,
    pub name: String,
    pub begin: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EscapedCharacter {
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StringLiteral {
    pub comment: String,
    pub name: String,
    pub begin: String,
    pub end: String,
    pub patterns: Vec<Pattern5>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern5 {
    pub include: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawStringLiteral {
    pub comment: String,
    pub name: String,
    pub begin: String,
    pub end: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sigils {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Self_field {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mut {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dyn {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Impl {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Box {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Const {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pub {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unsafe {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Where {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lifetime {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefLifetime {
    pub comment: String,
    #[serde(rename = "match")]
    pub match_field: String,
    pub captures: Captures2,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Captures2 {
    #[serde(rename = "1")]
    pub n1: N13,
    #[serde(rename = "2")]
    pub n2: N23,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N13 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N23 {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreTypes {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreVars {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreMarker {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreTraits {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StdTypes {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StdTraits {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub comment: String,
    pub name: String,
    #[serde(rename = "match")]
    pub match_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeParams {
    pub comment: String,
    pub name: String,
    pub begin: String,
    pub end: String,
    pub patterns: Vec<Pattern6>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pattern6 {
    pub include: String,
}
