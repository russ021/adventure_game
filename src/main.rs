use std::io::{self, Write};

// Represents a single story node.
struct StoryNode {
    text: String,
    options: Vec<String>,
    next_nodes: Vec<usize>,
}

impl StoryNode {
    fn new(text: String, options: Vec<String>, next_nodes: Vec<usize>) -> Self {
        Self {
            text,
            options,
            next_nodes,
        }
    }

    // Display the current story node's text and available options.
    fn display(&self) {
        println!("{}", self.text);
        for (index, option) in self.options.iter().enumerate() {
            println!("{}. {}", index + 1, option);
        }
    }

    // Get the index of the next story node based on the player's choice.
    fn get_next_node(&self) -> usize {
        loop {
            print!("Enter your choice: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            // Parse the choice as usize, subtracting 1 to adjust for 0-based indexing.
            match choice.trim().parse::<usize>() {
                Ok(index) if index >= 1 && index <= self.options.len() => {
                    return self.next_nodes[index - 1]
                }
                _ => {
                    println!("Invalid choice. Please enter a valid option number.");
                    continue;
                }
            }
        }
    }
}

fn main() {
    // Define story nodes
    let nodes = vec![
        StoryNode::new(
            "You find yourself in a dark forest. Which path do you take?".to_string(),
            vec!["Go left".to_string(), "Go right".to_string()],
            vec![1, 2],
        ),
        StoryNode::new(
            "You encounter a bear. Pop quiz hotshot, the bear charges at you, what do you do?".to_string(),
            vec!["Run away".to_string(), "Fight the bear".to_string()],
            vec![0, 3],
        ),
        StoryNode::new(
            "You stumble upon a red door. Do you open it?".to_string(),
            vec!["Yes".to_string(), "No".to_string()],
            vec![3, 4],
        ),
        StoryNode::new(
            "Congratulations! You are still alive.".to_string(),
            vec![],
            vec![],
        ),
        StoryNode::new(
            "Oh no! It was a trap and you fell into a rabbit hole.".to_string(),
            vec![],
            vec![],
        ),
    ];

    let mut current_node_index = 0; // Start from the first node

    loop {
        let current_node = &nodes[current_node_index];
        current_node.display();

        if current_node.options.is_empty() {
            break; // End of the story
        }

        current_node_index = current_node.get_next_node();
    }
}

