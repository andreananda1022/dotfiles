mod model;
mod error;
use model::{StatusPengiriman, KategoriBarang, Paket};
use error::EkspedisiError;
use std::collections::HashMap;

struct SistemEkspedisi {
    daftar_paket: HashMap<String, Paket>,
    log_sistem: Vec<String>
}

impl SistemEkspedisi {
    fn new() -> Self {
        SistemEkspedisi {
            daftar_paket: HashMap::new(),
            log_sistem: Vec::new()
        }
    }

    fn tambah_paket(&mut self, paket: Paket) {
        self.log_sistem.push(format!("Paket {} berhasil ditambahkan.", paket.no_resi));
        self.daftar_paket.insert(paket.no_resi.clone(), paket);
    }

    fn update_status(&mut self, no_resi: String, status_baru: StatusPengiriman) -> Result<(), EkspedisiError> {
        if !no_resi.contains('-') {
            return Err(EkspedisiError::FormatResiSalah);
        }

        if let Some(paket) = self.daftar_paket.get_mut(&no_resi) {
            if paket.status == StatusPengiriman::Terkirim {
                return Err(EkspedisiError::PaketSudahTerkirim);
            }

            paket.status = status_baru.clone();
            self.log_sistem.push(format!("Paket {}: Status diubah menjadi {}", no_resi, status_baru));
            Ok(())
        } else {
            Err(EkspedisiError::ResiTidakDitemukan(no_resi))
        }
    }

    fn tampilkan_log(&self) {
        for log in &self.log_sistem {
            println!("{}", log);
        }
    }
}

fn main() {
    println!("\n### Sistem Manajemen Paket ###\n");
    let mut sistem = SistemEkspedisi::new();
    let paket1 = Paket::new(
        String::from("CKG-SUB-001"),
        String::from("Andi Puryadi"),
        String::from("Jakarta"),
        3.5,
        KategoriBarang::Elektronik
    );
    let paket2 = Paket::new(
        String::from("PDG-UKU-003"),
        String::from("Budi Sudiarjo"),
        String::from("Padang"),
        4.2,
        KategoriBarang::Pakaian
    );
    let paket3 = Paket::new(
        String::from("JKT-CWY-023"),
        String::from("Retno Supriyanto"),
        String::from("Jakarta"),
        1.2,
        KategoriBarang::Dokumen
    );

    sistem.tambah_paket(paket1);
    sistem.tambah_paket(paket2);
    sistem.tambah_paket(paket3);

    // Kasus A: Format resi salah
    match sistem.update_status("SALAHFORMAT".to_string(), StatusPengiriman::Terkirim) {
        Ok(_) => println!("Sukses update"),
        Err(e) => println!("Error: {}", e)
    }

    // Kasus B: Resi tidak ditemukan
    match sistem.update_status("BDO-MES-002".to_string(), StatusPengiriman::Terkirim) {
        Ok(_) => println!("Sukses update"),
        Err(e) => println!("Error: {}", e)
    }

    // Kasus C: Tets update status
    match sistem.update_status("JKT-CWY-023".to_string(), StatusPengiriman::Transit("Padang".to_string())) {
        Ok(_) => println!("Sukses update"),
        Err(e) => println!("Error: {}", e)
    }

    sistem.tampilkan_log();
}
