#![no_std]

use soroban_sdk::{
contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Product {
pub id: u64,
pub name: String,
pub description: String,
pub category: Symbol,
pub quantity: u32,
pub min_stock: u32,
pub price: u64,
pub supplier_id: u64,
pub last_updated: u64,
pub owner: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct Supplier {
pub id: u64,
pub name: String,
pub contact: String,
pub address: String,
pub verified: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct Transaction {
pub id: u64,
pub product_id: u64,
pub transaction_type: Symbol,
pub amount: i32,
pub remaining_stock: u32,
pub timestamp: u64,
pub operator: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct LowStockAlert {
pub product_id: u64,
pub product_name: String,
pub current_stock: u32,
pub min_stock: u32,
}

const PRODUCTS: Symbol = symbol_short!("PROD");
const SUPPLIERS: Symbol = symbol_short!("SUPP");
const TRANSACTIONS: Symbol = symbol_short!("TRAN");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {
// =========================
// PRODUCT MANAGEMENT
// =========================
pub fn add_product(
    env: Env,
    owner: Address,
    name: String,
    description: String,
    category: Symbol,
    quantity: u32,
    min_stock: u32,
    price: u64,
    supplier_id: u64,
) -> u64 {
    owner.require_auth();

    let mut products: Vec<Product> = env
        .storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env));

    let id = products.len() as u64 + 1;

    let product = Product {
        id,
        name,
        description,
        category,
        quantity,
        min_stock,
        price,
        supplier_id,
        last_updated: env.ledger().timestamp(),
        owner,
    };

    products.push_back(product);
    env.storage().instance().set(&PRODUCTS, &products);

    id
}

pub fn update_stock(
    env: Env,
    operator: Address,
    product_id: u64,
    amount: i32,
) -> String {
    operator.require_auth();

    let mut products: Vec<Product> = env
        .storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env));

    let mut transactions: Vec<Transaction> = env
        .storage()
        .instance()
        .get(&TRANSACTIONS)
        .unwrap_or(Vec::new(&env));

    for i in 0..products.len() {
        let mut product = products.get(i).unwrap();

        if product.id == product_id {
            if amount < 0 && product.quantity < amount.unsigned_abs() {
                return String::from_str(&env, "Stock is insufficient");
            }

            if amount >= 0 {
                product.quantity += amount as u32;
            } else {
                product.quantity -= amount.unsigned_abs();
            }

            product.last_updated = env.ledger().timestamp();

            products.set(i, product.clone());
            env.storage().instance().set(&PRODUCTS, &products);

            let transaction_id = transactions.len() as u64 + 1;

            let transaction_type = if amount >= 0 {
                symbol_short!("IN")
            } else {
                symbol_short!("OUT")
            };

            transactions.push_back(Transaction {
                id: transaction_id,
                product_id,
                transaction_type,
                amount,
                remaining_stock: product.quantity,
                timestamp: env.ledger().timestamp(),
                operator,
            });

            env.storage()
                .instance()
                .set(&TRANSACTIONS, &transactions);

            return String::from_str(&env, "Stock updated successfully");
        }
    }

    String::from_str(&env, "Product not found")
}

pub fn update_price(
    env: Env,
    owner: Address,
    product_id: u64,
    new_price: u64,
) -> String {
    owner.require_auth();

    let mut products: Vec<Product> = env
        .storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env));

    for i in 0..products.len() {
        let mut product = products.get(i).unwrap();

        if product.id == product_id {
            product.price = new_price;
            product.last_updated = env.ledger().timestamp();

            products.set(i, product);
            env.storage().instance().set(&PRODUCTS, &products);

            return String::from_str(&env, "Price updated successfully");
        }
    }

    String::from_str(&env, "Product not found")
}

pub fn delete_product(env: Env, owner: Address, product_id: u64) -> String {
    owner.require_auth();

    let mut products: Vec<Product> = env
        .storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env));

    for i in 0..products.len() {
        if products.get(i).unwrap().id == product_id {
            products.remove(i);
            env.storage().instance().set(&PRODUCTS, &products);
            return String::from_str(&env, "Product deleted successfully");
        }
    }

    String::from_str(&env, "Product not found")
}

// =========================
// SUPPLIER MANAGEMENT
// =========================

pub fn add_supplier(
    env: Env,
    owner: Address,
    name: String,
    contact: String,
    address: String,
) -> u64 {
    owner.require_auth();

    let mut suppliers: Vec<Supplier> = env
        .storage()
        .instance()
        .get(&SUPPLIERS)
        .unwrap_or(Vec::new(&env));

    let id = suppliers.len() as u64 + 1;

    suppliers.push_back(Supplier {
        id,
        name,
        contact,
        address,
        verified: true,
    });

    env.storage().instance().set(&SUPPLIERS, &suppliers);

    id
}

// =========================
// QUERIES
// =========================

pub fn get_products(env: Env) -> Vec<Product> {
    env.storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env))
}

pub fn get_suppliers(env: Env) -> Vec<Supplier> {
    env.storage()
        .instance()
        .get(&SUPPLIERS)
        .unwrap_or(Vec::new(&env))
}

pub fn get_transactions(env: Env) -> Vec<Transaction> {
    env.storage()
        .instance()
        .get(&TRANSACTIONS)
        .unwrap_or(Vec::new(&env))
}

pub fn get_product_by_id(env: Env, product_id: u64) -> Vec<Product> {
    let products: Vec<Product> = env
        .storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env));

    let mut result = Vec::new(&env);

    for i in 0..products.len() {
        let product = products.get(i).unwrap();
        if product.id == product_id {
            result.push_back(product);
        }
    }

    result
}

pub fn get_low_stock_alerts(env: Env) -> Vec<LowStockAlert> {
    let products: Vec<Product> = env
        .storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env));

    let mut alerts = Vec::new(&env);

    for i in 0..products.len() {
        let product = products.get(i).unwrap();

        if product.quantity <= product.min_stock {
            alerts.push_back(LowStockAlert {
                product_id: product.id,
                product_name: product.name,
                current_stock: product.quantity,
                min_stock: product.min_stock,
            });
        }
    }

    alerts
}

pub fn get_total_inventory_value(env: Env) -> u64 {
    let products: Vec<Product> = env
        .storage()
        .instance()
        .get(&PRODUCTS)
        .unwrap_or(Vec::new(&env));

    let mut total: u64 = 0;

    for i in 0..products.len() {
        let product = products.get(i).unwrap();
        total += product.price * product.quantity as u64;
    }

    total
}
}
