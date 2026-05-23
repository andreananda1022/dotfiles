use std::fmt;

pub enum EkspedisiError {
    ResiTidakDitemukan(String),
    FormatResiSalah,
    PaketSudahTerkirim
}

impl fmt::Display for EkspedisiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EkspedisiError::ResiTidakDitemukan(resi) => {
                write!(f, "Nomor resi '{}' tidak ditemukan di sistem.", resi)
            }
            EkspedisiError::FormatResiSalah => {
                write!(f, "Format nomor resi tidak valid.")
            }
            EkspedisiError::PaketSudahTerkirim => {
                write!(f, "Paket sudah terkirim.")
            }
        }
    }
}
