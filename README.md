# 📦 Inventory Management Smart Contract (Soroban)

## 📖 Overview
This project is a **decentralized Inventory Management System** built using the Soroban smart contract platform on the Stellar network. It allows users to manage product stock efficiently, including adding, updating, retrieving, and removing inventory items.

The purpose of this project is to demonstrate how blockchain technology can be used to build a transparent and reliable inventory system.

---

## 🚀 Features
- ➕ Add new products with quantity
- 🔄 Update product stock
- 🔍 Retrieve product information
- ➖ Decrease stock (for transactions/orders)
- ❌ Remove products from inventory

---

## 🏗️ Project Structure

- Smart contracts are stored inside the `contracts/` directory.
- Each contract has its own `Cargo.toml`.
- The project uses a workspace managed by the root `Cargo.toml`.

---

## ⚙️ Technologies Used
- Rust
- Soroban SDK
- Stellar Network (Testnet)

---

## 🛠️ How It Works
The smart contract stores inventory data using a key-value structure:
- **Product Name → Quantity**

All operations interact with blockchain storage, ensuring data integrity and transparency.

---

## 📦 Core Functions

### `add_product`
Adds or updates a product in the inventory.

### `get_product`
Returns the quantity of a product.

### `increase_stock`
Increases the stock of an existing product.

### `decrease_stock`
Reduces stock when a transaction occurs.

### `remove_product`
Deletes a product from the inventory.

---

## 🚀 Build & Deploy
<img width="2817" height="1435" alt="image" src="https://github.com/user-attachments/assets/e5f24a69-d1d2-429f-8ec4-8cea93c5eafb" />

### 1. Build Contract
```bash
cargo build --target wasm32-unknown-unknown --release
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/inventory.wasm \
  --source alice \
  --network testnet
  Contract ID: CBLEFNS35VBPNJ7SJ46NDCKLOBHM6ZG2IWYJHJCVWQ444LKUI4PLJTMN

