use std::time::Duration;

use rodio::{source::TakeDuration, Sample, Source};

trait SourceUpgrade {
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

/// Internal function that builds a `FadeOut` object.
pub fn FadeOut<I>(input: I, duration: Duration, inputs_duration: f32) -> FadeOut<I>
where
    I: Source,
    I::Item: Sample,
{
    let duration = duration.as_secs() * 1000000000 + duration.subsec_nanos() as u64;
    //let input_duration = input.total_duration().unwrap().as_secs() * 1000000000
    //+ input.total_duration().unwrap().subsec_nanos() as u64;
    FadeOut {
        input,
        ns_from_start: -(inputs_duration * 1000000000.0 - duration as f32),
        total_ns: duration as f32,
    }
}

/// Filter that modifies raises the volume from silence over a time period.
#[derive(Clone, Debug)]
pub struct FadeOut<I> {
    input: I,
    ns_from_start: f32,
    total_ns: f32,
}

impl<I> FadeOut<I>
where
    I: Source,
    I::Item: Sample,
{
    /// Returns a reference to the inner source.
    #[inline]
    pub fn inner(&self) -> &I {
        &self.input
    }

    /// Returns a mutable reference to the inner source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        &mut self.input
    }

    /// Returns the inner source.
    #[inline]
    pub fn into_inner(self) -> I {
        self.input
    }
}

impl<I> Iterator for FadeOut<I>
where
    I: Source,
    I::Item: Sample,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let mut factor = 0.05;

        self.ns_from_start +=
            1000000000.0 / (self.input.sample_rate() as f32 * self.channels() as f32);
        if self.ns_from_start < 0.0 {
            factor = 1.0;
        }
        self.input.next().map(|value| value.amplify(factor))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.input.size_hint()
    }
}

impl<I> ExactSizeIterator for FadeOut<I>
where
    I: Source + ExactSizeIterator,
    I::Item: Sample,
{
}

impl<I> Source for FadeOut<I>
where
    I: Source,
    I::Item: Sample,
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        self.input.current_frame_len()
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.input.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.input.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.input.total_duration()
    }
}
