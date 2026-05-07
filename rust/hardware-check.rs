// Pengecekan Perangkat Keras Bootloader (Fungsi & Control Flow)
fn cek_memori(ram_mb: u32) -> String {
  if ram_mb < 512 {
    String::from("Kernel panic: Out of Memory.")
  } else if ram_mb < 1024 {
    String::from("Peringatan: Memori terbatas, memuat kernel minimal...")
  } else {
    String::from("Memulai proses booting...")
  }
}

fn main() {
  let status_sistem_lama = cek_memori(256);
  let status_sistem_baru = cek_memori(2048);
  println!("Sistem lama: {}", status_sistem_lama); // Sistem lama: Kernel panic: Out of Memory
  println!("Sistem baru: {}", status_sistem_baru); // Sistem baru: Memulai proses booting...
}
