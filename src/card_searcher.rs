#![allow(dead_code)]
use crate::card::{color::Colors, rarity::Rarity};

use std::fmt::Write;

use percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};

pub trait Search {
    fn to_query(&self) -> String;
}

impl Search for &str {
    fn to_query(&self) -> String {
        format!("q={}", percent_encode(self.as_bytes(), DEFAULT_ENCODE_SET))
    }
}

pub trait Param {
    fn to_param(&self) -> String;
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

impl Param for UniqueStrategy {
    fn to_param(&self) -> String {
        use UniqueStrategy::*;
        String::from("unique=")
            + match self {
                Cards => "cards",
                Arts => "art",
                Prints => "prints",
            }
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

impl Param for SortMethod {
    fn to_param(&self) -> String {
        use SortMethod::*;
        String::from("order=")
            + match self {
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

impl Param for SortDirection {
    fn to_param(&self) -> String {
        use SortDirection::*;
        format!(
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
pub enum BooleanParam {
    IncludeExtras,
    IncludeMultilingual,
    IncludeVaraitions,
    ColorIndicator,
    NewRarity,
    IsPhyrexian,
    IsHybrid,
    IsSplit,
    IsFlip,
    IsTransform,
    IsMeld,
    IsLeveler,
    IsSpell,
    IsPermanent,
    IsHistoric,
    IsModal,
    IsVanilla,
    IsFunny,
}

impl Param for BooleanParam {
    fn to_param(&self) -> String {
        use BooleanParam::*;
        format!(
            "{}:{}=true",
            match self {
                IncludeExtras | IncludeMultilingual | IncludeVaraitions => "include",
                ColorIndicator => "has",
                NewRarity => "new",
                _ => "is",
            },
            match self {
                IncludeExtras => "extras",
                IncludeMultilingual => "multilingual",
                IncludeVaraitions => "variations",
                ColorIndicator => "indicator",
                NewRarity => "rarity",
                IsPhyrexian => "phyrexian",
                IsHybrid => "hybrid",
                IsSplit => "split",
                IsFlip => "flip",
                IsTransform => "transform",
                IsMeld => "meld",
                IsLeveler => "leveler",
                IsSpell => "spell",
                IsPermanent => "permanent",
                IsHistoric => "historic",
                IsModal => "modal",
                IsVanilla => "vanilla",
                IsFunny => "funny",
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

impl std::fmt::Display for ComparisonExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ComparisonExpr::*;
        write!(
            f,
            "{}",
            match self {
                AtLeast => ">",
                AtLeastInclusive => ">=",
                AtMost => "<",
                AtMostInclusive => "<=",
                Is => "=",
                IsNot => "!=",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StringParam {
    ManaCost(String),
    Type(String),
    NotType(String),
    Oracle(String),
    OracleFull(String),
    Power(String, ComparisonExpr),
    Toughness(String, ComparisonExpr),
    Loyalty(String, ComparisonExpr),
}

impl Param for StringParam {
    fn to_param(&self) -> String {
        use StringParam::*;
        match self {
            ManaCost(s) => format!("s:{}", s),
            Type(s) => format!("t:{}", s),
            NotType(s) => format!("-t:{}", s),
            Oracle(s) => format!("o:{}", s),
            OracleFull(s) => format!("fo:{}", s),
            Power(s, c) => format!("pow{}{}", c, s),
            Toughness(s, c) => format!("tou{}{}", c, s),
            Loyalty(s, c) => format!("loy{}{}", c, s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumericParam {
    Page(usize),
    CMC(usize),
}

impl Param for NumericParam {
    fn to_param(&self) -> String {
        use NumericParam::*;
        match self {
            Page(p) => format!("page={}", p),
            CMC(p) => format!("cmc={}", p),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RarityParam {
    rarity: Rarity,
    comp_expr: ComparisonExpr,
}

impl RarityParam {
    pub fn rarity(rarity: Rarity, comp_expr: ComparisonExpr) -> Self {
        RarityParam { rarity, comp_expr }
    }
}

impl Param for RarityParam {
    fn to_param(&self) -> String {
        format!(
            "r{}{}",
            self.comp_expr,
            match self.rarity {
                Rarity::Common => "c",
                Rarity::Uncommon => "u",
                Rarity::Rare => "r",
                Rarity::Mythic => "m",
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorParam {
    Color(Colors, ComparisonExpr),
    ColorIdentity(Colors, ComparisonExpr),
}

impl ColorParam {
    fn new_colors(colors: Colors, comp_expr: ComparisonExpr) -> Self {
        ColorParam::Color(colors, comp_expr)
    }

    fn new_color_identity(colors: Colors, comp_expr: ComparisonExpr) -> Self {
        ColorParam::ColorIdentity(colors, comp_expr)
    }
}

impl Param for ColorParam {
    fn to_param(&self) -> String {
        use ColorParam::*;
        match self {
            Color(cl, ce) => format!("c{}{}", cl, ce),
            ColorIdentity(cl, ce) => format!("id{}{}", cl, ce),
        }
    }
}

pub struct SearchBuilder {
    unique: UniqueStrategy,
    order: SortMethod,
    dir: SortDirection,
    page: usize,
    include_extras: bool,
    include_multilingual: bool,
    include_variations: bool,
    params: Vec<Box<dyn Param>>,
}

impl SearchBuilder {
    fn new() -> Self {
        SearchBuilder {
            page: 1,
            unique: Default::default(),
            order: Default::default(),
            dir: Default::default(),
            include_extras: false,
            include_multilingual: false,
            include_variations: false,
            params: vec![],
        }
    }
    pub fn with_unique_strategy(&mut self, strat: UniqueStrategy) -> &mut Self {
        self.unique = strat;
        self
    }

    pub fn with_sort_order(&mut self, strat: SortMethod) -> &mut Self {
        self.order = strat;
        self
    }

    pub fn with_sort_direction(&mut self, dir: SortDirection) -> &mut Self {
        self.dir = dir;
        self
    }

    pub fn include_extras(&mut self) -> &mut Self {
        self.include_extras = true;
        self
    }

    pub fn include_multilingual(&mut self) -> &mut Self {
        self.include_multilingual = true;
        self
    }

    pub fn include_variations(&mut self) -> &mut Self {
        self.include_variations = true;
        self
    }

    pub fn on_page(&mut self, page: usize) -> &mut Self {
        self.page = page;
        self
    }

    pub fn add_param(&mut self, param: Box<dyn Param>) -> &mut Self {
        self.params.push(param);
        self
    }
}

impl Search for SearchBuilder {
    fn to_query(&self) -> String {
        use itertools::Itertools;
        let mut query = format!(
            "{}&{}&{}",
            self.unique.to_param(),
            self.order.to_param(),
            self.dir.to_param()
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
        if self.page > 1 {
            query += &format!("page={}", self.page + 1);
        }
        query += "q=";
        let _ = write!(
            query,
            "{}",
            percent_encode(
                self.params
                    .iter()
                    .map(|x| {
                        #[allow(clippy::redundant_closure)]
                        x.to_param()
                    })
                    .join("+")
                    .as_bytes(),
                DEFAULT_ENCODE_SET,
            )
        );
        query
    }
}
