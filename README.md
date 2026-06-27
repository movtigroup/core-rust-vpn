# هسته شبکه فوق‌سریع مبتنی بر Rust برای SSH-VPN

یک هسته شبکه بسیار امن و سریع که با زبان **Rust** بازنویسی شده است تا از **SSH Tunneling** غیرهمزمان، **SOCKS5**، **HTTP Proxy** و مدیریت خودکار **Split Tunneling** پشتیبانی کند.

## 🚀 ویژگی‌های کلیدی
- **پشتیبانی از SOCKS5 و HTTP Proxy**: امکان استفاده به عنوان یک پراکسی سرور داینامیک.
- **موتور دوگانه SSH**: پشتیبانی از `libssh2` و `russh` (راست بومی).
- **بدون Garbage Collection**: تضمین پایداری و سرعت در سطح سیستم.
- **Async IO**: استفاده از پشته غیرهمزمان Tokio برای مقیاس‌پذیری بالا.
- **Split Tunneling**: مدیریت جداول مسیریابی در لینوکس، مک و ویندوز.

## 📦 نصب و اجرا

### کامپایل
```bash
cargo build --release
```

### اجرا در حالت‌های مختلف
- **حالت مستقیم (Direct)**:
  ```bash
  cargo run
  ```
- **حالت SOCKS5**:
  ```bash
  CORE_MODE=socks5 cargo run
  ```
- **حالت HTTP Proxy**:
  ```bash
  CORE_MODE=http cargo run
  ```

## ⚖️ لایسنس
MIT License - Copyright (c) 2026 Taha Tehran
