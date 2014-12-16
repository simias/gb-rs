//! Game Boy MBC3 Real Time Clock emulation
//!
//! The RTC in the cartridge is backed by a 32.568kHz oscillator so we
//! could just simulate that with the emulated clock like the other
//! counters in the game boy, however that would mean that we would
//! drift from the real time wall clock time and we wouldn't be able
//! to compute the new value when the emulator is stopped and
//! restarded from a save file later.
//!
//! Instead a unix timestamp is used to remain synchronized with the
//! wall clock and to continue "counting" when the emulator is not
//! running. That gives us good portability at the cost of some
//! possible glitches if the system clock gets adjusted at runtime.

/// RTC status
#[deriving(Copy)]
pub enum Rtc {
    /// The RTC is halted and contains the value of the counter at the
    /// moment it was last running
    Halted(i64),
    /// The RTC is running and contains a reference time corresponding
    /// to time 0 from which we can recompute the current counter
    /// value when necessary. This reference time is an UTC date since
    /// it's *mostly* monotonic while still being portable across
    /// systems (so that the date stored in savefiles still make sense
    /// 3 weeks later on some other machine on an other timezone, for
    /// instance).
    Counting(i64),
}

impl Rtc {
    /// Build a new RTC. It starts halted with a counter set to 0
    pub fn new() -> Rtc {
        Rtc::Halted(0)
    }

    /// Return the current counter value
    pub fn counter(self) -> i64 {
        match self {
            Rtc::Halted(c)         => c,
            Rtc::Counting(zeroref) => {
                // Compute the time elapsed since the timer
                // 0-reference
                let r = now() - zeroref;

                if r >= 0 {
                    r
                } else {
                    // UTC is not strictly monotonic (because of leap
                    // seconds) and the system clock might get
                    // adjusted, so we have to deal with cases where
                    // the counter seems to count "backwards".
                    0
                }
            }
        }
    }

    /// Returns a `u64` containing the entire state of the RTC. This
    /// value can then be stored in a save file and restored later
    /// since the whole point of the RTC is to keep counting while the
    /// console is off.
    pub fn dump(self) -> u64 {
        match self {
            Rtc::Halted(c) =>
                // We're not counting so we know the counter value
                // won't change between now and whenever `restore` is
                // called. We just save the counter value with the
                // high bit set to denote that we're halted.
                (c as u64) | (1 << 63),
            Rtc::Counting(zeroref) =>
                // The counter is running: we save the current
                // reference time so that we'll be able to compute the
                // difference when we're restarted.
                zeroref as u64,
        }
    }

    /// Rebuild an RTC from a value dumped with `dump`
    pub fn from_dump(val: u64) -> Rtc {
        if val & (1 << 63) != 0 {
            Rtc::Halted((val & !(1 << 63)) as i64)
        } else {
            Rtc::Counting(val as i64)
        }
    }

    /// If the Rtc is stopped, start it
    pub fn start(&mut self) {
        match self {
            &Rtc::Halted(c) => {
                // Compute the new date corresponding to the 0
                // time reference. For instance, if `c` is 300
                // then the zeroref was 300seconds ago.
                let zeroref = now() - c;

                *self = Rtc::Counting(zeroref)
            }
            _ => (),
        }
    }

    /// Stop the Rtc
    pub fn stop(&mut self) {
        *self = Rtc::Halted(self.counter());
    }

    /// Return the current RTC date
    pub fn date(self) -> Date {
        Date(self.counter())
    }
}

/// Struct represented a latched date
#[deriving(Copy)]
pub struct Date(i64);

impl Date {
    /// Retrieve the date's seconds
    pub fn seconds(self) -> u8 {
        let Date(t) = self;

        (t % 60) as u8
    }

    /// Retrieve the date's minutes
    pub fn minutes(self) -> u8 {
        let Date(t) = self;

        ((t / 60) % 60) as u8
    }

    /// Retrieve the date's hours
    pub fn hours(self) -> u8 {
        let Date(t) = self;

        ((t / (60 * 60)) % 24) as u8
    }

    /// Retrieve the date's days
    pub fn days(self) -> i64 {
        let Date(t) = self;

        t / (60 * 60 * 24)
    }
}

// Return the number of seconds since the epoch
fn now() -> i64 {
    ::time::get_time().sec
}
