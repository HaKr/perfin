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
6. Styling and styking themes


[1]: <https://www.ing.nl/media/ING_CSV_Mijn_ING_Augustus2020_tcm162-201483.pdf> "Format description of ING's transactions download file (CSV)"
[2]: <https://github.com/ebcrowder/rust_ledger> "rust_ledger crate"
[3]: <https://github.com/sunng87/handlebars-rust> "handlebars-rust crate"
[4]: <https://handlebarsjs.com/> "Minimal templating on steroids"
[5]: <https://d3js.org/> "Data-Driven Documents"
[6]: <https://yaml.org/> "YAML Ain't Markup Language"