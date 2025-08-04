pub struct Factory {
    employees: i32,
    value: i32,
    purchasing_power: i32,
    location: Location,
}

pub struct Location {
    /// Unique Id location not sure If needed might drop later.
    id: i32,
    /// Local exhange rate is measuered on rate of 1/100 rate.
    local_exchangerate: i32,
    /// The rate the resource appears at.
    iron_rate: i32,
    /// the amount of currently discovered resources.
    iron: i32, 

}
