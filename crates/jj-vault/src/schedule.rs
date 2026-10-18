use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Timing {
    pub quiet: Duration,
    pub maximum_delay: Duration,
    pub reconciliation: Duration,
}
impl Default for Timing {
    fn default() -> Self {
        Self {
            quiet: Duration::from_secs(2),
            maximum_delay: Duration::from_secs(30),
            reconciliation: Duration::from_secs(60),
        }
    }
}
pub(crate) struct Schedule {
    timing: Timing,
    first: Option<Instant>,
    last: Instant,
    reconciliation: Instant,
}
impl Schedule {
    pub fn new(now: Instant, timing: Timing) -> Result<Self, String> {
        if timing.quiet.is_zero()
            || timing.maximum_delay < timing.quiet
            || timing.reconciliation < timing.quiet
            || timing.maximum_delay > Duration::from_secs(3600)
            || timing.reconciliation > Duration::from_secs(86400)
        {
            return Err("Invalid vault backup timing".into());
        }
        Ok(Self {
            timing,
            first: None,
            last: now,
            reconciliation: now,
        })
    }
    pub fn changed(&mut self, now: Instant) {
        self.first.get_or_insert(now);
        self.last = now;
    }
    pub fn due(&self) -> Instant {
        self.first.map_or(self.reconciliation, |first| {
            (self.last + self.timing.quiet)
                .min(first + self.timing.maximum_delay)
                .min(self.reconciliation)
        })
    }
    pub fn completed(&mut self, now: Instant) {
        self.first = None;
        self.reconciliation = now + self.timing.reconciliation;
    }
    pub fn failed(&mut self, now: Instant) {
        self.first = Some(now);
        self.last = now;
        self.reconciliation = now + self.timing.quiet;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn captures_on_open_and_after_quiet_or_maximum_delay() {
        let now = Instant::now();
        let mut s = Schedule::new(now, Timing::default()).unwrap();
        assert_eq!(s.due(), now);
        s.completed(now);
        s.changed(now + Duration::from_secs(1));
        assert_eq!(s.due(), now + Duration::from_secs(3));
        for second in 2..40 {
            s.changed(now + Duration::from_secs(second));
        }
        assert_eq!(s.due(), now + Duration::from_secs(31));
    }
    #[test]
    fn idle_reconciliation_and_failure_retry_remain_bounded() {
        let now = Instant::now();
        let mut s = Schedule::new(now, Timing::default()).unwrap();
        s.completed(now);
        assert_eq!(s.due(), now + Duration::from_secs(60));
        s.failed(now);
        assert_eq!(s.due(), now + Duration::from_secs(2));
    }
}
