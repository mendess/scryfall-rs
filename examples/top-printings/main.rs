use scryfall::card::Game;
use scryfall::search::prelude::*;

fn main() -> scryfall::Result<()> {
    let card_name = std::env::args().nth(1).expect("expected a card name param");

    let mut search_options = SearchOptions::new();
    search_options
        .unique(UniqueStrategy::Prints)
        .sort(SortOrder::Usd, SortDirection::Descending)
        .query(exact(card_name).and(in_game(Game::Paper)));

    println!("{}", serde_urlencoded::to_string(&search_options).unwrap());

    let cards = search_options
        .search()?
        .filter_map(|card| card.ok())
        .filter(|card| {
            card.prices.usd.is_some() || (!card.nonfoil && card.prices.usd_foil.is_some())
        });

    for card in cards {
        println!(
            "{name} | {set:>6} {cn:<4} | {usd}",
            name = card.name,
            set = card.set.to_string(),
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
