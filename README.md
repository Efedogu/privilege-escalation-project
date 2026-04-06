# İstinye Üniversitesi - Privilege Escalation Project
<p align="center">
  <img src="https://www.istinye.edu.tr/sites/default/files/logo_0.png" width="200">
</p>

## 📌 İçindekiler
* [Proje Hakkında](#proje-hakkında)
* [Kurulum](#kurulum)
* [Kullanım](#kullanım)
* [Özellikler](#özellikler)
* [Lisans](#lisans)

## 📖 Proje Hakkında
Bu proje, Linux sistemlerde yetki yükseltme (Privilege Escalation) açıklarını tespit etmek için geliştirilmiş bir güvenlik aracıdır.

**Danışman:** [Hocanın Adı Buraya]
# Privilege Escalation (Yetki Yükseltme) Analiz Aracı

Bu proje, Linux tabanlı sistemlerdeki potansiyel yetki yükseltme (Privilege Escalation) zafiyetlerini tespit etmek amacıyla **Rust** dili kullanılarak geliştirilmiş bir güvenlik analiz aracıdır.

## 🚀 Projenin Amacı
Siber güvenlik süreçlerinde "Enumeration" (Bilgi Toplama) aşamasını otomatize ederek; hatalı yapılandırılmış izinleri, kritik sistem bilgilerini ve istismar edilebilir SUID dosyalarını tespit etmektir.

## ✨ Özellikler
* **Sistem Bilgisi Toplama:** Mevcut kullanıcı ve kernel sürümü bilgilerini çekerek kernel istismarı (Dirty Cow vb.) olasılıklarını analiz eder.
* **SUID Taraması:** `/usr/bin` altındaki özel yetkili (SUID) dosyaları tespit eder (Görseldeki `pkexec` gibi kritik dosyaların kontrolü).
* **Dünya'ya Yazılabilir Dosyalar:** Sistemde her kullanıcının yazabildiği tehlikeli dosyaları denetler.
* **Renkli Terminal Çıktısı:** `colored` kütüphanesi ile kritik bulguları kırmızı, güvenli olanları yeşil gösterir.
* **Otomatik Raporlama:** Tüm tarama sonuçlarını otomatik olarak `tarama_raporu.txt` dosyasına kaydeder.

## 🛠️ Kullanılan Teknolojiler
* **Dil:** Rust
* **Kütüphaneler:** `colored` (Terminal renklendirme), `std::process` (Sistem komutları), `std::fs` (Dosya işlemleri)

## 📦 Kurulum ve Çalıştırma
Projeyi çalıştırmak için bilgisayarınızda Rust kurulu olmalıdır:

1. Projeyi indirin.
2. Terminalde proje dizinine gidin.
3. Aşağıdaki komutu çalıştırın:
   ```bash
   cargo run