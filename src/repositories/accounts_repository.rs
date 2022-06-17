use crate::{Account, AssignByContractDefinition, AssignByDescription, AssignByNameSearch};

pub trait AccountsRepository {
    fn find_account_by_reference(&self, reference: &str) -> Option<&Account>;
    fn search_account_by_name(&self, name: &str) -> Option<(&AssignByNameSearch, String)>;
    fn search_account_by_contract(
        &self,
        contract_code: &str,
    ) -> Option<&AssignByContractDefinition>;
    fn search_account_by_description(
        &self,
        key: &str,
        description: &str,
    ) -> Option<&AssignByDescription>;
}
