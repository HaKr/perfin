pub trait CostCentersRepository {
    fn find_cost_center_by_iban(&self, iban: &str) -> Option<String>;
}
