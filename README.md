# هسته شبکه فوق‌سریع مبتنی بر Rust برای SSH-VPN

[![English](https://img.shields.io/badge/English-README-blue?style=flat-square)](README.en.md) | [![فارسی](https://img.shields.io/badge/فارسی-README-green?style=flat-square)](README.md)

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?style=flat-square)](https://www.rust-lang.org) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow?style=flat-square)](LICENSE) [![Tokio](https://img.shields.io/badge/Tokio-Async-blue?style=flat-square)](https://tokio.rs)

یک هسته شبکه بسیار امن و سریع که با زبان **Rust** بازنویسی شده است تا از **SSH Tunneling** غیرهمزمان، **VPN encapsulation** و مدیریت خودکار **Split Tunneling (جدول مسیریابی)** پشتیبانی کند. این هسته به عنوان زیربنای نرم‌افزارهای مدرن VPN طراحی شده و پایداری ۱۰۰٪ و کارایی حداکثری را تضمین می‌کند.

## 🚀 مزایای معماری

- **موتور دوگانه SSH**: پشتیبانی همزمان از `libssh2` (سی) و `russh` (راست بومی) برای حداکثر سازگاری و امنیت.
- **بدون Garbage Collection**: حذف وقفه‌های زمان اجرا برای تضمین پینگ و پهنای باند ثابت.
- **امنیت حافظه تضمین شده**: تایید مدیریت حافظه در زمان کامپایل برای جلوگیری از سرریز بافر و تداخل داده‌ها.
- **پشته غیرهمزمان (Async)**: ساخته شده با Tokio برای مدیریت همزمان هزاران اتصال.
- **Split Tunneling**: پشتیبانی داخلی از پیکربندی روت‌های سیستم‌عامل (Linux, macOS, Windows).
- **سازگاری با SlipNet**: معماری طراحی شده برای سازگاری با پروتکل‌های DNS tunneling مشابه [SlipNet](https://github.com/anonvector/SlipNet).

## 🛠️ ساختار پروژه

```
core-rust-vpn/
├── src/
│   ├── main.rs           # نقطه ورود و مدیریت سرویس‌ها
│   ├── routing.rs        # مدیریت جداول مسیریابی مخصوص هر سیستم‌عامل
│   └── ssh/
│       ├── mod.rs        # ماژول موتورهای SSH
│       ├── libssh2_backend.rs  # بک‌اند LibSSH2 (سی)
│       └── russh_backend.rs   # بک‌اند Russh (راست بومی)
├── tests/
│   └── core_tests.rs     # تست‌های جامع یکپارچه‌سازی و پایداری
├── Cargo.toml           # وابستگی‌ها و تنظیمات پروژه
└── README.md            # مستندات پروژه
```

## 📦 نصب و راه‌اندازی

### پیش‌نیازها

- نصب ابزارهای Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Git برای کلون کردن مخزن

### راه‌اندازی

1. کپی کردن مخزن:
   ```bash
   git clone https://github.com/movtigroup/core-rust-vpn.git
   cd SSH-VPN
   ```

2. کامپایل پروژه در حالت Release:
   ```bash
   cargo build --release
   ```

### اجرا

اجرای فایل باینری خروجی:
```bash
cargo run
```

### تست

اجرای تست‌های ساختاری و پایداری:
```bash
cargo test -- --nocapture
```

## 📝 تنظیمات

تنظیمات هسته از طریق استراکت `SshConfig` قابل انجام است. ویژگی‌های فعلی:

- احراز هویت SSH با یوزرنیم و پسورد.
- فوروارد کردن پورت محلی (سازگار با SOCKS5/HTTP Proxy).
- مدیریت محدوده‌های IP برای Split Tunneling.
- انتخاب بین موتور SSH مبتنی بر C یا Rust بومی.

## ⚖️ لایسنس

منتشر شده تحت لایسنس MIT.
