use std::process::Command;

// 1. BURASI ANA MERKEZ (ORKESTRA ŞEFİ)
fn main() {
    println!("--- Yetki Yukseltme (Privilege Escalation) Araci ---");
    
    // Fonksiyonları burada sırayla çağırıyoruz:
    sistem_bilgilerini_topla();  // İlk bunu çalıştır
    suid_taramasi_yap();         // Sonra bunu
    yazilabilir_dosya_kontrolu(); // En son bunu
    
    println!("\n[*] Tum taramalar tamamlandi.");
}

// 2. SİSTEM BİLGİSİ TOPLAMA FONKSİYONU
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

// 3. SUID TARAMA FONKSİYONU
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
    }
}

// 4. YAZILABİLİR DOSYA KONTROL FONKSİYONU
fn yazilabilir_dosya_kontrolu() {
    println!("\n[2] Yazilabilir Kritik Dosyalar Kontrol Ediliyor...");
    println!("----------------------------------------------");

    let cikti = Command::new("find")
        .args([".", "-writable", "-type", "f"])
        .output()
        .expect("Komut calistirilamadi");

    if cikti.status.success() {
        let sonuc = String::from_utf8_lossy(&cikti.stdout);
        if sonuc.is_empty() {
            println!("[-] Su anki dizinde tehlikeli yazilabilir dosya bulunamadi.");
        } else {
            println!("[!] DIKKAT: Asagidaki dosyalara herkes yazabilir:\n{}", sonuc);
        }
    }
}