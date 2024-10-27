// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn check_product_name(product_name: &String) -> bool {
        if product_name.is_empty() || product_name.len() > 300 {
            return false;
        }
        true
    }
    fn check_quantity(quantity: u32) -> bool {
        if quantity <= 0 {
            return false;
        }
        true
    }
    fn check_unit_price(unit_price: u32) -> bool {
        if unit_price <= 0 {
            return false;
        }
        true
    }
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        if !Self::check_product_name(&product_name) {
            panic!("Product name is empty or too long");
        }
        if !Self::check_quantity(quantity) {
            panic!("Quantity must be strictly greater than zero");
        }
        if !Self::check_unit_price(unit_price) {
            panic!("Unit price must be strictly greater than zero");
        }
        Self {
            product_name,
            quantity,
            unit_price,
        }
    }
    pub fn set_product_name(&mut self, product_name: String) {
        if !Self::check_product_name(&product_name) {
            panic!("Product name is empty or too long");
        }
        self.product_name = product_name;
    }
    pub fn set_quantity(&mut self, quantity: u32) {
        if !Self::check_quantity(quantity) {
            panic!("Quantity must be strictly greater than zero");
        }
        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self, unit_price: u32) {
        if !Self::check_unit_price(unit_price) {
            panic!("Unit price must be strictly greater than zero");
        }
        self.unit_price = unit_price;
    }
    pub fn product_name(&self) -> &str {
        &self.product_name
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
