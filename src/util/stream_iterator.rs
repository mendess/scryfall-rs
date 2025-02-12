use std::{
    fmt,
    io::BufReader,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread,
};

use serde::{de::Visitor, Deserialize, Deserializer};

use crate::Error;

pub fn create<'de, Value: 'static + Deserialize<'de> + std::marker::Send>(
    reader: BufReader<impl std::io::Read + Send + 'static>,
) -> impl Iterator<Item = Result<Value, Error>> {
    struct ItemVisitor<V> {
        sender: SyncSender<Result<V, Error>>,
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
                        if self.sender.send(Ok(v)).is_err() {
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

    let (sender, receiver) = sync_channel::<Result<Value, Error>>(0);

    thread::spawn(move || {
        let mut deserializer = serde_json::Deserializer::from_reader(reader);
        if let Err(e) = deserializer.deserialize_seq(ItemVisitor::<Value> {
            sender: sender.clone(),
        }) {
            let _ = sender.send(Err(Error::JsonError(e))); //let _ = because error from calling send just means receiver has disconnected
        }
    });

    struct ItemIterator<A> {
        receiver: Receiver<Result<A, Error>>,
    }

    impl<A> Iterator for ItemIterator<A> {
        type Item = Result<A, Error>;

        fn next(&mut self) -> Option<Self::Item> {
            self.receiver.recv().ok()
        }
    }

    Box::new(ItemIterator { receiver })
}
