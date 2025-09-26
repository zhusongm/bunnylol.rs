#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
mod commands;
mod utils;

use utils::bunnylol_command::BunnylolCommandRegistry;

#[get("/search?<cmd>")]
fn search(cmd: &str) -> Redirect {
    println!("You typed in {}", cmd);

    let command = utils::get_command_from_query_string(cmd);
    let redirect_url = BunnylolCommandRegistry::process_command(command, cmd);

    Redirect::to(redirect_url)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build().mount("/", routes![search]).launch().await?;
    Ok(())
}
