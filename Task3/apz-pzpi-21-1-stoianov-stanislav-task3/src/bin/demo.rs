use std::io::Write;
use ligma as lib;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    lib::set_base_url("http://localhost:8080");
    lib::init_settings()?;
    loop {
        match prompt("Enter your command:").as_str() {
            "settings" => {
                let url = prompt("Base URL:");
                lib::set_base_url(&url);
            }
            "lend" => {
                let lendee_id = prompt("Who lends?");
                let book_id = prompt("Which book?");
                lib::lend_book(&lendee_id, &book_id)
                    .await
                    .inspect_err(|e| println!("Error: {e:?}"))
                    .ok();
                println!("Happy reading");
            }
            "return" => {
                let book_id = prompt("Which book?");
                lib::return_book(&book_id)
                    .await
                    .inspect_err(|e| println!("Error: {e:?}"))
                    .ok();
                println!("Returned the book successfully");
            }
            "quit" => break Ok(()),
            unknown => println!("command not found: {unknown}"),
        }
    }
}

fn prompt(prompt: &str) -> String {
    print!("{prompt} ");
    let mut buf = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}
