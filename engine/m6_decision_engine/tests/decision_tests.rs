use m6_decision_engine::{DecisionNode, DecisionNodeId, DecisionNodeType, DecisionOption, DecisionOptionId, DecisionTree};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tree() -> DecisionTree {
        let mut tree = DecisionTree::new(DecisionNodeId::new("start".to_string()));

        // Nodo de inicio
        let start_node = DecisionNode::new(
            DecisionNodeId::new("start".to_string()),
            "Punto de inicio".to_string(),
            DecisionNodeType::Start,
            vec![],
        );
        tree.add_node(start_node);

        // Nodo de decisión
        let mut decision_node = DecisionNode::new(
            DecisionNodeId::new("decision_1".to_string()),
            "¿Qué hacer?".to_string(),
            DecisionNodeType::Decision,
            vec![],
        );

        // Opciones
        let option1 = DecisionOption::new(
            DecisionOptionId::new("option_1".to_string()),
            "Opción A".to_string(),
            DecisionNodeId::new("end_a".to_string()),
            HashMap::new(),
        );

        let option2 = DecisionOption::new(
            DecisionOptionId::new("option_2".to_string()),
            "Opción B".to_string(),
            DecisionNodeId::new("end_b".to_string()),
            HashMap::new(),
        );

        tree.add_option(option1);
        tree.add_option(option2);

        decision_node.add_option(DecisionOptionId::new("option_1".to_string()));
        decision_node.add_option(DecisionOptionId::new("option_2".to_string()));
        tree.add_node(decision_node);

        // Nodos finales
        let end_node_a = DecisionNode::new(
            DecisionNodeId::new("end_a".to_string()),
            "Final A".to_string(),
            DecisionNodeType::End,
            vec![],
        );
        tree.add_node(end_node_a);

        let end_node_b = DecisionNode::new(
            DecisionNodeId::new("end_b".to_string()),
            "Final B".to_string(),
            DecisionNodeType::End,
            vec![],
        );
        tree.add_node(end_node_b);

        // Conectar inicio a decisión
        let start_option = DecisionOption::new(
            DecisionOptionId::new("start_option".to_string()),
            "Comenzar".to_string(),
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
    fn test_create_tree() {
        let tree = create_test_tree();
        assert_eq!(tree.nodes.len(), 4);
        assert_eq!(tree.options.len(), 3);
    }

    #[test]
    fn test_get_node() {
        let tree = create_test_tree();
        let node = tree.get_node("start").unwrap();
        assert_eq!(node.description, "Punto de inicio");
    }

    #[test]
    fn test_get_nonexistent_node() {
        let tree = create_test_tree();
        let result = tree.get_node("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_options_for_node() {
        let tree = create_test_tree();
        let options = tree.get_options_for_node("decision_1").unwrap();
        assert_eq!(options.len(), 2);
    }

    #[test]
    fn test_is_valid_option() {
        let tree = create_test_tree();
        assert!(tree.is_valid_option("decision_1", "option_1").unwrap());
        assert!(!tree.is_valid_option("decision_1", "nonexistent").unwrap());
    }
}
