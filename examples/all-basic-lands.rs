use futures::StreamExt;
use scryfall::{
    search::advanced::{SearchOptions, UniqueStrategy},
    search::prelude::*,
    Card,
};

#[tokio::main]
async fn main() -> scryfall::Result<()> {
    let opts = SearchOptions::new()
        .query(type_line("Basic").and(type_line("Land")))
        .unique(UniqueStrategy::Prints)
        .extras(true)
        .variations(true)
        .multilingual(true);

    let mut cards = Card::search(opts).await?.into_stream();
    while let Some(card) = cards.next().await {
        let card = card.expect("card should be deserialized succesfully");
        println!("{} - {}", card.id, card.printed_name.unwrap_or(card.name));
    }

    Ok(())
}
