use std::process::Command;
use colored::*;

/// Sistem bilgilerini (kullanıcı adı, kernel sürümü vb.) toplar.
/// Bu bilgiler kernel istismarı tespiti için kritiktir.
pub fn sistem_bilgilerini_topla() -> String {
    let mut log = String::from("[0] GENEL SISTEM BILGILERI\n");
    println!("\n{}", "[0] Genel Sistem Bilgileri Toplaniyor...".blue());

    let user = Command::new("whoami").output().ok();
    if let Some(u) = user {
        let name = String::from_utf8_lossy(&u.stdout).trim().to_string();
        println!("[+] Mevcut Kullanici: {}", name.yellow());
        log.push_str(&format!("Kullanici: {}\n", name));
    }
    log.push_str("--------------------------\n");
    log
}

/// SUID (Set User ID) yetkisine sahip dosyaları tarar.
/// Yanlış yapılandırılmış SUID dosyaları doğrudan root yetkisi verebilir.
pub fn suid_taramasi_yap() -> String {
    let mut log = String::from("\n[1] SUID DOSYALARI\n");
    println!("\n{}", "[1] SUID Yetkili Dosyalar Kontrol Ediliyor...".blue());

    let cikti = Command::new("find").args(["/usr/bin", "-perm", "-4000"]).output();

    if let Ok(o) = cikti {
        let sonuc = String::from_utf8_lossy(&o.stdout);
        log.push_str(&sonuc);
        if sonuc.is_empty() {
            println!("{}", "[-] SUID dosyasi bulunamadi.".green());
        } else {
            println!("{}", "[!] SUID dosyalari tespit edildi ve rapora eklendi.".red());
        }
    }
    log.push_str("--------------------------\n");
    log
}

/// Mevcut dizindeki herkes tarafından yazılabilir dosyaları bulur.
/// Konfigürasyon dosyalarına yetkisiz erişimi denetler.
pub fn yazilabilir_dosya_kontrolu() -> String {
    let mut log = String::from("\n[2] YAZILABILIR DOSYALAR\n");
    println!("\n{}", "[2] Yazilabilir Kritik Dosyalar Kontrol Ediliyor...".blue());

    let cikti = Command::new("find").args([".", "-writable", "-type", "f"]).output();

    if let Ok(o) = cikti {
        let sonuc = String::from_utf8_lossy(&o.stdout);
        log.push_str(&sonuc);
        if sonuc.is_empty() {
            println!("{}", "[-] Yazilabilir dosya bulunamadi.".green());
        } else {
            println!("{}", "[!] Yazilabilir dosyalar tespit edildi ve rapora eklendi.".red());
        }
    }
    log.push_str("--------------------------\n");
    log
}

/// Hassas sistem dosyalarının (shadow, sudoers vb.) erişim durumunu kontrol eder.
pub fn hassas_dosya_kontrolu() -> String {
    let mut log = String::from("\n[3] HASSAS DOSYA ERISIM KONTROLU\n");
    println!("\n{}", "[3] Hassas Dosya Erisimleri Denetleniyor...".blue());

    let dosyalar = vec!["/etc/shadow", "/etc/sudoers", "/root/.bash_history"];
    
    for dosya in dosyalar {
        let cikti = Command::new("ls").arg("-l").arg(dosya).output();
        if let Ok(o) = cikti {
            let sonuc = String::from_utf8_lossy(&o.stdout);
            log.push_str(&sonuc);
            println!("{}: {}", "Kontrol edildi".yellow(), dosya);
        }
    }
    log.push_str("--------------------------\n");
    log
}