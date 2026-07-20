//! `prita list`: show saved articles.

use prita_client::graphql::ArticleConnection;
use serde::Serialize;

use crate::cli::ListArgs;
use crate::error::CliError;
use crate::output::{Format, Render, emit};

pub async fn run(args: ListArgs, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let connection = client
        .list_articles(args.tag, Some(args.limit), args.after)
        .await?;
    emit(format, &ArticleList::from(connection));
    Ok(())
}

#[derive(Serialize)]
struct ArticleList {
    articles: Vec<ArticleSummary>,
    has_next_page: bool,
    end_cursor: Option<String>,
}

#[derive(Serialize)]
struct ArticleSummary {
    id: String,
    title: Option<String>,
    url: Option<String>,
    site_name: Option<String>,
    author: Option<String>,
    reading_progress_percent: Option<i32>,
    cursor: String,
}

impl From<ArticleConnection> for ArticleList {
    fn from(connection: ArticleConnection) -> Self {
        let articles = connection
            .edges
            .into_iter()
            .map(|edge| ArticleSummary {
                id: edge.node.id.into_inner(),
                title: edge.node.title,
                url: edge.node.url,
                site_name: edge.node.site_name,
                author: edge.node.author,
                reading_progress_percent: edge.node.reading_progress_percent,
                cursor: edge.cursor,
            })
            .collect();
        ArticleList {
            articles,
            has_next_page: connection.page_info.has_next_page,
            end_cursor: connection.page_info.end_cursor,
        }
    }
}

impl Render for ArticleList {
    fn plain(&self) -> String {
        if self.articles.is_empty() {
            return "No articles.".to_string();
        }
        let mut out = String::new();
        for article in &self.articles {
            let title = article.title.as_deref().unwrap_or("(untitled)");
            let progress = article
                .reading_progress_percent
                .map(|p| format!(" [{p}%]"))
                .unwrap_or_default();
            out.push_str(&format!("{}  {title}{progress}\n", article.id));

            let site = article.site_name.as_deref().unwrap_or_default();
            if let Some(url) = &article.url {
                if site.is_empty() {
                    out.push_str(&format!("    {url}\n"));
                } else {
                    out.push_str(&format!("    {site} · {url}\n"));
                }
            } else if !site.is_empty() {
                out.push_str(&format!("    {site}\n"));
            }
        }
        if self.has_next_page
            && let Some(cursor) = &self.end_cursor
        {
            out.push_str(&format!("\nmore available: --after {cursor}\n"));
        }
        out.trim_end().to_string()
    }
}
