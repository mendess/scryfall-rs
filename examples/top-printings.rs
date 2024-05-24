use futures::future;
use futures::stream::StreamExt;
use scryfall::card::Game;
use scryfall::search::prelude::*;
use scryfall::Card;

#[tokio::main]
async fn main() -> scryfall::Result<()> {
    let card_name = std::env::args().nth(1).expect("expected a card name param");

    let search_options = SearchOptions::new()
        .unique(UniqueStrategy::Prints)
        .sort(SortOrder::Usd, SortDirection::Descending)
        .query(exact(card_name).and(in_game(Game::Paper)));

    println!("{}", serde_urlencoded::to_string(&search_options).unwrap());

    let cards: Vec<Card> = search_options
        .search()
        .await?
        .into_stream_buffered(10)
        .filter_map(|card| async move { card.ok() })
        .filter(|card| {
            future::ready(
                card.prices.usd.is_some() || (!card.nonfoil && card.prices.usd_foil.is_some()),
            )
        })
        .collect()
        .await;

    for card in cards {
        println!(
            "{name} | {set:>6} {cn:<4} | {usd}",
            name = card.name,
            set = card.set,
            cn = card.collector_number,
            usd = card
                .prices
                .usd
                .or(card.prices.usd_foil)
                .unwrap_or_else(|| "-".to_string())
        );
    }

    Ok(())
}
