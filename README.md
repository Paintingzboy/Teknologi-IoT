# 🌐 Repositori Hasil Tugas Teknologi IoT

Repositori ini berisi dokumentasi, *source code*, dan laporan hasil pengerjaan tugas serta project untuk mata kuliah **Teknologi IoT**.

---

## 📂 Struktur Repositori

```text
.
├── Kelas_A/       # Hasil project dan tugas Kelas A
├── Kelas_B/       # Hasil project dan tugas Kelas B
├── Laporan/       # Berkas laporan resmi dan dokumentasi
├── .gitignore     # Konfigurasi pengabaian file build/cache
└── README.md      # Ringkasan repositori
📌 Deskripsi Direktori
Kelas_A/ : Berisi source code dan file proyek IoT yang dikerjakan oleh Kelas A.

Kelas_B/ : Berisi source code dan file proyek IoT yang dikerjakan oleh Kelas B.

Laporan/ : Berisi arsip laporan hasil pengujian, analisis, dan dokumentasi proyek.

🛠️ Teknologi & Tools
Mikrokontroler: ESP32 / ESP32-S3

Bahasa Pemrograman: Rust (esp-idf-sys, embuild) / C++

Pembangunan & Toolchain: Cargo, espflash, Git

💡 Catatan Pemeliharaan Repositori
Untuk menjaga ukuran repositori tetap ringan, seluruh project Rust/ESP32 di dalam repositori ini telah dibersihkan dari folder build (target/ dan .embuild/) menggunakan perintah:

PowerShell
cargo clean
