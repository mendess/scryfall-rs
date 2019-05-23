use crate::card::{color::Colors, rarity::Rarity};

use std::collections::HashMap;

pub trait Search {
    fn to_query(&self) -> String;
}

impl Search for &str {
    fn to_query(&self) -> String {
        use percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
        format!("q={}", percent_encode(self.as_bytes(), DEFAULT_ENCODE_SET))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UniqueStrategy {
    Cards,
    Arts,
    Prints,
}

impl Default for UniqueStrategy {
    fn default() -> Self {
        UniqueStrategy::Cards
    }
}

impl std::fmt::Display for UniqueStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use UniqueStrategy::*;
        write!(
            f,
            "unique={}",
            match self {
                Cards => "cards",
                Arts => "art",
                Prints => "prints",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortMethod {
    Name,
    Set,
    Released,
    Rarity,
    Color,
    Usd,
    Tix,
    Eur,
    Cmc,
    Power,
    Toughness,
    Edhrec,
    Artist,
}

impl Default for SortMethod {
    fn default() -> Self {
        SortMethod::Name
    }
}

impl std::fmt::Display for SortMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SortMethod::*;
        write!(
            f,
            "order={}",
            match self {
                Name => "name",
                Set => "set",
                Released => "released",
                Rarity => "rarity",
                Color => "color",
                Usd => "usd",
                Tix => "tix",
                Eur => "eur",
                Cmc => "cmc",
                Power => "power",
                Toughness => "toughness",
                Edhrec => "edhrec",
                Artist => "artist",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortDirection {
    Auto,
    Ascending,
    Descending,
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::Auto
    }
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use SortDirection::*;
        write!(
            f,
            "dir={}",
            match self {
                Auto => "auto",
                Ascending => "asc",
                Descending => "desc",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComparisonExpr {
    AtLeast,
    AtLeastInclusive,
    AtMost,
    AtMostInclusive,
    Is,
    IsNot,
}

#[derive(Debug, Default, Clone)]
pub struct SearchBuilder {
    unique_strategy: UniqueStrategy,
    sort_by: SortMethod,
    sort_direction: SortDirection,
    include_extras: bool,
    include_multilingual: bool,
    include_variations: bool,
    page: usize,
    colors: HashMap<ComparisonExpr, Colors>,
    color_identity: HashMap<ComparisonExpr, Colors>,
    indicator: bool,
    with_types: Vec<String>,
    without_types: Vec<String>,
    oracle_texts: Vec<(String, bool)>,
    is_pyrexian: bool,
    is_hybrid: bool,
    mana_cost: Option<String>,
    cmc: Option<u8>,
    power: Option<(String, ComparisonExpr)>,
    toughness: Option<(String, ComparisonExpr)>,
    loyalty: Option<(String, ComparisonExpr)>,
    split: bool,
    flip: bool,
    transform: bool,
    meld: bool,
    leveler: bool,
    spell: bool,
    permanent: bool,
    historic: bool,
    modal: bool,
    vanilla: bool,
    funny: bool,
    rarity: Option<(Rarity, ComparisonExpr)>,
    new_rarity: Option<(Rarity, ComparisonExpr)>,
}

#[allow(dead_code)]
impl SearchBuilder {
    pub fn unique_strategy(&mut self, strat: UniqueStrategy) -> &mut Self {
        self.unique_strategy = strat;
        self
    }

    pub fn sort_by(&mut self, sort_method: SortMethod) -> &mut Self {
        self.sort_by = sort_method;
        self
    }

    pub fn sort_direction(&mut self, sort_direction: SortDirection) -> &mut Self {
        self.sort_direction = sort_direction;
        self
    }

    pub fn with_extras(&mut self) -> &mut Self {
        self.include_extras = true;
        self
    }

    pub fn with_multilingual(&mut self) -> &mut Self {
        self.include_variations = true;
        self
    }

    pub fn with_colors(&mut self, colors: Colors, comp_expr: ComparisonExpr) -> &mut Self {
        self.colors.insert(comp_expr, colors);
        self
    }

    pub fn with_color_identity(&mut self, colors: Colors, comp_expr: ComparisonExpr) -> &mut Self {
        self.color_identity.insert(comp_expr, colors);
        self
    }

    pub fn with_indicator(&mut self) -> &mut Self {
        self.indicator = true;
        self
    }

    pub fn with_type(&mut self, type_name: String) -> &mut Self {
        self.with_types.push(type_name);
        self
    }

    pub fn with_types(&mut self, type_names: &mut Vec<String>) -> &mut Self {
        self.with_types.append(type_names);
        self
    }

    pub fn without_types(&mut self, type_names: &mut Vec<String>) -> &mut Self {
        self.without_types.append(type_names);
        self
    }

    pub fn with_oracle(&mut self, oracle_text: String) -> &mut Self {
        self.oracle_texts.push((oracle_text, false));
        self
    }

    pub fn with_oracle_full(&mut self, oracle_text: String) -> &mut Self {
        self.oracle_texts.push((oracle_text, true));
        self
    }

    pub fn with_mana_cost(&mut self, cost: String) -> &mut Self {
        self.mana_cost = Some(cost);
        self
    }

    pub fn with_cmc(&mut self, cmc: u8) -> &mut Self {
        self.cmc = Some(cmc);
        self
    }

    pub fn with_power(&mut self, power: String, comp_expr: ComparisonExpr) -> &mut Self {
        self.power = Some((power, comp_expr));
        self
    }

    pub fn with_toughness(&mut self, toughness: String, comp_expr: ComparisonExpr) -> &mut Self {
        self.toughness = Some((toughness, comp_expr));
        self
    }

    pub fn with_loyalty(&mut self, loyalty: String, comp_expr: ComparisonExpr) -> &mut Self {
        self.loyalty = Some((loyalty, comp_expr));
        self
    }

    pub fn and_is_split(&mut self) -> &mut Self {
        self.split = true;
        self
    }

    pub fn and_is_flip(&mut self) -> &mut Self {
        self.flip = true;
        self
    }

    pub fn and_is_transform(&mut self) -> &mut Self {
        self.transform = true;
        self
    }

    pub fn and_is_meld(&mut self) -> &mut Self {
        self.meld = true;
        self
    }

    pub fn and_is_leveler(&mut self) -> &mut Self {
        self.leveler = true;
        self
    }

    pub fn and_is_funny(&mut self) -> &mut Self {
        self.funny = true;
        self
    }

    pub fn with_rarity(&mut self, rarity: Rarity, comp_expr: ComparisonExpr) -> &mut Self {
        self.rarity = Some((rarity, comp_expr));
        self
    }

    pub fn with_new_rarity(&mut self, rarity: Rarity, comp_expr: ComparisonExpr) -> &mut Self {
        self.new_rarity = Some((rarity, comp_expr));
        self
    }
}

impl Search for SearchBuilder {
    fn to_query(&self) -> String {
        let mut query = format!(
            "{}&{}&{}",
            self.unique_strategy, self.sort_by, self.sort_direction
        );
        if self.include_extras {
            query += "include_extras=true";
        }
        if self.include_multilingual {
            query += "include_variations=true";
        }
        if self.include_variations {
            query += "include_multilingual=true";
        }
        if self.page > 0 {
            query += &format!("page={}", self.page + 1);
        }
        query
    }
}
