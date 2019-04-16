mod card;

#[cfg(test)]
mod tests {
    use crate::card;

    #[test]
    fn random() {
        let card = card::Card::random();
        println!("{:?}", card);
        assert!(card.is_ok())
    }

    #[test]
    fn all_cards() {
        let card = card::Card::all()
            .take(10)
            .map(|x| { println!("{:?}", x); x })
            .all(|x| x.is_ok());
        assert!(card)
    }

    #[test]
    fn search() {
        let cards = card::Card::search("Jace")
            .map(|x| { println!("{:?}", x); x })
            .all(|x| x.is_ok());
        assert!(cards)
    }

    #[test]
    fn named() {
        assert!(card::Card::named("Lightning Bolt").is_ok())
    }

    #[test]
    fn named_fuzzy() {
        assert!(card::Card::named_fuzzy("Light Bol").is_ok())
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
        assert_eq!(
            card::Card::card("0b81b329-4ef5-4b55-9fe7-9ed69477e96b").unwrap().id,
            "0b81b329-4ef5-4b55-9fe7-9ed69477e96b")
    }
}
