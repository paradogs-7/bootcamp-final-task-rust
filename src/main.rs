
use std::collections::HashMap;
use std::io::{self, Write};

struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

struct Inventory {
    items: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory { items: HashMap::new() }
    }
    fn add(&mut self, name: String, description: String, price: f64, quantity: u32) -> Result<(), String> {
        if self.items.contains_key(&name) {
            return Err("Product already exists".into());
        }
        let product = Product { name: name.clone(), description, price, quantity };
        self.items.insert(name, product);
        Ok(())
    }
    fn edit(&mut self, name: &str, description: Option<String>, price: Option<f64>, quantity: Option<u32>) -> Result<(), String> {
        let product = self.items.get_mut(name).ok_or("Product not found")?;
        if let Some(desc) = description { product.description = desc; }
        if let Some(p) = price { product.price = p; }
        if let Some(q) = quantity { product.quantity = q; }
        Ok(())
    }
    fn delete(&mut self, name: &str) -> Result<(), String> {
        self.items.remove(name).map(|_| ()).ok_or("Product not found".into())
    }
    fn list(&self) {
        println!("Inventory:");
        println!("{:<20} {:<10} {:<10} {:<10}", "Name", "Price", "Qty", "Desc");
        for product in self.items.values() {
            println!("{:<20} {:<10.2} {:<10} {:<10}", product.name, product.price, product.quantity, product.description);
        }
    }
}

struct SalesTransaction {
    product_name: String,
    quantity: u32,
    sale_price: f64,
    total: f64,
}

struct PurchaseTransaction {
    product_name: String,
    quantity: u32,
    purchase_price: f64,
    total: f64,
}

struct AuthManager {
    users: HashMap<String, String>,
}

impl AuthManager {
    fn new() -> Self {
        let mut users = HashMap::new();
        users.insert("admin".into(), "password".into());
        AuthManager { users }
    }
    fn authenticate(&self) -> bool {
        print!("Username: "); io::stdout().flush().ok();
        let mut username = String::new(); io::stdin().read_line(&mut username).ok();
        let username = username.trim();
        print!("Password: "); io::stdout().flush().ok();
        let mut password = String::new(); io::stdin().read_line(&mut password).ok();
        let password = password.trim();
        match self.users.get(username) {
            Some(pw) if pw == password => true,
            _ => false,
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input.trim().to_string()
}

fn main() {
    let auth = AuthManager::new();
    println!("Welcome to Rusty Store Inventory System");
    if !auth.authenticate() {
        println!("Authentication failed");
        return;
    }
    let mut inventory = Inventory::new();
    let mut sales: Vec<SalesTransaction> = Vec::new();
    let mut purchases: Vec<PurchaseTransaction> = Vec::new();
    loop {
        println!("\nMenu:\n1 Manage Inventory\n2 Record Sale\n3 Record Purchase\n4 Reports\n5 Exit");
        print!("Choice: "); io::stdout().flush().ok();
        let choice = read_input();
        match choice.as_str() {
            "1" => {
                println!("a Add\nb Edit\nc Delete\nd List");
                print!("Option: "); io::stdout().flush().ok();
                match read_input().as_str() {
                    "a" => {
                        print!("Name: "); io::stdout().flush().ok();
                        let name = read_input();
                        print!("Desc: "); io::stdout().flush().ok();
                        let desc = read_input();
                        print!("Price: "); io::stdout().flush().ok();
                        let price: f64 = read_input().parse().unwrap_or(0.0);
                        print!("Qty: "); io::stdout().flush().ok();
                        let qty: u32 = read_input().parse().unwrap_or(0);
                        match inventory.add(name, desc, price, qty) {
                            Ok(_) => println!("Product added"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    "b" => {
                        print!("Name: "); io::stdout().flush().ok();
                        let name = read_input();
                        print!("New Desc (or blank): "); io::stdout().flush().ok();
                        let desc = read_input();
                        let desc_opt = if desc.is_empty() { None } else { Some(desc) };
                        print!("New Price (or blank): "); io::stdout().flush().ok();
                        let price_in = read_input();
                        let price_opt = if price_in.is_empty() { None } else { price_in.parse().ok() };
                        print!("New Qty (or blank): "); io::stdout().flush().ok();
                        let qty_in = read_input();
                        let qty_opt = if qty_in.is_empty() { None } else { qty_in.parse().ok() };
                        match inventory.edit(&name, desc_opt, price_opt, qty_opt) {
                            Ok(_) => println!("Product edited"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    "c" => {
                        print!("Name: "); io::stdout().flush().ok();
                        let name = read_input();
                        match inventory.delete(&name) {
                            Ok(_) => println!("Product deleted"),
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    "d" => inventory.list(),
                    _ => println!("Invalid option"),
                }
            }
            "2" => {
                print!("Product: "); io::stdout().flush().ok();
                let name = read_input();
                print!("Qty: "); io::stdout().flush().ok();
                let qty: u32 = read_input().parse().unwrap_or(0);
                print!("Sale Price: "); io::stdout().flush().ok();
                let price: f64 = read_input().parse().unwrap_or(0.0);
                match inventory.items.get_mut(&name) {
                    Some(product) if product.quantity >= qty => {
                        product.quantity -= qty;
                        let total = price * qty as f64;
                        sales.push(SalesTransaction { product_name: name.clone(), quantity: qty, sale_price: price, total });
                        println!("Sale recorded" );
                    }
                    Some(_) => println!("Error: out of stock"),
                    None => println!("Error: product not found"),
                }
            }
            "3" => {
                print!("Product: "); io::stdout().flush().ok();
                let name = read_input();
                print!("Qty: "); io::stdout().flush().ok();
                let qty: u32 = read_input().parse().unwrap_or(0);
                print!("Purchase Price: "); io::stdout().flush().ok();
                let price: f64 = read_input().parse().unwrap_or(0.0);
                inventory.items.entry(name.clone()).and_modify(|p| p.quantity += qty).or_insert(Product { name: name.clone(), description: "".into(), price: price, quantity: qty });
                let total = price * qty as f64;
                purchases.push(PurchaseTransaction { product_name: name, quantity: qty, purchase_price: price, total });
                println!("Purchase recorded");
            }
            "4" => {
                println!("\nInventory Report"); inventory.list();
                println!("\nSales Report:");
                println!("{:<20} {:<10} {:<10} {:<10}", "Product", "Qty", "Price", "Total");
                for tx in &sales {
                    println!("{:<20} {:<10} {:<10.2} {:<10.2}", tx.product_name, tx.quantity, tx.sale_price, tx.total);
                }
                println!("\nPurchase Report:");
                println!("{:<20} {:<10} {:<10} {:<10}", "Product", "Qty", "Price", "Total");
                for tx in &purchases {
                    println!("{:<20} {:<10} {:<10.2} {:<10.2}", tx.product_name, tx.quantity, tx.purchase_price, tx.total);
                }
            }
            "5" => {
                println!("Goodbye");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_inventory_add_edit_delete() {
        let mut inv = Inventory::new();
        assert!(inv.add("item1".into(), "desc".into(), 10.0, 5).is_ok());
        assert!(inv.items.contains_key("item1"));
        assert!(inv.edit("item1", Some("d2".into()), Some(12.0), Some(8)).is_ok());
        let p = inv.items.get("item1").unwrap();
        assert_eq!(p.description, "d2");
        assert_eq!(p.price, 12.0);
        assert_eq!(p.quantity, 8);
        assert!(inv.delete("item1").is_ok());
        assert!(!inv.items.contains_key("item1"));
    }
    #[test]
    fn test_sales_and_purchase() {
        let mut inv = Inventory::new();
        inv.add("item2".into(), "d".into(), 5.0, 10).unwrap();
        let mut sales = Vec::new();
        if let Some(p) = inv.items.get_mut("item2") {
            p.quantity -= 4;
            let total = 6.0 * 4 as f64;
            sales.push(SalesTransaction { product_name: "item2".into(), quantity: 4, sale_price: 6.0, total });
        }
        assert_eq!(inv.items.get("item2").unwrap().quantity, 6);
        let mut purchases = Vec::new();
        inv.items.entry("item2".into()).and_modify(|p| p.quantity += 5);
        let total = 4.0 * 5 as f64;
        purchases.push(PurchaseTransaction { product_name: "item2".into(), quantity: 5, purchase_price: 4.0, total });
        assert_eq!(inv.items.get("item2").unwrap().quantity, 11);
    }
}
