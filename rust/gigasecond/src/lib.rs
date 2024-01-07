use {time::Duration, time::PrimitiveDateTime as DateTime};

pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1000000000)
}
