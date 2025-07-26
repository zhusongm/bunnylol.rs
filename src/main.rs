/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */


#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    println!("You typed in {}", cmd);

    let command = utils::get_command_from_query_string(&cmd);

    let redirect_url = match command {
        "devbunny" => utils::devbunny::construct_devbunny_url(&cmd),
        "gh" => utils::github::construct_github_url(&cmd),
        "tw" => utils::twitter::construct_twitter_url(&cmd),
        "mail" => utils::gmail::construct_gmail_url(&cmd),
        "r" => utils::reddit::construct_reddit_url(&cmd),
        _ => utils::google::construct_google_search_url(&cmd),
    };

    Redirect::to(redirect_url)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, search])
        .launch()
        .await?;
    Ok(())
}
