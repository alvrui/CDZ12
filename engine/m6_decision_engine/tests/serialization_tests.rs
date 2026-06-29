use m6_decision_engine::{DecisionNode, DecisionNodeId, DecisionNodeType, DecisionOption, DecisionOptionId, DecisionTree};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tree() -> DecisionTree {
        let mut tree = DecisionTree::new(DecisionNodeId::new("root".to_string()));

        let root_node = DecisionNode::new(
            DecisionNodeId::new("root".to_string()),
            "Root".to_string(),
            DecisionNodeType::Start,
            vec![],
        );
        tree.add_node(root_node);

        let option = DecisionOption::new(
            DecisionOptionId::new("opt_1".to_string()),
            "Option 1".to_string(),
            DecisionNodeId::new("node_1".to_string()),
            HashMap::new(),
        );
        tree.add_option(option);

        let node_1 = DecisionNode::new(
            DecisionNodeId::new("node_1".to_string()),
            "Node 1".to_string(),
            DecisionNodeType::Decision,
            vec![],
        );
        tree.add_node(node_1);

        tree
    }

    #[test]
    fn test_serialize_to_json() {
        let tree = create_test_tree();
        let json = serde_json::to_string(&tree).unwrap();
        assert!(json.contains("root"));
    }

    #[test]
    fn test_deserialize_from_json() {
        let tree = create_test_tree();
        let json = serde_json::to_string(&tree).unwrap();
        let deserialized: DecisionTree = serde_json::from_str(&json).unwrap();
        assert_eq!(tree.nodes.len(), deserialized.nodes.len());
    }

    #[test]
    fn test_serialize_to_yaml() {
        let tree = create_test_tree();
        let yaml = serde_yaml::to_string(&tree).unwrap();
        assert!(yaml.contains("root"));
    }

    #[test]
    fn test_serialize_to_messagepack() {
        let tree = create_test_tree();
        let msgpack = rmp_serde::to_vec(&tree).unwrap();
        assert!(!msgpack.is_empty());
    }
}
