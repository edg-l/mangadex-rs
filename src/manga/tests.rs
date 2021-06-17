#![cfg(test)]

use crate::{Client, ResourceType};

use super::*;
use chrono::prelude::*;
use isolanguage_1::LanguageCode;
use pretty_assertions::assert_eq;

#[tokio::test]
async fn list_manga() {
    let client = Client::default();
    let query = MangaQuery::default();
    let manga = ListManga { query: &query }.send(&client).await.unwrap();
    assert_eq!(manga.offset, 0);
    assert_eq!(manga.limit, 10);
}

#[tokio::test]
async fn view_manga() {
    let id = Uuid::parse_str("32d76d19-8a05-4db0-9fc2-e0b0648fe9d0").unwrap();
    let client = Client::default();
    let manga_result = ViewManga { id: &id }.send(&client).await.unwrap();

    let manga = manga_result.data;
    assert_eq!(manga.id, id);
    assert_eq!(
        manga
            .attributes
            .title
            .get(&LanguageCode::En)
            .map(String::as_str),
        Some("Solo Leveling")
    );
    assert_eq!(manga.attributes.original_language.as_str(), "ko");
    // 2019-08-25T10:51:55+00:00
    assert_eq!(
        manga.attributes.created_at,
        Utc.ymd(2019, 8, 25).and_hms(10, 51, 55)
    );
}

#[tokio::test]
async fn random_manga() {
    let client = Client::default();
    let manga_result = RandomManga.send(&client).await.unwrap();
    let manga = manga_result.data;
    assert_eq!(manga.r#type, ResourceType::Manga);
}

#[tokio::test]
async fn tag_list() {
    let client = Client::default();
    let tag_results = TagList.send(&client).await.unwrap();

    for result in &tag_results {
        let tag = &result.as_ref().unwrap().data;
        assert_eq!(tag.r#type, ResourceType::Tag);
    }
}
