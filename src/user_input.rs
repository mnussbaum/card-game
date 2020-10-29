use crate::game_rules::Action;
use text_io::read;

pub fn select_action(available_actions: Vec<&Action>) -> Option<&Action> {
    if available_actions.len() > 1 {
        println!(
            "Select action by number:\n{}",
            available_actions
                .iter()
                .enumerate()
                .map(|(i, a)| format!("{}. {}", i, a))
                .collect::<Vec<String>>()
                .join("\n"),
        );
        let selected_action_index: usize = read!();
        if let Some(chosen_action) = available_actions.get(selected_action_index) {
            return Some(chosen_action);
        } else {
            println!("Invalid action selection");
            return select_action(available_actions);
        }
    } else if available_actions.len() == 1 {
        println!("{}", available_actions[0]);
        return Some(available_actions[0]);
    } else {
        println!("No actions available to you right now");
        return None;
    }
}
