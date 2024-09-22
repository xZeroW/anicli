use anicli::api::search_anime;
use dialoguer::Select;
use std::io::{self, Write};
use tokio;

#[tokio::main]
async fn main() {
    // Prompt the user for input
    print!("Search anime: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let keyword = input.trim();

    match search_anime(keyword).await {
        Ok(api_response) => {
            if api_response.animes.is_empty() {
                println!("No anime found.");
                return;
            }

            // Collect anime names for the selection list
            let anime_names: Vec<String> = api_response
                .animes
                .iter()
                .map(|anime| anime.name.clone())
                .collect();

            // Use dialoguer to create a selection prompt
            let selection = Select::new()
                .with_prompt("Select an anime")
                .default(0)
                .items(&anime_names[..])
                .interact()
                .expect("Failed to read selection");

            // Get the selected anime's ID
            let selected_anime = &api_response.animes[selection];
            println!("Selected anime ID: {}", selected_anime.id);
        }
        Err(e) => eprintln!("Error fetching anime: {}", e),
    }
}
