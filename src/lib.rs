pub mod card;
pub mod error;
pub mod set;
pub mod util;

pub use error::Result;

#[cfg(test)]
mod tests {
    use crate::card;
    use crate::set;

    #[test]
    fn random() {
        assert!(card::Card::random().is_ok())
    }

    #[test]
    fn all_cards() {
        let cards = card::Card::all()
            .take(1)
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();
        assert_eq!(cards.len(), 1)
    }

    #[test]
    fn search() {
        card::Card::search("Jace")
            .map(|x| x.unwrap())
            .for_each(drop);
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

    #[test]
    fn all_sets() {
        set::Set::all().map(|x| x.unwrap()).for_each(drop);
    }
}
