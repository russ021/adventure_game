use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

// Represents a single story node.
#[derive(Deserialize, Debug)]
struct StoryNode {
    text: String,
    options: Vec<String>,
    next_nodes: Vec<String>,
}

impl StoryNode {
    // Display the current story node's text and available options.
    fn display(&self) {
        println!("\n{}", self.text);
        self.display_options();
    }

    // Display options, if any
    fn display_options(&self) {
        if self.options.is_empty() {
            println!("(End of story)");
            return;
        }
        for (index, option) in self.options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }
    }

    // Get the ID of the next story node based on the player's choice.
    fn get_next_node(&self) -> Option<&String> {
        if self.options.is_empty() {
            return None;
        }

        loop {
            print!("Enter your choice: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            match choice.trim().parse::<usize>() {
                Ok(index) if (1..=self.options.len()).contains(&index) => {
                    return Some(&self.next_nodes[index - 1]);
                }
                _ => println!("Invalid choice. Please enter a valid option number."),
            }
        }
    }
}

fn main() {
    // Load story data from JSON file
    let data = fs::read_to_string("story.json").expect("Unable to read story.json");
    let nodes: HashMap<String, StoryNode> =
        serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut current_node_id = String::from("start");

    loop {
        if let Some(current_node) = nodes.get(&current_node_id) {
            current_node.display();

            if let Some(next_node_id) = current_node.get_next_node() {
                current_node_id = next_node_id.clone();
            } else {
                break; // End of the story
            }
        } else {
            println!("Error: Node '{}' not found!", current_node_id);
            break;
        }
    }
}
