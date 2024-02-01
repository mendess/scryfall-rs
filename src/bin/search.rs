use std::process::ExitCode;

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut argv = std::env::args().skip(1);
    let card = match argv.next().as_deref() {
        Some("-i") => {
            scryfall::Card::scryfall_id(
                argv.next()
                    .expect("missing scryfall_id argument")
                    .parse()
                    .expect("invalid uuid"),
            )
            .await?
        },
        Some("-n") => {
            scryfall::Card::named(&argv.next().expect("missing scryfall_id argument")).await?
        },
        Some(opt) => return Err(format!("invalid option {opt}").into()),
        None => scryfall::Card::random().await?,
    };

    println!("{card:#?}");
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(e) = run().await {
        eprintln!("{e:#?}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
