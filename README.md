Berikut adalah isi file README.md utama (root) yang sudah disesuaikan tanpa menyebutkan asisten laboratorium/praktikum. Kamu bisa langsung menyalin (copy-paste) seluruh kode di bawah ini ke dalam file README.md utama repositorimu:

Markdown
# 🌐 Repositori Mata Kuliah Teknologi IoT

Selamat datang di repositori resmi mata kuliah **Teknologi IoT**. Repositori ini digunakan sebagai pusat pengumpulan tugas, project perkuliahan, serta laporan resmi mahasiswa selama semester berjalan.

---

## 📂 Struktur Direktori

```text
.
├── Kelas_A/       # Folder pengumpulan project & tugas mahasiswa Kelas A
├── Kelas_B/       # Folder pengumpulan project & tugas mahasiswa Kelas B
├── Laporan/       # Folder arsip laporan resmi dan dokumen pendukung
├── .gitignore     # Daftar file/folder yang diabaikan oleh Git
└── README.md      # Panduan utama repositori
📌 Panduan Penggunaan Direktori
Kelas_A/ : Khusus penyimpan tugas individu maupun kelompok mahasiswa Kelas A.

Kelas_B/ : Khusus penyimpan tugas individu maupun kelompok mahasiswa Kelas B.

Laporan/ : Tempat pengumpulan file laporan resmi (format PDF) dari tiap tugas atau project.

⚠️ Aturan Pengumpulan Project (PENTING!)
1. Format Penamaan Folder
Setiap pengunggahan tugas di folder Kelas_A/ maupun Kelas_B/ wajib mengikuti format berikut:

Tugas Individu: [NIM]_[Nama-Lengkap]

Contoh: 2042241001_Ahmad_Fauzi

Tugas Kelompok: Kelompok_[Nomor]_[Nama-Project]

Contoh: Kelompok_01_Smart_Monitoring_Suhu

2. Bebas File Build / Cache (Penting untuk Project Rust/ESP32)
Untuk mencegah ukuran repositori membengkak akibat file kompilasi (seperti folder target/ atau .embuild/), WAJIB menjalankan perintah pembersihan sebelum commit/upload:

PowerShell
cargo clean
Catatan: Jangan mengunggah folder target/ atau file ZIP yang berisi folder build. Ukuran project yang bersih hanya berukuran beberapa Kilobyte/Megabyte.

🛠️ Teknologi & Environment
Mikrokontroler: ESP32 / ESP32-S3

Bahasa & Framework: Rust (esp-idf-sys, embuild) / C++

Tools: Cargo, espflash, Git
