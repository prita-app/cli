//! Typed GraphQL operations, checked against the vendored schema at build time.

use cynic::QueryBuilder;

use crate::PritaClient;
use crate::error::Error;

#[cynic::schema("prita")]
pub mod schema {}

#[derive(cynic::QueryVariables, Debug, Default)]
pub struct ArticlesVariables {
    pub tag: Option<cynic::Id>,
    pub first: Option<i32>,
    pub after: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "ArticlesVariables")]
pub struct ArticlesQuery {
    #[arguments(tag: $tag, first: $first, after: $after)]
    pub articles: ArticleConnection,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ArticleConnection {
    pub page_info: PageInfo,
    pub edges: Vec<ArticleEdge>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct ArticleEdge {
    pub cursor: String,
    pub node: Article,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Article {
    pub id: cynic::Id,
    pub title: Option<String>,
    pub url: Option<String>,
    pub author: Option<String>,
    pub site_name: Option<String>,
    pub description: Option<String>,
    pub captured_at: Option<String>,
    pub reading_progress_percent: Option<i32>,
}

impl PritaClient {
    /// Fetch saved articles. A `tag` scopes the list to that tag's subtree;
    /// `first` and `after` paginate.
    pub async fn list_articles(
        &self,
        tag: Option<String>,
        first: Option<i32>,
        after: Option<String>,
    ) -> Result<ArticleConnection, Error> {
        let vars = ArticlesVariables {
            tag: tag.map(cynic::Id::new),
            first,
            after,
        };
        let data = self.run(ArticlesQuery::build(vars)).await?;
        Ok(data.articles)
    }
}
