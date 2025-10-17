use std::collections::{hash_map::Entry, HashMap};

use scryfall::bulk;
use tokio_stream::StreamExt as _;

#[tokio::main]
async fn main() {
    let cards = bulk::all_cards()
        .await
        .unwrap()
        .map(Result::unwrap)
        .collect::<Vec<_>>()
        .await;

    let mut seen = HashMap::new();
    for c in cards {
        macro_rules! extract {
            ($card:ident, $field:ident) => {
                $card.$field.as_ref().map(|_| stringify!($field))
            };
        }
        match seen.entry((
            c.image_status,
            c.image_uris.as_ref().map(|c| {
                extract!(c, png)
                    .into_iter()
                    .chain(extract!(c, border_crop))
                    .chain(extract!(c, art_crop))
                    .chain(extract!(c, large))
                    .chain(extract!(c, normal))
                    .chain(extract!(c, small))
                    .collect::<Vec<_>>()
            }),
        )) {
            Entry::Vacant(slot) => {
                slot.insert(1);
                println!(
                    "{:?} => {}",
                    c.image_status,
                    serde_json::to_string_pretty(&c.image_uris).unwrap()
                )
            },
            Entry::Occupied(mut v) => {
                *v.get_mut() += 1;
                if matches!(c.image_status, scryfall::card::ImageStatus::Missing)
                    && c.image_uris.is_some()
                {
                    println!(
                        "{:?} => {}",
                        c.image_status,
                        serde_json::to_string_pretty(&c.image_uris).unwrap()
                    )
                }
            },
        }
    }
    for ((status, fields), count) in seen {
        println!("{status:?} | {fields:?} | {count}");
    }
}
