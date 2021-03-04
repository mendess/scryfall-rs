use clap::Clap;
use scryfall::card::Game;
use scryfall::card_searcher::{
    GameParam,
    SearchBuilder,
    SortDirection,
    SortMethod,
    UniqueStrategy,
};

#[derive(Clap)]
struct Opts {
    card_name: String,
}

fn main() -> scryfall::Result<()> {
    let opts: Opts = Opts::parse();

    let mut builder = SearchBuilder::new();
    builder
        .with_unique_strategy(UniqueStrategy::Prints)
        .sorting_by(SortMethod::Usd)
        .with_sort_direction(SortDirection::Descending)
        .param(format!("!\"{}\"", opts.card_name))
        .param(GameParam::InGame(Game::Paper));

    println!("{}", serde_urlencoded::to_string(&builder).unwrap());

    let cards = builder
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
