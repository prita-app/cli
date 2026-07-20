//! Article commands: list, get, save, rm, tag, untag, progress.

use prita_client::graphql::{ArticleConnection, ArticleTags};
use serde::Serialize;

use crate::cli::{GetArgs, ListArgs, ProgressArgs, RmArgs, SaveArgs, TagArgs};
use crate::commands::view::{self, ArticleDetailView, ArticleList, ArticleView, TagView};
use crate::error::CliError;
use crate::output::{Format, Render, emit};

pub async fn list(args: ListArgs, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let connection = client
        .list_articles(args.tag, Some(args.limit), args.after)
        .await?;
    emit(format, &article_list(connection));
    Ok(())
}

fn article_list(connection: ArticleConnection) -> ArticleList {
    ArticleList {
        has_next_page: connection.page_info.has_next_page,
        end_cursor: connection.page_info.end_cursor,
        articles: connection
            .edges
            .into_iter()
            .map(|edge| ArticleView::from(edge.node))
            .collect(),
    }
}

pub async fn get(args: GetArgs, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    match client.get_article(args.id).await? {
        Some(article) => {
            emit(format, &ArticleDetailView::from(article));
            Ok(())
        }
        None => Err(CliError::new("not_found", "no article with that id").with_exit(3)),
    }
}

pub async fn save(args: SaveArgs, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let result = client.upload_article(args.url, args.content).await?;
    emit(
        format,
        &SaveResult {
            was_already_saved: result.was_already_saved,
            article: ArticleView::from(result.article),
        },
    );
    Ok(())
}

pub async fn rm(args: RmArgs, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let article = client.delete_article(args.id).await?;
    emit(
        format,
        &RmResult {
            deleted: true,
            id: article.id.into_inner(),
            title: article.title,
        },
    );
    Ok(())
}

pub async fn tag(args: TagArgs, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let article = client.tag_article(args.article_id, args.tag_id).await?;
    emit(format, &tag_result(article));
    Ok(())
}

pub async fn untag(args: TagArgs, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let article = client.untag_article(args.article_id, args.tag_id).await?;
    emit(format, &tag_result(article));
    Ok(())
}

fn tag_result(article: ArticleTags) -> TagResult {
    TagResult {
        article_id: article.id.into_inner(),
        title: article.title,
        tags: article.tags.into_iter().map(TagView::from).collect(),
    }
}

pub async fn progress(args: ProgressArgs, format: Format) -> Result<(), CliError> {
    if !(0..=100).contains(&args.percent) {
        return Err(CliError::new(
            "invalid_percent",
            "percent must be between 0 and 100",
        ));
    }
    let client = super::client()?;
    let saved = client
        .save_reading_progress(args.article_id.clone(), args.percent, args.anchor)
        .await?;
    emit(
        format,
        &ProgressResult {
            saved,
            article_id: args.article_id,
            reading_progress_percent: args.percent,
        },
    );
    Ok(())
}

#[derive(Serialize)]
struct SaveResult {
    article: ArticleView,
    was_already_saved: bool,
}

impl Render for SaveResult {
    fn plain(&self) -> String {
        let note = if self.was_already_saved {
            " (already saved)"
        } else {
            ""
        };
        format!("Saved{note}:\n{}", view::article_lines(&self.article))
    }
}

#[derive(Serialize)]
struct RmResult {
    deleted: bool,
    id: String,
    title: String,
}

impl Render for RmResult {
    fn plain(&self) -> String {
        format!("Deleted {} ({}).", self.title, self.id)
    }
}

#[derive(Serialize)]
struct TagResult {
    article_id: String,
    title: String,
    tags: Vec<TagView>,
}

impl Render for TagResult {
    fn plain(&self) -> String {
        let names: Vec<&str> = self.tags.iter().map(|t| t.name.as_str()).collect();
        let tags = if names.is_empty() {
            "(none)".to_string()
        } else {
            names.join(", ")
        };
        format!("{}\ntags: {tags}", self.title)
    }
}

#[derive(Serialize)]
struct ProgressResult {
    saved: bool,
    article_id: String,
    reading_progress_percent: i32,
}

impl Render for ProgressResult {
    fn plain(&self) -> String {
        format!("Progress set to {}%.", self.reading_progress_percent)
    }
}
