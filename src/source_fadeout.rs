use std::time::Duration;

use rodio::{source::TakeDuration, Sample, Source};

pub trait SourceUpgrade {
    fn take_duration_fadeout(self, duration: Duration) -> TakeDuration<Self>
    where
        Self: Sized;
}

impl<S> SourceUpgrade for S
where
    S: Source,
    S::Item: Sample,
{
    fn take_duration_fadeout(self, duration: Duration) -> TakeDuration<Self> {
        let mut t = self.take_duration(duration);
        t.set_filter_fadeout();
        t
    }
}
