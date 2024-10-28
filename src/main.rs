use std::io::{self, Write};

// Represents a single story node.
struct StoryNode {
    text: String,
    options: Vec<String>,
    next_nodes: Vec<usize>,
}

impl StoryNode {
    fn new(text: &str, options: Vec<&str>, next_nodes: Vec<usize>) -> Self {
        Self {
            text: text.to_string(),
            options: options.into_iter().map(String::from).collect(),
            next_nodes,
        }
    }

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

    // Get the index of the next story node based on the player's choice.
    fn get_next_node(&self) -> Option<usize> {
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
                    return Some(self.next_nodes[index - 1]);
                }
                _ => println!("Invalid choice. Please enter a valid option number."),
            }
        }
    }
}

fn main() {
    // Define story nodes
    let nodes = vec![
        StoryNode::new(
            "You find yourself in a dark alley. Which way do you go?",
            vec!["Go left", "Go right"],
            vec![1, 2],
        ),
        StoryNode::new(
            "You encounter security. Pop quiz hotshot, security charges at you, what do you do?",
            vec!["Run away", "Take his gun"],
            vec![0, 3],
        ),
        StoryNode::new(
            "You shot the security guard? You briskly discharge the rest of the bullets and render the gun useless.",
            vec!["Yes", "No"],
            vec![3, 4],
        ),
        StoryNode::new("Congratulations! You are still alive.", vec![], vec![]),
        StoryNode::new("Oh no! It was a trap and you fell into a rabbit hole.", vec![], vec![]),
    ];

    let mut current_node_index = 0; // Start from the first node

    loop {
        let current_node = &nodes[current_node_index];
        current_node.display();

        if let Some(next_node_index) = current_node.get_next_node() {
            current_node_index = next_node_index;
        } else {
            break; // End of the story
        }
    }
}

