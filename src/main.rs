#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/<cmd>")]
fn search(cmd: &str) -> Redirect {
    println!("You typed in {}", cmd);

    let command = utils::get_command_from_query_string(cmd);

    let redirect_url = match command {
        "devbunny" => utils::devbunny::construct_devbunny_url(cmd),
        "gh" => utils::github::construct_github_url(cmd),
        "tw" => utils::twitter::construct_twitter_url(cmd),
        "mail" => utils::gmail::construct_gmail_url(cmd),
        "r" => utils::reddit::construct_reddit_url(cmd),
        "rei" => utils::rei::construct_rei_url(cmd),
        _ => utils::google::construct_google_search_url(cmd),
    };

    Redirect::to(redirect_url)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![search]).launch().await?;
    Ok(())
}
