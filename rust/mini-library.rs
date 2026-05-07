// Core Logic Sistem Perpustakaan Mini
struct Buku {
  judul: String,
  penulis: String,
  sedang_dipinjam: bool
}

struct Perpustakaan {
  koleksi: Vec<Buku>
}

impl Buku {
  fn new(judul: String, penulis: String) -> Buku {
    Buku {
      judul,
      penulis,
      sedang_dipinjam: false
    }
  }
}

impl Perpustakaan {
  fn new() -> Perpustakaan {
    Perpustakaan {
      koleksi: Vec::new()
    }
  }

  fn tambah_buku(&mut self, buku: Buku) {
    self.koleksi.push(buku);
  }

  fn cek_status_buku(&self, judul_dicari: &str) {
    let mut ditemukan = false;
    for buku in &self.koleksi {
      if judul_dicari == buku.judul {
        ditemukan = true;
        println!("Status buku dipinjam: {}", buku.sedang_dipinjam);
        break;
      }
    }

    if !ditemukan {
      println!("Judul yang anda cari tidak ditemukan.");
    }
  }

  fn pinjam_buku(&mut self, judul_dicari: &str) {
    let mut ditemukan = false;
    for buku in &mut self.koleksi {
      if judul_dicari == buku.judul {
        ditemukan = true;
        if buku.sedang_dipinjam == false {
          buku.sedang_dipinjam = true;
          println!("Anda berhasil meminjam buku.");
        } else {
          println!("Buku yang anda cari sedang dipinjam.");
        }
        break;
      }
    }

    if !ditemukan {
      println!("Judul yang anda cari tidak ditemukan.");
    }
  }
}

fn main() {
  let judul1 = String::from("Pemrograman Rust");
  let penulis1 = String::from("Budi Santoso");

  let buku1 = Buku::new(judul1, penulis1);

  let judul2 = String::from("Struktur Data");
  let penulis2 = String::from("Siti Aminah");

  let buku2 = Buku::new(judul2, penulis2);

  let mut perpus_kota = Perpustakaan::new();
  perpus_kota.tambah_buku(buku1);
  perpus_kota.tambah_buku(buku2);

  println!("=== Pengecekan Awal ===");              // === Pengecekan Awal ===
  perpus_kota.cek_status_buku("Pemrograman Rust");  // Status buku dipinjam: false
  perpus_kota.cek_status_buku("Buku Something");    // Judul yang anda cari tidak ditemukan.

  println!("\n=== Proses Peminjaman ===");          // === Proses Peminjaman ===
  perpus_kota.pinjam_buku("Pemrograman Rust");      // Anda berhasil meminjam buku.

  println!("\n=== Pengecekan Akhir ===");           // === Pengecekan Akhir ===
  perpus_kota.cek_status_buku("Pemrograman Rust");  // Status buku dipinjam: true
}
