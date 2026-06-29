//use crate::DecisionOption;
use crate::error::DecisionError;
use crate::models::{DecisionNode, DecisionNodeId, DecisionOption, DecisionOptionId, DecisionState, DecisionTree};

pub struct DecisionEngine {
    tree: DecisionTree,
    state: DecisionState,
}

impl DecisionEngine {
    pub fn new(tree: DecisionTree) -> Self {
        Self {
            state: DecisionState::new(tree.root_node.clone()),
            tree,
        }
    }

    pub fn get_current_node(&self) -> Result<&DecisionNode, DecisionError> {
        self.tree.get_node(&self.state.current_node.id)
    }

    pub fn get_current_options(&self) -> Result<Vec<&DecisionOption>, DecisionError> {
        self.tree.get_options_for_node(&self.state.current_node.id)
    }

    pub fn choose_option(&mut self, option_id: &str) -> Result<(), DecisionError> {
        if !self.tree.is_valid_option(&self.state.current_node.id, option_id)? {
            return Err(DecisionError::InvalidOption);
        }

        let option = self.tree.get_option(option_id)?;
        let target_node = self.tree.get_node(&option.target_node.id)?;

        self.state.current_node = target_node.id.clone();
        self.state.add_to_history(self.state.current_node.clone());

        Ok(())
    }

    pub fn reset(&mut self) {
        self.state = DecisionState::new(self.tree.root_node.clone());
    }

    pub fn get_state(&self) -> &DecisionState {
        &self.state
    }

    pub fn can_choose_option(&self, option_id: &str) -> bool {
        self.tree.is_valid_option(&self.state.current_node.id, option_id).unwrap_or(false)
    }
}
