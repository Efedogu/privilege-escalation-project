use std::process::Command;
use colored::*;

pub fn sistem_bilgilerini_topla() -> String {
    let mut log = String::from("[0] GENEL SISTEM BILGILERI\n");
    println!("\n{}", "[0] Genel Sistem Bilgileri Toplaniyor...".blue().bold());
    let user = Command::new("whoami").output().ok();
    if let Some(u) = user {
        let name = String::from_utf8_lossy(&u.stdout).trim().to_string();
        println!("[+] Mevcut Kullanici: {}", name.yellow());
        log.push_str(&format!("Kullanici: {}\n", name));
    }
    log.push_str("--------------------------\n");
    log
}

pub fn suid_taramasi_yap() -> String {
    let mut log = String::from("\n[1] SUID DOSYALARI\n");
    println!("\n{}", "[1] SUID Yetkili Dosyalar Kontrol Ediliyor...".blue().bold());
    let cikti = Command::new("find").args(["/usr/bin", "-perm", "-4000"]).output();
    if let Ok(o) = cikti {
        let sonuc = String::from_utf8_lossy(&o.stdout);
        log.push_str(&sonuc);
    }
    log.push_str("--------------------------\n");
    log
}

pub fn yazilabilir_dosya_kontrolu() -> String {
    let mut log = String::from("\n[2] YAZILABILIR DOSYALAR\n");
    println!("\n{}", "[2] Yazilabilir Kritik Dosyalar Kontrol Ediliyor...".blue().bold());
    let cikti = Command::new("find").args([".", "-writable", "-type", "f"]).output();
    if let Ok(o) = cikti {
        let sonuc = String::from_utf8_lossy(&o.stdout);
        log.push_str(&sonuc);
    }
    log.push_str("--------------------------\n");
    log
}

pub fn hassas_dosya_kontrolu() -> String {
    let mut log = String::from("\n[3] HASSAS DOSYA ERISIM KONTROLU\n");
    println!("\n{}", "[3] Hassas Dosya Erisimleri Denetleniyor...".blue().bold());
    let dosyalar = vec!["/etc/shadow", "/etc/sudoers"];
    for dosya in dosyalar {
        let cikti = Command::new("ls").arg("-l").arg(dosya).output();
        if let Ok(o) = cikti {
            let sonuc = String::from_utf8_lossy(&o.stdout);
            log.push_str(&sonuc);
        }
    }
    log.push_str("--------------------------\n");
    log
}

pub fn cron_taramasi_yap() -> String {
    let mut log = String::from("\n[4] CRON GOREVLERI KONTROLU\n");
    println!("\n{}", "[4] Zamanlanmis Gorevler (Cron) Denetleniyor...".blue().bold());
    let cikti = Command::new("ls").args(["-la", "/etc/cron.d"]).output();
    if let Ok(o) = cikti {
        let sonuc = String::from_utf8_lossy(&o.stdout);
        log.push_str(&sonuc);
    }
    log.push_str("--------------------------\n");
    log
}