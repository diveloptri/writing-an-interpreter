#[cfg(test)]
mod tests {
    use crate::object::object::{Object};

    #[test]
    fn test_string_hash_key() {
        let hello1 = Object::String("Hello World".to_string());
        let hello2 = Object::String("Hello World".to_string());
        let diff1= Object::String("Wonderful World".to_string());
        let diff2= Object::String("Wonderful World".to_string());

        let hello1_key = hello1.hash_key().expect("String should be hashable");
        let hello2_key= hello2.hash_key().expect("String should be hashable");
        let diff1_key = diff1.hash_key().expect("String should be hashable");
        let diff2_key = diff2.hash_key().expect("String should be hashable");

        assert_eq!(
            hello1_key,
            hello2_key,
            "string with same content have different hash keys"
        );

        assert_eq!(
            diff1_key,
            diff2_key,
            "string with same content have different hash keys"
        );

        assert_ne!(
            hello1_key,
            diff1_key,
            "string with same content have different hash keys"
        );
    }

    #[test]
    fn test_boolean_hash_key() {
        let true1 = Object::Boolean(true);
        let true2 = Object::Boolean(true);
        let false1= Object::Boolean(false);
        let false2= Object::Boolean(false);

        let true1_key = true1.hash_key().expect("Booleans should be hashable");
        let true2_key= true2.hash_key().expect("Booleans should be hashable");
        let false1_key = false1.hash_key().expect("Booleans should be hashable");
        let false2_key = false2.hash_key().expect("Booleans should be hashable");

        assert_eq!(
            true1_key,
            true2_key,
            "trues do not have same hash key"
        );

        assert_eq!(
            false1_key,
            false2_key,
            "falses do not have same hash key"
        );

        assert_ne!(
            true1_key,
            false2_key,
            "true has same hash key as false"
        );
    }

    #[test]
    fn test_integer_hash_key() {
        let one1= Object::Integer(1);
        let one2= Object::Integer(1);
        let two1= Object::Integer(2);
        let two2= Object::Integer(2);

        let one1_key = one1.hash_key().expect("Integer should be hashable");
        let one2_key = one2.hash_key().expect("Integer should be hashable");
        let two1_key = two1.hash_key().expect("Integer should be hashable");
        let two2_key = two2.hash_key().expect("Integer should be hashable");

        assert_eq!(
            one1_key,
            one2_key,
            "integers with same content have different hash keys"
        );

        assert_eq!(
            two1_key,
            two2_key,
            "integers with same content have different hash keys"
        );

        assert_ne!(
            one1_key,
            two2_key,
            "integers with different content have same hash keys"
        );
    }

}