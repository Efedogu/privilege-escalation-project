mod scanner;
use std::fs::File;
use std::io::Write;
use colored::*;

fn main() {
    // [1. HAMLE] Profesyonel Açılış Banner'ı
    println!("{}", "===================================================".green());
    println!("{}", "   PRIVILEGE ESCALATION SCANNER - v1.1.0".bright_red().bold());
    println!("{}", "   Gelistirici: Efedogu | Istinye Universitesi".bright_yellow());
    println!("{}", "===================================================".green());

    let mut rapor = String::new();
    rapor.push_str("--- YETKI YUKSELTME TARAMA RAPORU ---\n\n");

    println!("{}", "\n[*] Tarama Baslatiliyor...".cyan());
    
    // Tarama Modülleri Çağrılıyor
    rapor.push_str(&scanner::sistem_bilgilerini_topla());
    rapor.push_str(&scanner::suid_taramasi_yap());
    rapor.push_str(&scanner::yazilabilir_dosya_kontrolu());
    rapor.push_str(&scanner::hassas_dosya_kontrolu());

    // Raporu Dosyaya Yazma İşlemi
    match File::create("tarama_raporu.txt") {
        Ok(mut dosya) => {
            if let Err(e) = dosya.write_all(rapor.as_bytes()) {
                println!("{}: {}", "[-] Rapor yazilirken hata olustu".red(), e);
            } else {
                println!("\n{}", "[+] Tum sonuclar 'tarama_raporu.txt' dosyasina kaydedildi.".yellow().bold());
            }
        },
        Err(e) => println!("{}: {}", "[-] Rapor dosyasi olusturulamadi".red(), e),
    }

    println!("\n{}", "[*] Tarama ve raporlama islemi basariyla tamamlandi.".green().bold());
}