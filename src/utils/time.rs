use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use chrono_tz::Asia::Bangkok;

pub fn set_global_timezone() {
    // ใน Rust การตั้งค่า Global Timezone ในระดับ OS/Env มักจะทำผ่าน TZ env var
    // เพื่อให้ library ที่เรียกใช้ localtime (ผ่าน C) ทำงานได้ถูกต้อง
    std::env::set_var("TZ", "Asia/Bangkok");
    println!("\x1b[1;34m🕒 Global timezone set to Asia/Bangkok (UTC+07:00)\x1b[0m");
}

pub fn now_bangkok() -> DateTime<Tz> {
    Utc::now().with_timezone(&Bangkok)
}

pub struct BangkokTimer;

impl tracing_subscriber::fmt::time::FormatTime for BangkokTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = now_bangkok();
        write!(w, "{}", now.format("%Y-%m-%dT%H:%M:%S%.3f+07:00"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_bangkok() {
        let time = now_bangkok();
        // Check if the timezone is Asia/Bangkok (+07:00)
        assert!(time.timezone().to_string().contains("Bangkok"));
    }

    #[test]
    fn test_set_global_timezone() {
        set_global_timezone();
        assert_eq!(std::env::var("TZ").unwrap(), "Asia/Bangkok");
    }

    #[test]
    fn test_bangkok_timer() {
        use tracing_subscriber::fmt::time::FormatTime;
        let timer = BangkokTimer;
        let mut buf = String::new();
        let mut writer = tracing_subscriber::fmt::format::Writer::new(&mut buf);
        let res = timer.format_time(&mut writer);
        assert!(res.is_ok());
        assert!(buf.contains("+07:00"));
    }
}
