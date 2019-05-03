use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Price {
    usd: Option<String>,
    usd_foil: Option<String>,
    eur: Option<String>,
    tix: Option<String>,
}
