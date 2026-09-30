# 🌐 Repositori Mata Kuliah Teknologi IoT

Repositori resmi untuk pengelolaan tugas, pengumpulan project akhir, dan arsip laporan perkuliahan mata kuliah **Teknologi IoT**.

---

## 📂 Struktur Direktori

```text
.
├── Kelas_A/       # Folder pengumpulan project & tugas Kelas A
├── Kelas_B/       # Folder pengumpulan project & tugas Kelas B
├── Laporan/       # Folder arsip laporan resmi dan dokumen pendukung
├── .gitignore     # Pengaturan pengabaian file build/cache
└── README.md      # Panduan utama repositori

```

---

## 📌 Panduan Direktori

* **`Kelas_A/`** : Direktori penyimpanan tugas individu dan kelompok mahasiswa Kelas A.
* **`Kelas_B/`** : Direktori penyimpanan tugas individu dan kelompok mahasiswa Kelas B.
* **`Laporan/`** : Tempat pengumpulan berkas laporan dalam format PDF.

---

## ⚠️ Ketentuan Pengumpulan Project

### 1. Format Penamaan Folder

Setiap pengunggahan tugas di folder `Kelas_A/` maupun `Kelas_B/` menggunakan format penamaan berikut:

* **Tugas Individu:** `NIM_Nama-Lengkap`
*Contoh:* `2042241001_Ahmad_Fauzi`
* **Tugas Kelompok:** `Kelompok_Nomor_Nama-Project`
*Contoh:* `Kelompok_01_Smart_Monitoring_Suhu`

### 2. Bebas File Build & Cache

Untuk menjaga ukuran repositori tetap ringan, **WAJIB** membersihkan folder hasil kompilasi (seperti `target/` atau `.embuild/`) sebelum melakukan pengunggahan atau *commit*.

Untuk project berbasis Rust, jalankan perintah berikut di terminal sebelum membuat file kompresi atau upload:

```powershell
cargo clean

```

---

## 🛠️ Lingkungan Pengembangan

* **Mikrokontroler:** ESP32 / ESP32-S3
* **Bahasa & Framework:** Rust (`esp-idf-sys`, `embuild`) / C++
* **Tools:** Cargo, `espflash`, Git

```

```
