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

## 🛠 Teknik Analiz ve Derinlik

### 1. Mimari Seçimi (Neden Rust ve Modüler Yapı?)
* **Güvenlik (Memory Safety):** Proje için Rust seçildi çünkü bellek güvenliği (memory safety) hatalarını derleme aşamasında önler. Siber güvenlik araçlarında "buffer overflow" gibi açıkların olmaması kritiktir.
* **Modülerlik:** Kodun `main.rs` ve `scanner.rs` olarak ayrılması, projenin ölçeklenebilirliğini sağlar. Yeni tarama modülleri (örn: network scanner) eklendiğinde ana mantık bozulmadan sisteme entegre edilebilir.

### 2. Güvenlik Sonuçları ve Etki Analizi
* **SUID Taraması:** Yanlış yapılandırılmış bir SUID dosyası, saldırganın saniyeler içinde "root" yetkisine ulaşmasına neden olur.
* **Hassas Dosya Erişimi:** `/etc/shadow` gibi dosyaların okunabilir olması, parolanın offline olarak kırılabileceği anlamına gelir. Aracımız bu zafiyetleri önceden tespit ederek sistem sıkılaştırma (hardening) sürecine katkı sağlar.

### 3. Gelecek Geliştirmeler ve İyileştirme Planı
* **Dinamik Exploit Veritabanı:** Araç, gelecekte bilinen kernel açıklarını (CVE) bir API üzerinden çekerek otomatik exploit önerisi sunacak şekilde genişletilebilir.
* **JSON Çıktı Desteği:** SIEM sistemlerine (Splunk, ELK) veri basabilmesi için tarama sonuçlarının JSON formatında dışa aktarılması planlanmaktadır.