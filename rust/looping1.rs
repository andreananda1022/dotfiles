// Hitung angka 0 pada array (Looping & Mutability)
fn hitung_angka_nol(arr: &[u8]) -> u8 {
  let mut jumlah = 0;
  for &element in arr {
    if element == 0 {
      jumlah += 1;
    }
  }
  jumlah
}

fn main() {
  let arr: [u8; 6] = [12, 0, 45, 0, 0, 34];
  let total = hitung_angka_nol(&arr);
  println!("Jumlah angka nol dalam array: {}", total); // Jumlah angka nol dalam array: 3
}
