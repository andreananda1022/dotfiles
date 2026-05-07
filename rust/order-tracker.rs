// E-Commerce Order Tracker
enum StatusPesanan {
  Baru,
  Diproses,
  Dikirim(String),
  Selesai,
  Dibatalkan(String)
}

fn cetak_info_pesanan(status: StatusPesanan) {
  match status {
    StatusPesanan::Baru => println!("Pesanan baru saja dibuat. Menunggu konfirmasi penjual."),
    StatusPesanan::Diproses => println!("Pesanan Anda sedang dikemas."),
    StatusPesanan::Dikirim(resi) => println!("Pesanan sedang dalam perjalanan. Nomor Resi: {}", resi),
    StatusPesanan::Selesai => println!("Pesanan telah selesai, Terima kasih telah berbelanja!"),
    StatusPesanan::Dibatalkan(alasan) => println!("Pesanan dibatalkan. Alasan: {}", alasan)
  }
}

fn evaluasi_rating(rating: u8) {
  match rating{
    5 => println!("Luar biasa!"),
    4 => println!("Bagus!"),
    1..=3 => println!("Kami akan berusaha lebih baik lagi."),
    _ => println!("Rating tidak valid.")
  }
}

fn main() {
  let pesanan1 = StatusPesanan::Baru;
  cetak_info_pesanan(pesanan1);                                                  // Pesanan baru saja dibuat. Menunggu konfirmasi penjual.

  let pesanan2 = StatusPesanan::Dikirim(String::from("RESI-2605041423"));
  cetak_info_pesanan(pesanan2);                                                  // Pesanan sedang dalam perjalanan. Nomor Resi: RESI-2605041423

  let pesanan3 = StatusPesanan::Dibatalkan(String::from("Stok barang habis"));
  cetak_info_pesanan(pesanan3);                                                  // Pesanan dibatalkan. Alasan: Stok barang habis

  println!("\n--- Cek Evaluasi Rating ---");                                     // --- Cek Evaluasi Rating ---
  evaluasi_rating(5);                                                            // Luar biasa!
  evaluasi_rating(2);                                                            // Kami akan berusaha lebih baik lagi.
  evaluasi_rating(10);                                                           // Rating tidak valid
}
