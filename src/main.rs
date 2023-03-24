extern crate tokio;
extern crate reqwest;
extern crate error_chain;

use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/todos/1").await?;
    println!("response:\nstatus: {}\nheaders:\n{:#?}", response.status(), response.headers());

    let body = response.text().await?;
    println!("body:\n{}", body);

    Ok(())
}
