use std::time::Duration;

trait DurationExt {
    fn ext_secs_f64_precision(&self, precision: usize) -> String;
}

impl DurationExt for Duration {
    /// 格式化秒数输出，指定小数位数
    fn ext_secs_f64_precision(&self, precision: usize) -> String {
        format!("{:.*}", precision, self.as_secs_f64())
    }
}

pub fn display(duration: Duration) -> String {
    format!("{}s", duration.ext_secs_f64_precision(3))
}
