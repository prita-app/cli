//! Tag commands: list the tree, plus create/rename/move/color/delete/path/pin/unpin.

use prita_client::graphql::TagColor;
use serde::Serialize;

use crate::cli::{TagsArgs, TagsCommand};
use crate::commands::view::{TagList, TagView};
use crate::error::CliError;
use crate::output::{Format, Render, emit};

pub async fn run(args: TagsArgs, format: Format) -> Result<(), CliError> {
    match args.command {
        None | Some(TagsCommand::List) => list(format).await,
        Some(TagsCommand::Create {
            name,
            parent,
            color,
        }) => create(name, parent, color, format).await,
        Some(TagsCommand::Rename { id, name }) => rename(id, name, format).await,
        Some(TagsCommand::Move { id, parent }) => move_tag(id, parent, format).await,
        Some(TagsCommand::Color { id, color }) => set_color(id, color, format).await,
        Some(TagsCommand::Delete { id }) => delete(id, format).await,
        Some(TagsCommand::Path { segments }) => path(segments, format).await,
        Some(TagsCommand::Pin { id }) => pin(id, format).await,
        Some(TagsCommand::Unpin { id }) => unpin(id, format).await,
    }
}

async fn list(format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let tags = client.list_tags().await?;
    emit(format, &TagList::from(tags));
    Ok(())
}

async fn create(
    name: String,
    parent: Option<String>,
    color: Option<String>,
    format: Format,
) -> Result<(), CliError> {
    let color = color.as_deref().map(parse_color).transpose()?;
    let client = super::client()?;
    let tag = client.create_tag(name, parent, color).await?;
    emit(format, &TagView::from(tag));
    Ok(())
}

async fn rename(id: String, name: String, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let tag = client.rename_tag(id, name).await?;
    emit(format, &TagView::from(tag));
    Ok(())
}

async fn move_tag(id: String, parent: Option<String>, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let tag = client.move_tag(id, parent).await?;
    emit(format, &TagView::from(tag));
    Ok(())
}

async fn set_color(id: String, color: String, format: Format) -> Result<(), CliError> {
    let color = if color.eq_ignore_ascii_case("none") {
        None
    } else {
        Some(parse_color(&color)?)
    };
    let client = super::client()?;
    let tag = client.set_tag_color(id, color).await?;
    emit(format, &TagView::from(tag));
    Ok(())
}

async fn delete(id: String, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let result = client.delete_tag(id).await?;
    emit(
        format,
        &DeleteResult {
            deleted_count: result.deleted_count,
            deleted_tag_ids: result
                .deleted_tag_ids
                .into_iter()
                .map(|i| i.into_inner())
                .collect(),
        },
    );
    Ok(())
}

async fn path(segments: Vec<String>, format: Format) -> Result<(), CliError> {
    if segments.is_empty() {
        return Err(CliError::new("empty_path", "provide at least one path segment"));
    }
    let client = super::client()?;
    let tags = client.create_tag_path(segments).await?;
    emit(format, &TagList::from(tags));
    Ok(())
}

async fn pin(id: String, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let tag = client.pin_tag(id).await?;
    emit(format, &TagView::from(tag));
    Ok(())
}

async fn unpin(id: String, format: Format) -> Result<(), CliError> {
    let client = super::client()?;
    let tag = client.unpin_tag(id).await?;
    emit(format, &TagView::from(tag));
    Ok(())
}

fn parse_color(name: &str) -> Result<TagColor, CliError> {
    TagColor::parse(name).ok_or_else(|| {
        CliError::new(
            "invalid_color",
            format!("unknown color `{name}` (valid: {})", TagColor::NAMES.join(", ")),
        )
    })
}

#[derive(Serialize)]
struct DeleteResult {
    deleted_count: i32,
    deleted_tag_ids: Vec<String>,
}

impl Render for DeleteResult {
    fn plain(&self) -> String {
        format!("Deleted {} tag(s).", self.deleted_count)
    }
}
