// Sistem Inventaris Kedai kopi
enum KategoriItem {
  Minuman,
  Makanan,
  Merchandise
}

struct Item {
  nama: String,
  harga: f64,
  kategori: KategoriItem,
  stok: u32
}

impl Item {
  fn new(nama: String, harga: f64, kategori: KategoriItem, stok: u32) -> Item {
    Item { nama, harga, kategori, stok }
  }

  fn tampilkan_info(&self) {
    let nama_kategori = match self.kategori {
      KategoriItem::Minuman => "Minuman",
      KategoriItem::Makanan => "Makanan",
      KategoriItem::Merchandise => "Merchandise"
    };

    println!("=== Details Item ===");
    println!("Nama: {}\nHarga: {}\nKategori: {}\nSisa Stok: {}\n", self.nama, self.harga, nama_kategori, self.stok);
  }

  fn jual(&mut self) {
    if self.stok > 0 {
      self.stok -= 1;
      println!("Berhasil menjual 1 {}.", self.nama);
    } else {
      println!("Stok {} habis!", self.nama);
    }
  }
}

fn tambah_stok(item: &mut Item, jumlah: u32) {
  item.stok += jumlah;
  println!("Stok {} berhasil ditambah.", item.nama);
} 

fn main() {
  let mut kopi = Item::new(String::from("Kopi"), 15000.00, KategoriItem::Minuman, 7);
  let mut roti = Item::new(String::from("Roti Bakar"), 20000.00, KategoriItem::Makanan, 1);

  kopi.tampilkan_info();
  roti.tampilkan_info();

  println!("\n=== Proses Transaksi ===");
  kopi.jual();
  roti.jual();
  roti.jual(); // akan gagal karena stok habis
  
  println!("\n=== Restock Barang ===");
  tambah_stok(&mut roti, 13);
  roti.jual(); // seharusnya berhasil karna roti sudah restock

  println!("\n=== Info Akhir ===");
  let daftar_item = [&kopi, &roti];
  for item in daftar_item {
    item.tampilkan_info();
  }
}
