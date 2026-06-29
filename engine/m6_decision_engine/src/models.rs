//use crate::DecisionNode as OtherDecisionNode;;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionNodeId {
    pub id: String,
}

impl DecisionNodeId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionOptionId {
    pub id: String,
}

impl DecisionOptionId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionNode {
    pub id: DecisionNodeId,
    pub description: String,
    pub node_type: DecisionNodeType,
    pub options: Vec<DecisionOptionId>,
}

impl DecisionNode {
    pub fn new(
        id: DecisionNodeId,
        description: String,
        node_type: DecisionNodeType,
        options: Vec<DecisionOptionId>,
    ) -> Self {
        Self {
            id,
            description,
            node_type,
            options,
        }
    }

    pub fn add_option(&mut self, option_id: DecisionOptionId) {
        self.options.push(option_id);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionNodeType {
    Start,
    Decision,
    End,
    Event,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionOption {
    pub id: DecisionOptionId,
    pub description: String,
    pub target_node: DecisionNodeId,
    pub conditions: HashMap<String, String>,
}

impl DecisionOption {
    pub fn new(
        id: DecisionOptionId,
        description: String,
        target_node: DecisionNodeId,
        conditions: HashMap<String, String>,
    ) -> Self {
        Self {
            id,
            description,
            target_node,
            conditions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionTree {
    pub nodes: HashMap<String, DecisionNode>,
    pub options: HashMap<String, DecisionOption>,
    pub root_node: DecisionNodeId,
}

impl DecisionTree {
    pub fn new(root_node: DecisionNodeId) -> Self {
        Self {
            nodes: HashMap::new(),
            options: HashMap::new(),
            root_node,
        }
    }

    pub fn add_node(&mut self, node: DecisionNode) {
        self.nodes.insert(node.id.id.clone(), node);
    }

    pub fn add_option(&mut self, option: DecisionOption) {
        self.options.insert(option.id.id.clone(), option);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionState {
    pub current_node: DecisionNodeId,
    pub history: Vec<DecisionNodeId>,
}

impl DecisionState {
    pub fn new(start_node: DecisionNodeId) -> Self {
        Self {
            current_node: start_node.clone(),
            history: vec![start_node],
        }
    }

    pub fn add_to_history(&mut self, node_id: DecisionNodeId) {
        self.history.push(node_id);
    }
}
