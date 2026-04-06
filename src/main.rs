mod scanner; // scanner.rs dosyasını projeye dahil eder
use std::fs::File;
use std::io::Write;
use colored::*;

fn main() {
    let mut rapor_icerigi = String::new();
    rapor_icerigi.push_str("--- YETKI YUKSELTME TARAMA RAPORU ---\n\n");

    println!("{}", "--- Yetki Yukseltme (Privilege Escalation) Araci ---".bright_cyan().bold());
    
    // Fonksiyonları artık scanner:: ismiyle çağırıyoruz
    let sistem_bilgisi = scanner::sistem_bilgilerini_topla();
    rapor_icerigi.push_str(&sistem_bilgisi);

    let suid_bilgisi = scanner::suid_taramasi_yap();
    rapor_icerigi.push_str(&suid_bilgisi);

    let yazilabilir_bilgisi = scanner::yazilabilir_dosya_kontrolu();
    rapor_icerigi.push_str(&yazilabilir_bilgisi);

    let hassas_bilgi = scanner::hassas_dosya_kontrolu(); 
    rapor_icerigi.push_str(&hassas_bilgi);

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