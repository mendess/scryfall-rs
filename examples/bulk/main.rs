use futures::StreamExt;

#[tokio::main]
async fn main() -> scryfall::Result<()> {
    let mut stream = scryfall::bulk::all_cards().await?;

    let mut error_count = 0;
    let mut count = 0;

    while let Some(card) = stream.next().await {
        match card {
            Ok(_) => {
                count += 1;
                if count % 5000 == 0 {
                    println!("{count}");
                }
            },
            Err(e) => {
                println!("{:?}", e);
                error_count += 1;
            },
        }
    }

    println!("Found {} cards and {} errors", count, error_count);

    Ok(())
}
