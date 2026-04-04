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