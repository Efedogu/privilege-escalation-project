mod scanner;
use std::fs::File;
use std::io::Write;
use colored::*;

fn main() {
    // Banner ve Tasarim
    println!("{}", "===================================================".green());
    println!("{}", "   PRIVILEGE ESCALATION SCANNER - v1.1.0".bright_red().bold());
    println!("{}", "   Gelistirici: Efedogu | Istinye Universitesi".bright_yellow());
    println!("{}", "===================================================".green());

    let mut rapor = String::new();
    rapor.push_str("--- YETKI YUKSELTME TARAMA RAPORU ---\n\n");

    // Fonksiyonlari Sirayla Calistir
    rapor.push_str(&scanner::sistem_bilgilerini_topla());
    rapor.push_str(&scanner::suid_taramasi_yap());
    rapor.push_str(&scanner::yazilabilir_dosya_kontrolu());
    rapor.push_str(&scanner::hassas_dosya_kontrolu());
    rapor.push_str(&scanner::cron_taramasi_yap());

    // Dosyaya Yazdir
    match File::create("tarama_raporu.txt") {
        Ok(mut dosya) => {
            dosya.write_all(rapor.as_bytes()).expect("Yazma hatasi");
            println!("\n{}", "[+] Rapor 'tarama_raporu.txt' olarak kaydedildi.".yellow().bold());
        },
        Err(e) => println!("Hata: {}", e),
    }

    println!("\n{}", "[*] Islem basariyla tamamlandi.".green().bold());
}