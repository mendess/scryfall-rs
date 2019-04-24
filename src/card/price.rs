use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Price {
    usd: Option<String>,
    usd_foil: Option<String>,
    eur: Option<String>,
    tix: Option<String>,
}
