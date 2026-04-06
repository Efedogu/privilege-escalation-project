# Rust resmi imajını kullanıyoruz
FROM rust:1.70

# Çalışma dizinini ayarla
WORKDIR /usr/src/privilege-escalation-project

# Proje dosyalarını kopyala
COPY . .

# Projeyi derle
RUN cargo build --release

# Programı çalıştır
CMD ["./target/release/privilege-escalation-project"]