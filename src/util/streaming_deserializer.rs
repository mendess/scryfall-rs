use std::fmt;
use std::marker::Send;

use futures::Stream;
use serde::de::DeserializeOwned;
use serde::{de::Visitor, Deserialize, Deserializer};
use tokio::io::AsyncRead;
use tokio::sync::mpsc::{channel, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::io::SyncIoBridge;

use crate::Error;

pub fn create<Value, R>(reader: R) -> impl Stream<Item = Result<Value, Error>>
where
    Value: DeserializeOwned + Send + 'static,
    R: AsyncRead + Unpin + Send + 'static,
{
    struct ItemVisitor<V> {
        sender: Sender<Result<V, Error>>,
    }

    impl<'de, V: Deserialize<'de>> Visitor<'de> for ItemVisitor<V> {
        type Value = ();

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("seq of items")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            loop {
                let result = seq.next_element();
                match result {
                    Ok(Some(v)) => {
                        if self.sender.blocking_send(Ok(v)).is_err() {
                            break;
                        }
                    },
                    Ok(None) => break,
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }
    }

    let (sender, receiver) = channel::<Result<Value, Error>>(50);

    let sync_reader = SyncIoBridge::new(reader);
    tokio::task::spawn_blocking(move || {
        let mut deserializer = serde_json::Deserializer::from_reader(sync_reader);
        if let Err(e) = deserializer.deserialize_seq(ItemVisitor::<Value> {
            sender: sender.clone(),
        }) {
            let _ = sender.send(Err(Error::JsonError(e))); //let _ = because error from calling send just means receiver has disconnected
        }
    });

    ReceiverStream::new(receiver)
}
