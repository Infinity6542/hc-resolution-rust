use serde::Deserialize;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
    descendants: u32,
}

fn main() {
    println!("🔶 Top 10 Hacker News Stories\n");

    let client = reqwest::blocking::Client::new();
    let top_ids: Vec<u64> = client
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .expect("Failed to fetch top stores")
        .json()
        .expect("Failed to parse story IDs");

    for (i, id) in top_ids.iter().take(10).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");
        let story: Story = client
            .get(&url)
            .send()
            .expect("Failed to fetch story")
            .json()
            .expect("Failed to parse story");
        let link = story.url.as_deref().unwrap_or("(no URL)");
        println!("{}. {}\n", i + 1, story.title);
        println!("   Author: {}\n", story.by);
        println!("   Score: {}\n", story.score);
        println!("   Comments: {}\n", story.descendants);
        println!("   {}\n", link);
    }
}
