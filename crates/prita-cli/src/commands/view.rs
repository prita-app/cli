//! Shared output views: plain-Rust structs the commands serialize (JSON) or
//! render for humans (`--plain`), converted from the client's GraphQL types.

use std::collections::HashMap;

use prita_client::graphql::{ArticleDetail, ArticleSummary, Tag};
use serde::Serialize;

use crate::output::Render;

#[derive(Serialize)]
pub struct TagView {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub parent_id: Option<String>,
    pub kind: String,
    pub pinned: bool,
}

impl From<Tag> for TagView {
    fn from(tag: Tag) -> Self {
        TagView {
            id: tag.id.into_inner(),
            name: tag.name,
            color: tag.color.map(|c| c.as_str().to_string()),
            parent_id: tag.parent_id.map(|i| i.into_inner()),
            kind: tag.kind.as_str().to_string(),
            pinned: tag.pinned_at.is_some(),
        }
    }
}

impl Render for TagView {
    fn plain(&self) -> String {
        tag_line(self)
    }
}

fn tag_line(tag: &TagView) -> String {
    let color = tag
        .color
        .as_deref()
        .map(|c| format!(" [{c}]"))
        .unwrap_or_default();
    let pin = if tag.pinned { " *" } else { "" };
    format!("{}  {}{color}{pin}", tag.name, tag.id)
}

#[derive(Serialize)]
pub struct TagList {
    pub tags: Vec<TagView>,
}

impl From<Vec<Tag>> for TagList {
    fn from(tags: Vec<Tag>) -> Self {
        TagList {
            tags: tags.into_iter().map(TagView::from).collect(),
        }
    }
}

impl Render for TagList {
    fn plain(&self) -> String {
        if self.tags.is_empty() {
            return "No tags.".to_string();
        }
        render_tree(&self.tags)
    }
}

/// Render the flat tag list as an indented tree, siblings alphabetized.
fn render_tree(tags: &[TagView]) -> String {
    let mut children: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, tag) in tags.iter().enumerate() {
        children
            .entry(tag.parent_id.clone().unwrap_or_default())
            .or_default()
            .push(i);
    }
    for group in children.values_mut() {
        group.sort_by(|&a, &b| tags[a].name.to_lowercase().cmp(&tags[b].name.to_lowercase()));
    }

    let mut out = String::new();
    let mut stack: Vec<(usize, usize)> = Vec::new();
    if let Some(roots) = children.get("") {
        for &i in roots.iter().rev() {
            stack.push((i, 0));
        }
    }
    while let Some((i, depth)) = stack.pop() {
        let indent = "  ".repeat(depth);
        out.push_str(&indent);
        out.push_str(&tag_line(&tags[i]));
        out.push('\n');
        if let Some(kids) = children.get(&tags[i].id) {
            for &k in kids.iter().rev() {
                stack.push((k, depth + 1));
            }
        }
    }
    out.trim_end().to_string()
}

#[derive(Serialize)]
pub struct ArticleView {
    pub id: String,
    pub title: String,
    pub url: String,
    pub site_name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub reading_progress_percent: Option<i32>,
}

impl From<ArticleSummary> for ArticleView {
    fn from(article: ArticleSummary) -> Self {
        ArticleView {
            id: article.id.into_inner(),
            title: article.title,
            url: article.url,
            site_name: article.site_name,
            author: article.author,
            description: article.description,
            reading_progress_percent: article.reading_progress_percent,
        }
    }
}

impl Render for ArticleView {
    fn plain(&self) -> String {
        article_lines(self)
    }
}

/// Two lines: "id  title [%]" then an indented "site · url".
pub fn article_lines(article: &ArticleView) -> String {
    let progress = match article.reading_progress_percent {
        Some(p) if p > 0 => format!(" [{p}%]"),
        _ => String::new(),
    };
    let mut out = format!("{}  {}{progress}", article.id, article.title);

    let site = article.site_name.as_deref().unwrap_or_default();
    match (site.is_empty(), &article.url) {
        (true, url) if url.is_empty() => {}
        (true, url) => out.push_str(&format!("\n    {url}")),
        (false, url) if url.is_empty() => out.push_str(&format!("\n    {site}")),
        (false, url) => out.push_str(&format!("\n    {site} · {url}")),
    }
    out
}

#[derive(Serialize)]
pub struct ArticleList {
    pub articles: Vec<ArticleView>,
    pub has_next_page: bool,
    pub end_cursor: Option<String>,
}

impl Render for ArticleList {
    fn plain(&self) -> String {
        if self.articles.is_empty() {
            return "No articles.".to_string();
        }
        let mut out = String::new();
        for article in &self.articles {
            out.push_str(&article_lines(article));
            out.push('\n');
        }
        if self.has_next_page
            && let Some(cursor) = &self.end_cursor
        {
            out.push_str(&format!("\nmore available: --after {cursor}\n"));
        }
        out.trim_end().to_string()
    }
}

#[derive(Serialize)]
pub struct ArticleDetailView {
    pub id: String,
    pub title: String,
    pub url: String,
    pub site_name: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub published_time: Option<String>,
    pub captured_at: String,
    pub reading_progress_percent: Option<i32>,
    pub read_at: Option<String>,
    pub tags: Vec<TagView>,
    pub text: String,
}

impl From<ArticleDetail> for ArticleDetailView {
    fn from(article: ArticleDetail) -> Self {
        ArticleDetailView {
            id: article.id.into_inner(),
            title: article.title,
            url: article.url,
            site_name: article.site_name,
            author: article.author,
            description: article.description,
            published_time: article.published_time,
            captured_at: article.captured_at,
            reading_progress_percent: article.reading_progress_percent,
            read_at: article.read_at,
            tags: article.tags.into_iter().map(TagView::from).collect(),
            text: article.text_content,
        }
    }
}

impl Render for ArticleDetailView {
    fn plain(&self) -> String {
        let mut out = format!("{}\n{}\n", self.title, self.url);
        if let Some(author) = &self.author {
            out.push_str(&format!("by {author}\n"));
        }
        if !self.tags.is_empty() {
            let names: Vec<&str> = self.tags.iter().map(|t| t.name.as_str()).collect();
            out.push_str(&format!("tags: {}\n", names.join(", ")));
        }
        if let Some(p) = self.reading_progress_percent {
            out.push_str(&format!("progress: {p}%\n"));
        }
        out.push('\n');
        out.push_str(&self.text);
        out
    }
}
