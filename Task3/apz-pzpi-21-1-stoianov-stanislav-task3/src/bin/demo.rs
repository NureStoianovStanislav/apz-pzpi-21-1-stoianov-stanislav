use std::io::Write;

use ligma as library;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    library::set_base_url("http://localhost:8080");
    library::init_settings()?;
    loop {
        let input = prompt("Enter your command:");
        match input.as_str() {
            "settings" => {
                let url = prompt("Base URL:");
                library::set_base_url(&url);
            }
            "lend" => {
                let lendee_id = prompt("Who lends?");
                let book_id = prompt("Which book?");
                library::lend_book(&lendee_id, &book_id).await?;
                println!("Happy reading");
            }
            "return" => {
                let book_id = prompt("Which book?");
                library::return_book(&book_id).await?;
                println!("Returned the book successfully");
            }
            "quit" => break,
            unknown => println!("command not found: {unknown}"),
        }
    }
    Ok(())
}

fn prompt(prompt: &str) -> String {
    print!("{prompt} ");
    let mut buf = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
