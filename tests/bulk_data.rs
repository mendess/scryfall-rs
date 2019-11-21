use scryfall::card::Card;

#[test]
fn parse_bulk_data() {
    let cards: Vec<Card> =
        reqwest::get("https://archive.scryfall.com/json/scryfall-oracle-cards.json")
            .unwrap()
            .json()
            .unwrap();
    assert!(!cards.is_empty());
}
