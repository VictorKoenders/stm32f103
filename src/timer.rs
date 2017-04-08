use peripheral::tim::tim2;
use peripheral::rcc;

pub struct Timer{}

impl Timer {
    pub fn new() -> Timer {
        let mut rcc = rcc::apb1enr::modify();
        rcc.tim2en = true;
        rcc.save();

        let mut cr1 = tim2::cr1::modify();
        cr1.opm = 0;
        cr1.cen = 1;
        cr1.save();

        let mut psc = tim2::psc::modify();
        psc.psc = 7_999;
        psc.save();

        Timer {
        }
    }

    pub fn sleep_ms(&self, time: u16) {
        let start_time = tim2::cnt::load().cnt;
        let (target_time, mut overflow_required) = start_time.overflowing_add(time);

        loop {
            let current_time = tim2::cnt::load().cnt;
            if current_time < start_time { overflow_required = false; }
            if current_time > target_time && !overflow_required { break; }
        }
    }
}
