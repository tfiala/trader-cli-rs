use brokerage_messages::domain::BrokerageAccount;

async fn fetch_accounts(server_uri: String) -> anyhow::Result<Vec<BrokerageAccount>> {
    // Simulate fetching accounts from the server
    println!("Fetching accounts from {}", server_uri);

    let accounts = reqwest::get(format!("{}/v1/api/brokerage/accounts", server_uri))
        .await?
        .json::<Vec<BrokerageAccount>>()
        .await?;
    Ok(accounts)
}

pub async fn handle_command(args: crate::Cli, server_uri: String) -> anyhow::Result<()> {
    match args.command {
        crate::Commands::Accounts => {
            println!("Fetching accounts...");
            // Call the function to fetch accounts here
            // fetch_accounts(server_uri).await?;
            let accounts = fetch_accounts(server_uri).await?;
            for account in accounts {
                println!("Account: {:?}", account);
            }
        }
        crate::Commands::EndOfDaySummary(args) => {
            println!("Fetching end of day summary for {} days...", args.days);
            // Call the function to fetch end of day summary here
            // fetch_end_of_day_summary(server_uri, args.days).await?;
        }
    }
    Ok(())
}
