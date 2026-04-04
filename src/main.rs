use std::process::Command;
use colored::*; // Renklendirme kütüphanesini kullan

fn main() {
    println!("{}", "--- Yetki Yukseltme (Privilege Escalation) Araci ---".bright_cyan().bold());
    
    sistem_bilgilerini_topla();
    suid_taramasi_yap();
    yazilabilir_dosya_kontrolu();

    println!("\n{}", "[*] Tum taramalar basariyla tamamlandi.".green().bold());
}

fn sistem_bilgilerini_topla() {
    println!("\n{}", "[0] Genel Sistem Bilgileri Toplaniyor...".blue());
    println!("----------------------------------------------");

    let user = Command::new("whoami").output().ok();
    if let Some(u) = user {
        let name = String::from_utf8_lossy(&u.stdout).trim().to_string();
        println!("[+] Mevcut Kullanici: {}", name.yellow());
    }
}

fn suid_taramasi_yap() {
    println!("\n{}", "[1] SUID Yetkili Dosyalar Kontrol Ediliyor...".blue());
    println!("----------------------------------------------");

    let cikti = Command::new("find")
        .args(["/usr/bin", "-perm", "-4000"])
        .output();

    match cikti {
        Ok(o) => {
            let sonuc = String::from_utf8_lossy(&o.stdout);
            if sonuc.is_empty() {
                println!("{}", "[-] Kritik SUID dosyasi bulunamadi.".green());
            } else {
                println!("{}", "[!] Bulunan Kritik Dosyalar:".red().bold());
                println!("{}", sonuc);
            }
        },
        Err(_) => println!("{}", "[!] Hata: Bu tarama sadece Linux sistemlerde calisir.".on_red()),
    }
}

fn yazilabilir_dosya_kontrolu() {
    println!("\n{}", "[2] Yazilabilir Kritik Dosyalar Kontrol Ediliyor...".blue());
    println!("----------------------------------------------");

    let cikti = Command::new("find")
        .args([".", "-writable", "-type", "f"])
        .output();

    if let Ok(o) = cikti {
        let sonuc = String::from_utf8_lossy(&o.stdout);
        if sonuc.is_empty() {
            println!("{}", "[-] Tehlikeli yazilabilir dosya bulunamadi.".green());
        } else {
            println!("{}", "[!] DIKKAT: Yazilabilir dosyalar var!".red().bold());
            println!("{}", sonuc);
        }
    }
}