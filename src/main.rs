use std::process::Command;

fn main() {
    println!("--- Yetki Yukseltme (Privilege Escalation) Araci ---");
    println!("[*] Sistem analiz ediliyor...");
    
    // SUID fonksiyonunu cagir
    suid_taramasi_yap();
}

fn suid_taramasi_yap() {
    println!("\n[1] SUID Yetkili Dosyalar Kontrol Ediliyor...");
    println!("----------------------------------------------");

    // Linux 'find' komutu ile tehlikeli olabilecek SUID dosyalarini ararız
    let cikti = Command::new("find")
        .args(["/usr/bin", "-perm", "-4000"])
        .output()
        .expect("Komut calistirilamadi");

    if cikti.status.success() {
        let sonuc = String::from_utf8_lossy(&cikti.stdout);
        if sonuc.is_empty() {
            println!("[-] Kritik SUID dosyasi bulunamadi.");
        } else {
            println!("[+] Bulunan Kritik Dosyalar:\n{}", sonuc);
        }
    } else {
        println!("[!] Tarama sirasinda bir hata olustu. (Linux tabanli sistem gereklidir)");
    }
}

use std::process::Command;

fn main() {
    // Program başladığında ekrana başlık yazar
    println!("--- Yetki Yukseltme (Privilege Escalation) Araci ---");
    
    // 1. ADIM: Sistem bilgilerini topla (Burada çağırıyoruz)
    sistem_bilgilerini_topla();

    // 2. ADIM: SUID dosyalarını tara (Burada çağırıyoruz)
    suid_taramasi_yap();

    println!("\n[*] Tarama tamamlandi.");
}

// --- FONKSİYON TANIMLARI ---

fn sistem_bilgilerini_topla() {
    println!("\n[0] Genel Sistem Bilgileri Toplaniyor...");
    println!("----------------------------------------------");

    let user = Command::new("whoami").output().ok();
    let kernel = Command::new("uname").arg("-a").output().ok();

    if let Some(u) = user {
        println!("[+] Mevcut Kullanici: {}", String::from_utf8_lossy(&u.stdout).trim());
    }
    
    if let Some(k) = kernel {
        println!("[+] Kernel Surumu: {}", String::from_utf8_lossy(&k.stdout).trim());
    }
}

fn suid_taramasi_yap() {
    println!("\n[1] SUID Yetkili Dosyalar Kontrol Ediliyor...");
    println!("----------------------------------------------");

    let cikti = Command::new("find")
        .args(["/usr/bin", "-perm", "-4000"])
        .output()
        .expect("Komut calistirilamadi");

    if cikti.status.success() {
        let sonuc = String::from_utf8_lossy(&cikti.stdout);
        if sonuc.is_empty() {
            println!("[-] Kritik SUID dosyasi bulunamadi.");
        } else {
            println!("[+] Bulunan Kritik Dosyalar:\n{}", sonuc);
        }
    } else {
        println!("[!] Tarama sirasinda bir hata olustu. (Linux tabanli sistem gereklidir)");
    }
}