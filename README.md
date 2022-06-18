# perfin - Personal Finance

Keep track of your expenses, income and savings.


This tool offers a way to keep track of where your money goes, with as little as possible
manual labour. It goes through imported bank transactions and assigns them to one of
the configured accounts. One does not need to be an accountant, but we use solid 
bookkeeping techniques in order to have verified results.

Of course there is thec [Rust Ledger CLI][2], but that assumes you to have the amounts colledcted
per account to begin with.

This project starts with a download file of bank transactions from your own bank and an
(self configured) accounting schema. The accounting schema also includes rules on how to 
determine to which account a transaction can be assigned.

## Functionality
1. ✅ Web server and handlebars templates based framework
2. ✅ Accounting schema with assignment rules
3. ✅ Import of bank transactions from the (Dutch) [ING bank][1]

## To-do
1. Manually assign bank transactions to an account
2. Overview per account and/or per cost center
3. Graphical views of the account overviews
4. Maintenance of accounting schema in UI
5. User roles and multiple administrations
6. Styling and styling themes

## Configuration
The configuration is stored in [Yaml files][6] in the _data/organisations_ folder. Each organisations has it's own 
subfolder there, and for each book year there is again a subfolder.

### Example
 + data
     + organisations
         + some ID
            + 2020
            + 2021
            + 2022
                + ledger.yaml   

### Configuration file syntax
The configuration is in ledger.yaml, which has the following structure:

- **name**:  The name of the organisation
- **currency_iso*: ISO code of the currency used for this ledger

- **bank_formats**: hash of supported bank upload formats
  - **KEY**: one of the known uploads (currently, only **ing** is supported)
    - **name**: name for this format
    - **description**: Explanation of the format to the end user
 
 - **cost_centers**: list of cost centers that can be used. At least one **must** be configured 
 
 - **accounts**: hash of accounts
   - **KEY**: short code
      - **description**: More descriptive label 

 - **bank_accounts**: hash of IBANs for which transactions can be uploaded
   - **KEY=IBAN**: IBAN of your own bank account
     - **cost_center**: One of the cost center codes (from **cost_centers**). Must be unique within the bank accounts
     - **description**: descriptive label

 - **relations**: hash of known IBANs to show up as a known relation name (may be empty)
   - **KEY=IBAN**: IBAN of the known bank account
     - **name**: relation name to use in bank transactions 
 
 - **assign_by_contract**: hash of contract IDs that can be assiged immediately
    - **KEY**: search term as specified by your bank (ING uses _Machtiging ID_)
      - **account**: account code to assign, must exist under **accounts**
      - _note_: optional hint about the contract, not further used in UI
      - _description_: optional prefix for the description field of the bank transaction

- **assign_by_description**: hash of cost centers plus relation names and search texts that can be assiged immediately
    - **KEY**: concatenation of a **cost center** the literal " & " and a relation name
                (the relation name from the **relations** hash, or as specified by the bank) 
      - **search**: [Regular expression][8] to find in the transaction description
                    (If your not familiar with regular expressions, just enter a plain search string)
      - _note_: optional hint about the contract, not further used in UI
  
- **assign_by_name**: hash of account codes with search string that can be assiged immediately
  - **KEY=account code"**: A code from the **accounts** hash
    - list of search [Regular expressions][8] to find in the transaction description
      (If your not familiar with regular expressions, just enter a plain search string) 
      
#### Example
```yaml
name: Personal costs and income
currency_iso: EUR

bank_formats:
  ing:
    name: ing
    description: ING comma separated values

cost_centers:
  - Hers
  - His

accounts:
  mortgage:
    description: Mortgage
  utitilties:
    description: Gas, Water en Power
  assurances:
    description: Assurances
  telecom:
    description: Telecommunication
  household:
    description: Groceries and such
  recreational:
    description: Restaurants, amusement parks and day trips
  inventory:
    description: Inventory
  maintenance:
    description: House & garden maintenance
  misc:
    description: Non specified
  holidays:
    description: Holidays
  transport:
    description: Transportation
  healthcare:
    description: Healthcare
  income:
    description: Salaries and Tax refunds
  savings:
    description: Savings


bank_accounts:
  NL00XXXX0000000001:
    cost_center: Hers
    description: Her bank
  NL00XXXX0000000002:
    cost_center: His
    description: His bank

relations:
  NL00XXXX0000000011:
    name: Tax office
  NL00XXXX0000000012:
    name: Her employer
  NL00XXXX0000000013:
    name: His employer

assign_by_contract:
  XXXXXXXXXXXXXX:
    account: transport
  123456789:
    account: utilities
    note: Power company contract no: 678534658734657
    description: Power

assign_by_description:
  Hers & Her employer:
    - search: salary
      account: income
  His & Impressive company Inc:
    - search: monthly
      account: income
      note: WA verzekering wordt betaald via gezinspakket
  Hers & Tax office:
    - search: License XX-123-YYYY-4
      account: transport
      note: monthly driver's tax
    - search: Refund
      account: income
      note: annula tax refund

assign_by_name:
  household:
    - \w+ groceries
    - \w+ bakery  
  holidays:
    - Hotel \w+
  recreational:
    - Cafe {naam}
    - Some non informative 
    - Restaurant {naam}
  misc:
    - present shop
    - flowers

```
  

## Technical description
The basis is the [Axum web application framework][7], where the pages are dynamically created by [Handlebars templates][4].

[1]: <https://www.ing.nl/media/ING_CSV_Mijn_ING_Augustus2020_tcm162-201483.pdf> "Format description of ING's transactions download file (CSV)"
[2]: <https://github.com/ebcrowder/rust_ledger> "rust_ledger crate"
[3]: <https://github.com/sunng87/handlebars-rust> "handlebars-rust crate"
[4]: <https://handlebarsjs.com/> "Minimal templating on steroids"
[5]: <https://d3js.org/> "Data-Driven Documents"
[6]: <https://yaml.org/> "YAML Ain't Markup Language"
[7]: <https://github.com/tokio-rs/axum> "web application framework in Rust, based on Tokio, Tower and Hyper"
[8]: <https://regex101.com/> "Regular expressions"
