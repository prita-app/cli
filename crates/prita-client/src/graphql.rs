//! Typed GraphQL operations, checked against the vendored schema at build time.

use cynic::{MutationBuilder, QueryBuilder};

use crate::PritaClient;
use crate::error::Error;

#[cynic::schema("prita")]
pub mod schema {}

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

/// A tag's color, from a fixed palette. Orange is deliberately absent (brand accent).
#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(graphql_type = "TagColor")]
pub enum TagColor {
    Tomato,
    Crimson,
    Plum,
    Violet,
    Indigo,
    Blue,
    Teal,
    Grass,
    Amber,
    Brown,
}

impl TagColor {
    /// The color names accepted on the command line, in palette order.
    pub const NAMES: &'static [&'static str] = &[
        "tomato", "crimson", "plum", "violet", "indigo", "blue", "teal", "grass", "amber", "brown",
    ];

    /// Parse a color name case-insensitively.
    pub fn parse(name: &str) -> Option<Self> {
        Some(match name.trim().to_ascii_lowercase().as_str() {
            "tomato" => Self::Tomato,
            "crimson" => Self::Crimson,
            "plum" => Self::Plum,
            "violet" => Self::Violet,
            "indigo" => Self::Indigo,
            "blue" => Self::Blue,
            "teal" => Self::Teal,
            "grass" => Self::Grass,
            "amber" => Self::Amber,
            "brown" => Self::Brown,
            _ => return None,
        })
    }

    /// The lowercase color name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tomato => "tomato",
            Self::Crimson => "crimson",
            Self::Plum => "plum",
            Self::Violet => "violet",
            Self::Indigo => "indigo",
            Self::Blue => "blue",
            Self::Teal => "teal",
            Self::Grass => "grass",
            Self::Amber => "amber",
            Self::Brown => "brown",
        }
    }
}

/// Whether a tag is user-owned or a system tag.
#[derive(cynic::Enum, Clone, Copy, Debug)]
#[cynic(graphql_type = "TagKind")]
pub enum TagKind {
    User,
    UpNext,
    Favorites,
}

impl TagKind {
    /// The lowercase kind name.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::UpNext => "up_next",
            Self::Favorites => "favorites",
        }
    }
}

// ---------------------------------------------------------------------------
// Output fragments
// ---------------------------------------------------------------------------

#[derive(cynic::QueryFragment, Debug)]
pub struct Tag {
    pub id: cynic::Id,
    pub name: String,
    pub color: Option<TagColor>,
    pub parent_id: Option<cynic::Id>,
    pub kind: TagKind,
    pub created_at: String,
    pub pinned_at: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Article")]
pub struct ArticleSummary {
    pub id: cynic::Id,
    pub title: String,
    pub url: String,
    pub author: Option<String>,
    pub site_name: Option<String>,
    pub description: Option<String>,
    pub captured_at: String,
    pub reading_progress_percent: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Article")]
pub struct ArticleDetail {
    pub id: cynic::Id,
    pub title: String,
    pub url: String,
    pub author: Option<String>,
    pub site_name: Option<String>,
    pub description: Option<String>,
    pub published_time: Option<String>,
    pub captured_at: String,
    pub reading_progress_percent: Option<i32>,
    pub read_at: Option<String>,
    pub text_content: String,
    pub tags: Vec<Tag>,
}

/// An article plus its tags, returned by the tag/untag mutations.
#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Article")]
pub struct ArticleTags {
    pub id: cynic::Id,
    pub title: String,
    pub tags: Vec<Tag>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "ArticleConnection")]
pub struct ArticleConnection {
    pub page_info: PageInfo,
    pub edges: Vec<ArticleEdge>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "ArticleEdge")]
pub struct ArticleEdge {
    pub cursor: String,
    pub node: ArticleSummary,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "ArticleUploadResult")]
pub struct ArticleUploadResult {
    pub article: ArticleSummary,
    pub was_already_saved: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "DeleteTagResult")]
pub struct DeleteTagResult {
    pub deleted_tag_ids: Vec<cynic::Id>,
    pub deleted_count: i32,
}

// ---------------------------------------------------------------------------
// Input objects
// ---------------------------------------------------------------------------

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "UploadArticleInput")]
pub struct UploadArticleInput {
    pub url: String,
    pub content: Option<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "SaveReadingProgressInput")]
pub struct SaveReadingProgressInput {
    pub article_id: cynic::Id,
    pub reading_progress_percent: i32,
    pub reading_progress_anchor_idx: Option<i32>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "CreateTagInput")]
pub struct CreateTagInput {
    pub name: String,
    pub parent_id: Option<cynic::Id>,
    pub color: Option<TagColor>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "RenameTagInput")]
pub struct RenameTagInput {
    pub id: cynic::Id,
    pub name: String,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "SetTagColorInput")]
pub struct SetTagColorInput {
    pub id: cynic::Id,
    pub color: Option<TagColor>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "MoveTagInput")]
pub struct MoveTagInput {
    pub id: cynic::Id,
    pub parent_id: Option<cynic::Id>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "CreateTagPathInput")]
pub struct CreateTagPathInput {
    pub segments: Vec<String>,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "TagArticleInput")]
pub struct TagArticleInput {
    pub article_id: cynic::Id,
    pub tag_id: cynic::Id,
}

#[derive(cynic::InputObject, Debug)]
#[cynic(graphql_type = "UntagArticleInput")]
pub struct UntagArticleInput {
    pub article_id: cynic::Id,
    pub tag_id: cynic::Id,
}

// ---------------------------------------------------------------------------
// Operations
// ---------------------------------------------------------------------------

/// Reused by every operation whose only argument is an `id`.
#[derive(cynic::QueryVariables, Debug)]
pub struct IdVariables {
    pub id: cynic::Id,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct ArticlesVariables {
    pub tag: Option<cynic::Id>,
    pub first: Option<i32>,
    pub after: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ArticlesVariables")]
struct ArticlesQuery {
    #[arguments(tag: $tag, first: $first, after: $after)]
    articles: ArticleConnection,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "IdVariables")]
struct ArticleQuery {
    #[arguments(id: $id)]
    article: Option<ArticleDetail>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query")]
struct TagsQuery {
    tags: Vec<Tag>,
}

macro_rules! input_variables {
    ($name:ident, $input:ty) => {
        #[derive(cynic::QueryVariables, Debug)]
        pub struct $name {
            pub input: $input,
        }
    };
}

input_variables!(UploadVariables, UploadArticleInput);
input_variables!(SaveProgressVariables, SaveReadingProgressInput);
input_variables!(CreateTagVariables, CreateTagInput);
input_variables!(RenameTagVariables, RenameTagInput);
input_variables!(SetTagColorVariables, SetTagColorInput);
input_variables!(MoveTagVariables, MoveTagInput);
input_variables!(CreateTagPathVariables, CreateTagPathInput);
input_variables!(TagArticleVariables, TagArticleInput);
input_variables!(UntagArticleVariables, UntagArticleInput);

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UploadVariables")]
struct UploadArticleMutation {
    #[arguments(input: $input)]
    upload_article: ArticleUploadResult,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "IdVariables")]
struct DeleteArticleMutation {
    #[arguments(id: $id)]
    delete_article: ArticleSummary,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "SaveProgressVariables")]
struct SaveReadingProgressMutation {
    #[arguments(input: $input)]
    save_reading_progress: bool,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "TagArticleVariables")]
struct TagArticleMutation {
    #[arguments(input: $input)]
    tag_article: ArticleTags,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "UntagArticleVariables")]
struct UntagArticleMutation {
    #[arguments(input: $input)]
    untag_article: ArticleTags,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "CreateTagVariables")]
struct CreateTagMutation {
    #[arguments(input: $input)]
    create_tag: Tag,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "RenameTagVariables")]
struct RenameTagMutation {
    #[arguments(input: $input)]
    rename_tag: Tag,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "SetTagColorVariables")]
struct SetTagColorMutation {
    #[arguments(input: $input)]
    set_tag_color: Tag,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "MoveTagVariables")]
struct MoveTagMutation {
    #[arguments(input: $input)]
    move_tag: Tag,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "CreateTagPathVariables")]
struct CreateTagPathMutation {
    #[arguments(input: $input)]
    create_tag_path: Vec<Tag>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "IdVariables")]
struct DeleteTagMutation {
    #[arguments(id: $id)]
    delete_tag: DeleteTagResult,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "IdVariables")]
struct PinTagMutation {
    #[arguments(id: $id)]
    pin_tag: Tag,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Mutation", variables = "IdVariables")]
struct UnpinTagMutation {
    #[arguments(id: $id)]
    unpin_tag: Tag,
}

// ---------------------------------------------------------------------------
// Client methods
// ---------------------------------------------------------------------------

fn id(value: String) -> cynic::Id {
    cynic::Id::new(value)
}

impl PritaClient {
    /// Fetch saved articles. A `tag` scopes to that tag's subtree; `first`/`after` paginate.
    pub async fn list_articles(
        &self,
        tag: Option<String>,
        first: Option<i32>,
        after: Option<String>,
    ) -> Result<ArticleConnection, Error> {
        let vars = ArticlesVariables {
            tag: tag.map(id),
            first,
            after,
        };
        Ok(self.run(ArticlesQuery::build(vars)).await?.articles)
    }

    /// Fetch a single article (with its readable text and tags), or `None` if not found.
    pub async fn get_article(&self, article_id: String) -> Result<Option<ArticleDetail>, Error> {
        let vars = IdVariables { id: id(article_id) };
        Ok(self.run(ArticleQuery::build(vars)).await?.article)
    }

    /// Fetch the whole tag tree as a flat list.
    pub async fn list_tags(&self) -> Result<Vec<Tag>, Error> {
        Ok(self.run(TagsQuery::build(())).await?.tags)
    }

    /// Save an article from a URL. `content` optionally supplies pre-fetched HTML.
    pub async fn upload_article(
        &self,
        url: String,
        content: Option<String>,
    ) -> Result<ArticleUploadResult, Error> {
        let vars = UploadVariables {
            input: UploadArticleInput { url, content },
        };
        Ok(self.run(UploadArticleMutation::build(vars)).await?.upload_article)
    }

    /// Delete an article. Returns the article that was removed.
    pub async fn delete_article(&self, article_id: String) -> Result<ArticleSummary, Error> {
        let vars = IdVariables { id: id(article_id) };
        Ok(self.run(DeleteArticleMutation::build(vars)).await?.delete_article)
    }

    /// Set reading progress (0-100) for an article.
    pub async fn save_reading_progress(
        &self,
        article_id: String,
        percent: i32,
        anchor: Option<i32>,
    ) -> Result<bool, Error> {
        let vars = SaveProgressVariables {
            input: SaveReadingProgressInput {
                article_id: id(article_id),
                reading_progress_percent: percent,
                reading_progress_anchor_idx: anchor,
            },
        };
        Ok(self
            .run(SaveReadingProgressMutation::build(vars))
            .await?
            .save_reading_progress)
    }

    /// Add a tag to an article. Returns the article with its updated tags.
    pub async fn tag_article(
        &self,
        article_id: String,
        tag_id: String,
    ) -> Result<ArticleTags, Error> {
        let vars = TagArticleVariables {
            input: TagArticleInput {
                article_id: id(article_id),
                tag_id: id(tag_id),
            },
        };
        Ok(self.run(TagArticleMutation::build(vars)).await?.tag_article)
    }

    /// Remove a tag from an article. Returns the article with its updated tags.
    pub async fn untag_article(
        &self,
        article_id: String,
        tag_id: String,
    ) -> Result<ArticleTags, Error> {
        let vars = UntagArticleVariables {
            input: UntagArticleInput {
                article_id: id(article_id),
                tag_id: id(tag_id),
            },
        };
        Ok(self.run(UntagArticleMutation::build(vars)).await?.untag_article)
    }

    /// Create a tag. Omit `parent` for a root tag.
    pub async fn create_tag(
        &self,
        name: String,
        parent: Option<String>,
        color: Option<TagColor>,
    ) -> Result<Tag, Error> {
        let vars = CreateTagVariables {
            input: CreateTagInput {
                name,
                parent_id: parent.map(id),
                color,
            },
        };
        Ok(self.run(CreateTagMutation::build(vars)).await?.create_tag)
    }

    /// Rename a tag.
    pub async fn rename_tag(&self, tag_id: String, name: String) -> Result<Tag, Error> {
        let vars = RenameTagVariables {
            input: RenameTagInput { id: id(tag_id), name },
        };
        Ok(self.run(RenameTagMutation::build(vars)).await?.rename_tag)
    }

    /// Set or clear a tag's color. `None` clears it.
    pub async fn set_tag_color(
        &self,
        tag_id: String,
        color: Option<TagColor>,
    ) -> Result<Tag, Error> {
        let vars = SetTagColorVariables {
            input: SetTagColorInput { id: id(tag_id), color },
        };
        Ok(self.run(SetTagColorMutation::build(vars)).await?.set_tag_color)
    }

    /// Reparent a tag (its subtree moves with it). `None` moves it to the root.
    pub async fn move_tag(&self, tag_id: String, parent: Option<String>) -> Result<Tag, Error> {
        let vars = MoveTagVariables {
            input: MoveTagInput {
                id: id(tag_id),
                parent_id: parent.map(id),
            },
        };
        Ok(self.run(MoveTagMutation::build(vars)).await?.move_tag)
    }

    /// Delete a tag and its whole subtree. Articles are only untagged, never deleted.
    pub async fn delete_tag(&self, tag_id: String) -> Result<DeleteTagResult, Error> {
        let vars = IdVariables { id: id(tag_id) };
        Ok(self.run(DeleteTagMutation::build(vars)).await?.delete_tag)
    }

    /// Create a whole path of tags at once (an atomic `mkdir -p`).
    pub async fn create_tag_path(&self, segments: Vec<String>) -> Result<Vec<Tag>, Error> {
        let vars = CreateTagPathVariables {
            input: CreateTagPathInput { segments },
        };
        Ok(self.run(CreateTagPathMutation::build(vars)).await?.create_tag_path)
    }

    /// Pin a tag into the navigation.
    pub async fn pin_tag(&self, tag_id: String) -> Result<Tag, Error> {
        let vars = IdVariables { id: id(tag_id) };
        Ok(self.run(PinTagMutation::build(vars)).await?.pin_tag)
    }

    /// Unpin a tag.
    pub async fn unpin_tag(&self, tag_id: String) -> Result<Tag, Error> {
        let vars = IdVariables { id: id(tag_id) };
        Ok(self.run(UnpinTagMutation::build(vars)).await?.unpin_tag)
    }
}
