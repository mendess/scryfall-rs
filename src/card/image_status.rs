use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(missing_docs)]
pub enum ImageStatus {
    Missing,
    Placeholder,
    Lowres,
    HighresScan,
}
