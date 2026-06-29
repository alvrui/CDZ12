use m6_decision_engine::{DecisionEngine, DecisionNode, DecisionNodeId, DecisionNodeType, DecisionOption, DecisionOptionId, DecisionState, DecisionTree};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tree() -> DecisionTree {
        let mut tree = DecisionTree::new(DecisionNodeId::new("start".to_string()));

        let start_node = DecisionNode::new(
            DecisionNodeId::new("start".to_string()),
            "Start".to_string(),
            DecisionNodeType::Start,
            vec![],
        );
        tree.add_node(start_node);

        let mut decision_node = DecisionNode::new(
            DecisionNodeId::new("decision_1".to_string()),
            "Decision".to_string(),
            DecisionNodeType::Decision,
            vec![],
        );

        let option1 = DecisionOption::new(
            DecisionOptionId::new("option_1".to_string()),
            "Option 1".to_string(),
            DecisionNodeId::new("end_1".to_string()),
            HashMap::new(),
        );

        let option2 = DecisionOption::new(
            DecisionOptionId::new("option_2".to_string()),
            "Option 2".to_string(),
            DecisionNodeId::new("end_2".to_string()),
            HashMap::new(),
        );

        tree.add_option(option1);
        tree.add_option(option2);

        decision_node.add_option(DecisionOptionId::new("option_1".to_string()));
        decision_node.add_option(DecisionOptionId::new("option_2".to_string()));
        tree.add_node(decision_node);

        let end_node_1 = DecisionNode::new(
            DecisionNodeId::new("end_1".to_string()),
            "End 1".to_string(),
            DecisionNodeType::End,
            vec![],
        );
        tree.add_node(end_node_1);

        let end_node_2 = DecisionNode::new(
            DecisionNodeId::new("end_2".to_string()),
            "End 2".to_string(),
            DecisionNodeType::End,
            vec![],
        );
        tree.add_node(end_node_2);

        let start_option = DecisionOption::new(
            DecisionOptionId::new("start_option".to_string()),
            "Start".to_string(),
            DecisionNodeId::new("decision_1".to_string()),
            HashMap::new(),
        );
        tree.add_option(start_option);
        if let Some(start_node) = tree.nodes.get_mut("start") {
            start_node.add_option(DecisionOptionId::new("start_option".to_string()));
        }

        tree
    }

    #[test]
    fn test_create_engine() {
        let tree = create_test_tree();
        let engine = DecisionEngine::new(tree);
        assert_eq!(engine.get_state().current_node.id, "start");
    }

    #[test]
    fn test_choose_option() {
        let tree = create_test_tree();
        let mut engine = DecisionEngine::new(tree);

        assert!(engine.choose_option("start_option").is_ok());
        assert_eq!(engine.get_state().current_node.id, "decision_1");
    }

    #[test]
    fn test_choose_invalid_option() {
        let tree = create_test_tree();
        let mut engine = DecisionEngine::new(tree);

        let result = engine.choose_option("invalid_option");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_current_options() {
        let tree = create_test_tree();
        let mut engine = DecisionEngine::new(tree);
        engine.choose_option("start_option").unwrap();

        let options = engine.get_current_options().unwrap();
        assert_eq!(options.len(), 2);
    }

    #[test]
    fn test_can_choose_option() {
        let tree = create_test_tree();
        let mut engine = DecisionEngine::new(tree);
        engine.choose_option("start_option").unwrap();

        assert!(engine.can_choose_option("option_1"));
        assert!(!engine.can_choose_option("invalid"));
    }

    #[test]
    fn test_reset_engine() {
        let tree = create_test_tree();
        let mut engine = DecisionEngine::new(tree);

        engine.choose_option("start_option").unwrap();
        engine.reset();

        assert_eq!(engine.get_state().current_node.id, "start");
    }

    #[test]
    fn test_decision_history() {
        let tree = create_test_tree();
        let mut engine = DecisionEngine::new(tree);

        engine.choose_option("start_option").unwrap();
        engine.choose_option("option_1").unwrap();

        let state = engine.get_state();
        assert_eq!(state.history.len(), 3); // start -> decision_1 -> end_1
        assert_eq!(state.history[0].id, "start");
        assert_eq!(state.history[1].id, "decision_1");
        assert_eq!(state.history[2].id, "end_1");
    }
}
