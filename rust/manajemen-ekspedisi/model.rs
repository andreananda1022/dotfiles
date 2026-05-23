use std::fmt;

#[derive(PartialEq, Clone)]
pub enum StatusPengiriman {
    Pending,
    Transit(String),
    Terkirim,
    Gagal(String)
}

impl fmt::Display for StatusPengiriman {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusPengiriman::Pending => {
                write!(f, "Pending")
            }
            StatusPengiriman::Transit(lokasi) => {
                write!(f, "Transit: {}", lokasi)
            }
            StatusPengiriman::Terkirim => {
                write!(f, "Terkirim")
            }
            StatusPengiriman::Gagal(alasan) => {
                write!(f, "Gagal: {}", alasan)
            }
        }
    }
}

pub enum KategoriBarang {
    Dokumen,
    Elektronik,
    Pakaian
}

pub struct Paket {
    pub no_resi: String,
    pub pengirim: String,
    pub tujuan: String,
    pub berat_kg: f64,
    pub kategori: KategoriBarang,
    pub status: StatusPengiriman
}

impl Paket {
    pub fn new(no_resi: String, pengirim: String, tujuan: String, berat_kg: f64, kategori: KategoriBarang) -> Self {
        Paket {
            no_resi,
            pengirim,
            tujuan,
            berat_kg,
            kategori,
            status: StatusPengiriman::Pending
        }
    }
}
