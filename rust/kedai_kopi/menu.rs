pub mod menu {
  pub enum TypesOfCoffee {
    Espresso,
    Latte,
    Cappuccino
  }

  pub enum CoffeeSize {
    Medium,
    Large
  }

  pub fn price(types_of_coffee: &TypesOfCoffee) -> u32 {
    match types_of_coffee {
      TypesOfCoffee::Espresso => 15000,
      TypesOfCoffee::Latte => 22000,
      TypesOfCoffee::Cappuccino => 26000
    }
  }

  pub fn additional_price(coffee_size: &CoffeeSize) -> u32 {
    match coffee_size {
      CoffeeSize::Medium => 0,
      CoffeeSize::Large => 5000
    }
  }
}

pub mod transaction {
  use super::menu::{ TypesOfCoffee, CoffeeSize, price, additional_price };

  pub struct Order {
    pub customer_name: String,
    pub coffee: TypesOfCoffee,
    pub size: CoffeeSize
  }

  impl Order {
    pub fn total_order(&self) -> u32 {
      price(&self.coffee) + additional_price(&self.size)
    }
  }

  pub fn print_receipt(order: &Order) {
    let coffee_type = match order.coffee {
      TypesOfCoffee::Espresso => "Espresso",
      TypesOfCoffee::Latte => "Latte",
      TypesOfCoffee::Cappuccino => "Cappuccino"
    };

    let coffee_size = match order.size {
      CoffeeSize::Medium => "Medium",
      CoffeeSize::Large => "Large"
    };

    println!("\n===== Receipt =====");
    println!("\nName: {}", order.customer_name);
    println!("Coffee: {}", coffee_type);
    println!("Size: {}", coffee_size);
    println!("Total harga: Rp{}", order.total_order());
    println!("\n===================");
  }
}
