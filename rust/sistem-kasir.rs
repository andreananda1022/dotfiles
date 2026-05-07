// Core Logic Sistem Kasir
fn hitung_total(items: &Vec<(String, f64)>) -> f64 {
  let mut total = 0.0;

  for (_, harga) in items {
    total += harga;
  }
  total
}

fn main() {
  let mut keranjang: Vec<(String, f64)> = Vec::new();
  keranjang.push((String::from("Apel"), 35000.0));
  keranjang.push((String::from("Susu"), 25000.0));

  let total_belanja: f64 = hitung_total(&keranjang);
  let diskon: f64 = if total_belanja > 40000.0 {
    0.10 * total_belanja
  } else {
    0.0
  };

  let total_akhir = total_belanja - diskon;

  println!("Total belanja: Rp{}", total_belanja);           // Total belanja: Rp60000
  println!("Diskon yang didapat: Rp{}", diskon);            // Diskon yang didapat: Rp6000
  println!("Total yang harus dibayar: Rp{}", total_akhir);  // Total yang harus dibayar: Rp54000
}
