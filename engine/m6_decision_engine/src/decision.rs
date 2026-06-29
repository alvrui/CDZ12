use std::collections::HashMap;
use crate::error::DecisionError;
use crate::models::{DecisionNode, DecisionNodeId, DecisionOption, DecisionOptionId, DecisionTree};

impl DecisionTree {
    pub fn get_node(&self, id: &str) -> Result<&DecisionNode, DecisionError> {
        self.nodes
            .get(id)
            .ok_or_else(|| DecisionError::NodeNotFound(id.to_string()))
    }

    pub fn get_option(&self, id: &str) -> Result<&DecisionOption, DecisionError> {
        self.options
            .get(id)
            .ok_or_else(|| DecisionError::OptionNotFound(id.to_string()))
    }

    pub fn get_options_for_node(&self, node_id: &str) -> Result<Vec<&DecisionOption>, DecisionError> {
        let node = self.get_node(node_id)?;
        let mut result = Vec::new();
        for option_id in &node.options {
            result.push(self.get_option(&option_id.id)?);
        }
        Ok(result)
    }

    pub fn is_valid_option(&self, node_id: &str, option_id: &str) -> Result<bool, DecisionError> {
        let node = self.get_node(node_id)?;
        Ok(node.options.iter().any(|opt| opt.id == option_id))
    }
}
