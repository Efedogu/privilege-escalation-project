# Kullanıcı Kılavuzu

## Sistemi Çalıştırma
1. `cargo build` ile projeyi derleyin.
2. `cargo run` ile taramayı başlatın.

## Rapor Analizi
Tarama bittikten sonra kök dizinde oluşan `tarama_raporu.txt` dosyasını inceleyin. Bu rapor şunları içerir:
- SUID zafiyetleri
- Yazılabilir kritik dosyalar
- Cron işleri zafiyet analizi