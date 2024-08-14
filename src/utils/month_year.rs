#[derive(Clone)]
pub struct MonthYear {
    month: u8,
    year: i32,
}

impl MonthYear {
    pub fn new(month: u8, year: i32) -> MonthYear {
        MonthYear { month, year }
    }

    pub fn get_month(&self) -> u8 {
        self.month
    }
    pub fn get_year(&self) -> i32 {
        self.year
    }
    pub fn get_month_string(&self) -> String {
        let prefix = if self.month < 10 {
            "0"
        } else {
            ""
        };
        format!("{}{}", prefix, self.month)
    }
}
