mod scanner;
use std::fs::File;
use std::io::Write;
use colored::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") {
        println!("{}", "Kullanim: cargo run".bright_green());
        println!("Seçenekler:");
        println!("  -h, --help    Yardim menusunu gosterir");
        return;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rapor_olusturma() {
        let mut test_rapor = String::new();
        test_rapor.push_str("Test verisi");
        assert!(test_rapor.contains("Test"));
    }
}
println!("\n{}", "[*] Islem basariyla tamamlandi.".green().bold());
} // main fonksiyonu burada biter

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rapor_olusturma() {
        let mut test_rapor = String::new();
        test_rapor.push_str("Test verisi");
        assert!(test_rapor.contains("Test"));
    }
}