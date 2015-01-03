extern crate time;
use std::fmt;
use time::time::precise_time_ns;
use std::io::fs::File;
use std::collections::HashMap;
use std::cell::RefCell;

pub struct TimeDB {
    times: HashMap<&'static str, InvocationTracking>,
}

pub struct InvocationTracking {
    count: u64,
    total_time: u64,
}

impl fmt::Show for InvocationTracking {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let avg_time = match self.count == 0 {
            true => 0.0f64,
            false => self.total_time as f64 / self.count as f64,
        };
        write!(f, "{} | {}", self.count, avg_time)
    }
}


impl InvocationTracking {
    fn new(t: &TimeIt) -> InvocationTracking {
        let mut i = InvocationTracking {
            count: 0,
            total_time: 0,
        };
        i.add_time(t);
        return i;
    }
    fn add_time(&mut self, t: &TimeIt) {
        let dur = precise_time_ns() - t.start_time;
        self.count += 1;
        self.total_time += dur;
    }
}


impl TimeDB {
    fn add_time(&mut self, time: &TimeIt) {
        println!("add_time");
        if !self.times.contains_key(time.name) {
            self.times.insert(time.name, InvocationTracking::new(time));
        } else {
            self.times.get_mut(time.name).unwrap().add_time(time);
        }
    }
}

impl fmt::Show for TimeDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("Printing timedb");
        for (name, res) in self.times.iter() {
            write!(f, "{} => {}", name, res);
        }
        write!(f, "")
    }
}


pub struct TimeIt {
    start_time: u64,
    name: &'static str,
}

impl TimeIt {
    pub fn new(name: &'static str) -> TimeIt {
        TimeIt {
            start_time: precise_time_ns(),
            name: name,
        }
    }
}

impl Drop for TimeIt {
    fn drop(&mut self) {
//        TIMES.add_time(self);
    }
}

pub struct TimeFileSave {
    file_name: &'static str,
}

impl Drop for TimeFileSave {
    fn drop(&mut self) {
        println!("Filesave");
/*        match File::create(&Path::new(self.file_name)) {
            Err(_) => {}
            Ok(mut f) => {
//                let m1 : TimeDB = TimeDB{times: TIMES.times};
//                write!(&mut f, "{}", m1);
            }
        };*/
    }
}

impl TimeFileSave {
    pub fn new(file_name: &'static str) -> TimeFileSave {
        TimeFileSave {
            file_name: file_name,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use time::TimeIt;
    use time::TimeFileSave;
    #[ignore]
    #[test]
    fn test_times() {
        time_3();
        TimeFileSave::new("name");
        panic!("Should fail");
    }

    fn time_1() -> u64{
        let _ = TimeIt::new("time_1");
        let a = 3u64;
        return a * 3;
    }

    fn time_3() {
        TimeIt::new("time_3");
        println!("before time_1");
        time_1();
        println!("after time_1");
    }
}
