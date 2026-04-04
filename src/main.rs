use std::process::Command;
use std::fs::File;
use std::io::Write;
use colored::*;

fn main() {
    let mut rapor_icerigi = String::new();
    rapor_icerigi.push_str("--- YETKI YUKSELTME TARAMA RAPORU ---\n\n");

    println!("{}", "--- Yetki Yukseltme (Privilege Escalation) Araci ---".bright_cyan().bold());
    
    // 1. Sistem Bilgileri
    let sistem_bilgisi = sistem_bilgilerini_topla();
    rapor_icerigi.push_str(&sistem_bilgisi);

    // 2. SUID Taraması
    let suid_bilgisi = suid_taramasi_yap();
    rapor_icerigi.push_str(&suid_bilgisi);

    // 3. Yazılabilir Dosya Kontrolü
    let yazilabilir_bilgisi = yazilabilir_dosya_kontrolu();
    rapor_icerigi.push_str(&yazilabilir_bilgisi);

    // Raporu Dosyaya Kaydet
    match File::create("tarama_raporu.txt") {
        Ok(mut dosya) => {
            dosya.write_all(rapor_icerigi.as_bytes()).expect("Dosyaya yazilamadi");
            println!("\n{}", "[+] Tarama sonuclari 'tarama_raporu.txt' dosyasina kaydedildi.".yellow().bold());
        },
        Err(_) => println!("\n[!] Hata: Rapor dosyasi olusturulamadi."),
    }

    println!("\n{}", "[*] Tum islemler tamamlandi.".green().bold());
}

fn sistem_bilgilerini_topla() -> String {
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

fn suid_taramasi_yap() -> String {
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

fn yazilabilir_dosya_kontrolu() -> String {
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