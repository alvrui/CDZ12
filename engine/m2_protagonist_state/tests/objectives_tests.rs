use m2_protagonist_state::{Objective, Objectives, ProtagonistError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_objectives() {
        let objs = Objectives::new();
        assert!(objs.list.is_empty());
    }

    #[test]
    fn test_add_objective() {
        let mut objs = Objectives::new();
        let obj = Objective::new("Complete mission".to_string());
        objs.add_objective("obj_1".to_string(), obj);
        assert_eq!(objs.list.len(), 1);
        assert_eq!(objs.list["obj_1"].description, "Complete mission");
        assert!(!objs.list["obj_1"].completed);
        assert_eq!(objs.list["obj_1"].progress, 0);
    }

    #[test]
    fn test_remove_objective() {
        let mut objs = Objectives::new();
        let obj = Objective::new("Complete mission".to_string());
        objs.add_objective("obj_1".to_string(), obj);
        assert!(objs.remove_objective("obj_1").is_ok());
        assert_eq!(objs.list.len(), 0);
    }

    #[test]
    fn test_update_progress() {
        let mut objs = Objectives::new();
        let obj = Objective::new("Complete mission".to_string());
        objs.add_objective("obj_1".to_string(), obj);
        assert!(objs.update_progress("obj_1", 50).is_ok());
        assert_eq!(objs.list["obj_1"].progress, 50);
    }

    #[test]
    fn test_increase_progress() {
        let mut objs = Objectives::new();
        let obj = Objective::new("Complete mission".to_string());
        objs.add_objective("obj_1".to_string(), obj);
        assert!(objs.increase_progress("obj_1", 30).is_ok());
        assert_eq!(objs.list["obj_1"].progress, 30);
    }

    #[test]
    fn test_mark_completed() {
        let mut objs = Objectives::new();
        let obj = Objective::new("Complete mission".to_string());
        objs.add_objective("obj_1".to_string(), obj);
        assert!(objs.mark_completed("obj_1").is_ok());
        assert!(objs.list["obj_1"].completed);
    }

    #[test]
    fn test_increase_progress_to_completion() {
        let mut objs = Objectives::new();
        let obj = Objective::new("Complete mission".to_string());
        objs.add_objective("obj_1".to_string(), obj);
        assert!(objs.increase_progress("obj_1", 100).is_ok());
        assert!(objs.list["obj_1"].completed);
        assert_eq!(objs.list["obj_1"].progress, 100);
    }

    #[test]
    fn test_get_nonexistent_objective() {
        let objs = Objectives::new();
        assert!(objs.get_objective("obj_1").is_err());
        assert_eq!(
            objs.get_objective("obj_1").unwrap_err(),
            ProtagonistError::ObjectiveNotFound("obj_1".to_string())
        );
    }
}
