function renumber(class_name) {
    const elements = document.getElementsByClassName(class_name);
    if (typeof elements == "object" && Reflect.ownKeys(elements).indexOf("0") >= 0) {
        const collection = elements[0];
        const labels = Array.prototype.slice.call(collection.getElementsByClassName("index"));
        labels.forEach((label, index) => {
            const cat_index = label.innerText;
            label.innerText = `${index + 1}.${cat_index}`;
        });
    }
}

function calculate(class_name, amounts) {
    const elements = document.getElementsByClassName(class_name);
    if (typeof elements == "object" && Reflect.ownKeys(elements).indexOf("0") >= 0) {
        const collection = elements[0];
        const headers = Array.prototype.slice.call(collection.getElementsByClassName("category-header"));
        headers.forEach(header_element => {
            const category_amounts = new Amounts(header_element, amounts);
            const amount_elements = Array.prototype.slice.call(header_element.getElementsByClassName("amount"));
            amount_elements.forEach(amount_element => {
                const amount_str = amount_element.innerText;
                const amount = Number.parseFloat(amount_str);
                category_amounts.add(amount);
            });
            category_amounts.update();
        });

        amounts.update();
    }

}

class Amounts {
    constructor(element, parent) {
        this.parent = parent
        if (element) {
            this.db = element.getElementsByClassName("db")[0];
            this.cr = element.getElementsByClassName("cr")[0];
            this.tot = element.getElementsByClassName("tot")[0];
        }
        this.debit = 0.0;
        this.credit = 0.0;
    }

    add(amount) {
        if (amount < 0.0) {
            this.credit += Math.abs(amount);
        } else {
            this.debit += amount;
        }

        if (this.parent) {
            this.parent.add(amount);
        }
    }

    get total() {
        return this.debit - this.credit;
    }

    update() {
        this.db.innerText = `${this.debit.toFixed(2)}`;
        this.cr.innerText = `${this.credit.toFixed(2)}`;
        this.tot.innerText = `${this.total.toFixed(2)}`;
    }
}



renumber("imported");
renumber("assigned");

const total_general = document.getElementById("total_general");
const total_imported = document.getElementById("total_imported");
const total_assigned = document.getElementById("total_assigned");

const general = new Amounts(total_general);
const imported = new Amounts(total_imported, general);
const assigned = new Amounts(total_assigned, general);


calculate("imported", imported);
calculate("assigned", assigned);

general.update();