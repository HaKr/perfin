use crate::Relation;

pub trait RelationsRepository {
    fn find_relation_by_reference(&self, reference: &str) -> Option<&Relation>;
}
