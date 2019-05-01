mod card;
mod card_set;
mod util;

#[cfg(test)]
mod tests {
    use crate::card;

    #[test]
    fn random() {
        assert!(card::Card::random().is_ok())
    }

    #[test]
    fn all_cards() {
        let card = card::Card::all().take(10).collect::<Vec<_>>();
        assert!(card.iter().all(|x| x.is_ok()));
        assert_eq!(card.len(), 10)
    }

    #[test]
    fn search() {
        let cards = card::Card::search("Jace").all(|x| x.is_ok());
        assert!(cards)
    }

    #[test]
    fn named() {
        let card = card::Card::named("Lightning Bolt").unwrap();
        assert_eq!(card.name, "Lightning Bolt")
    }

    #[test]
    fn named_fuzzy() {
        let card = card::Card::named_fuzzy("Light Bol").unwrap();
        assert_eq!(card.name, "Lightning Bolt")
    }

    #[test]
    fn multiverse() {
        assert!(card::Card::multiverse("409574").is_ok())
    }

    #[test]
    fn mtgo() {
        assert!(card::Card::mtgo("54957").is_ok())
    }

    #[test]
    fn arena() {
        assert!(card::Card::arena("67330").is_ok())
    }

    #[test]
    fn tcgplayer() {
        assert!(card::Card::tcgplayer("162145").is_ok())
    }

    #[test]
    fn id() {
        let card = card::Card::card("0b81b329-4ef5-4b55-9fe7-9ed69477e96b").unwrap();
        assert_eq!(card.id, "0b81b329-4ef5-4b55-9fe7-9ed69477e96b")
    }

    #[test]
    fn set() {
        assert!(card::Card::mtgo("54957").unwrap().set_uri.fetch().is_ok())
    }
}
