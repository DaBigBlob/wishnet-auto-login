use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let params = [("username", "sardar_dsl"), ("password", "wnpl123")];// replace sardar_dsl and wnpl123 with your own stuf
    let client = reqwest::Client::new();
    let response = client.post("http://2.2.2.2/login")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?
        .text()
        .await?;
    let line_arr = response.split('\n').collect::<Vec<&str>>();
    println!("LOGGEDIN: \t{}", line_arr[12].replace("<input type=\"hidden\" name=\"logged-in\" value=", "").replace("/>", ""));
    println!("ERROR \t\t{}", line_arr[13].replace("<input type=\"hidden\" name=\"error-orig\" value=", "").replace("/>", ""));
    Ok(())
}
