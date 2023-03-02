struct Date {
    year: u32,
    month: usize,
    weekday: u8, 
}

struct DateIterator {
    date: Date,
    max: u32,
}

impl Iterator for DateIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        const DAYS_IN_MONTH_REGULAR: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]; 
        const DAYS_IN_MONTH_LEAP:    [u8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]; 

        // increase day month and year
        self.date.weekday = 
            if self.date.year % 4 == 0 && !(self.date.year % 100 == 0 && self.date.year % 400 != 0) {
                (self.date.weekday + DAYS_IN_MONTH_LEAP[self.date.month]) % 7 
            } else { 
                (self.date.weekday + DAYS_IN_MONTH_REGULAR[self.date.month]) % 7 
            }; 

        self.date.month += 1;
        if self.date.month >= 12 {
            self.date.month = 0; 
            self.date.year += 1; 
        }

        // return none when over max
        if self.date.year >= self.max {
            None
        }
        else {
            Some(self.date.weekday)
        }
    }
}



fn main() {
    let iterator_2001 = DateIterator {
        max : 2001,
        date : Date {
            year : 1900,
            month : 0,
            weekday: 1
        }
    }; 

    let iterator_1901 = DateIterator {
        max : 1901,
        date : Date {
            year : 1900,
            month : 0,
            weekday: 1
        }
    }; 

    let count_total = iterator_2001.filter(|weekday| *weekday == 0).count(); 
    let count_begin = iterator_1901.filter(|weekday| *weekday == 0).count(); 

    println!("sundays (total): {}\nsundays (1900): {}\nsundays (diff): {}", count_total, count_begin, count_total - count_begin);
}
