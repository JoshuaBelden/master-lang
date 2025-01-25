use time::{ PrimitiveDateTime as DateTime, Duration};

pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}
