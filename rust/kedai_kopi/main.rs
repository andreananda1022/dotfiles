mod menu;

use std::io::{self, Write};
use menu::menu::{ TypesOfCoffee, CoffeeSize };
use menu::transaction::{ Order, print_receipt };

fn get_user_input(var: &mut String, text: &str) {
  print!("{}", text);
  io::stdout().flush().expect("Gagal melakukan flush!");
  io::stdin().read_line(var).expect("Gagal membaca input");
}

fn main() {
  println!("Selamat Datang di Kedai Kopi Rust!");

  loop {
    let mut customer_name = String::new();
    get_user_input(&mut customer_name, "\nMasukkan nama (atau ketik 'Cancel' untuk keluar): ");

    if customer_name.trim() == "Cancel" {
      println!("Terima kasih!");
      break;
    }

    let mut type_of_coffee = String::new();
    get_user_input(&mut type_of_coffee, "Pilih Kopi (1: Espresso, 2: Latte, 3: Cappuccino): ");

    let coffee = match type_of_coffee.trim() {
      "1" => TypesOfCoffee::Espresso,
      "2" => TypesOfCoffee::Latte,
      "3" => TypesOfCoffee::Cappuccino,
      _ => {
        println!("Pilihan kopi tidak tersedia!");
        continue;
      }
    };

    let mut coffee_size = String::new();
    get_user_input(&mut coffee_size, "Pilih Ukuran (1: Medium, 2: Large): ");

    let size = match coffee_size.trim() {
      "1" => CoffeeSize::Medium,
      "2" => CoffeeSize::Large,
      _ => {
        println!("Pilihan ukuran kopi tidak tersedia!");
        continue;
      }
    };

    let pesanan = Order {
      customer_name: customer_name.trim().to_string(),
      coffee: coffee,
      size: size
    };

    print_receipt(&pesanan);
  }
}
