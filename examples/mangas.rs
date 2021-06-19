use mangadex::api::manga::*;
use mangadex::schema::manga::*;
use mangadex::schema::LanguageCode;
use mangadex::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();

    let list_manga = ListMangaBuilder::default()
        .add_status(MangaStatus::Ongoing)
        .build()?;

    let mangas = list_manga.send(&client).await?;

    for manga_result in &mangas.results {
        if let Ok(manga) = &manga_result {
            let english_title = manga
                .data
                .attributes
                .title
                .get(&LanguageCode::English)
                .unwrap();
            println!("Got manga {} with id: {:x}", english_title, manga.data.id);
        }
    }

    Ok(())
}
