use std::path::PathBuf;

use async_recursion::async_recursion;
use audiotags::{Tag, TagType};
use tokio::fs;

pub async fn copy_tags(from: &PathBuf, to: &PathBuf, is_mp3: bool) -> anyhow::Result<()> {
    let mut from_tag = Tag::default().read_from_path(from)?;

    if is_mp3 {
        let mut mp3_tags = from_tag.to_dyn_tag(TagType::Id3v2);
        mp3_tags.write_to_path(to.to_str().unwrap())?;
    } else {
        from_tag.write_to_path(to.to_str().unwrap())?;
    }

    return Ok(());
}

#[async_recursion]
pub async fn valid_tags(flac_dir_path: &PathBuf) -> anyhow::Result<bool> {
    let mut dir = fs::read_dir(flac_dir_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            if valid_tags(&path).await? {
                return Ok(true);
            }
        } else {
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if file_name.ends_with(".flac") {
                if !validate_tags_of_file(path) {
                    return Ok(false);
                }
            }
        }
    }

    return Ok(true);
}

pub fn validate_tags_of_file(path: PathBuf) -> bool {
    let tag = Tag::new().read_from_path(&path).unwrap();

    if tag.artist().is_none() {
        return false;
    }

    if tag.album().is_none() {
        return false;
    }

    if tag.title().is_none() {
        return false;
    }

    let (track_number, total_tracks) = tag.track();

    if track_number.is_none() || total_tracks.is_none() {
        return false;
    }

    return true;
}