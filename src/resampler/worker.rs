//! Asynchronous worker thread doing the actual resampling

use super::Async;

use std::sync::Arc;
use spu::{SampleBuffer, SAMPLE_MAX};
use std::sync::mpsc::Receiver;
use std::num::{Int, FromPrimitive};
use std::default::Default;

/// Asynchronous worker tasked doing the actual adaptative resampling
pub struct AsyncResampler<T: Send> {
    /// Channel used to receive the samples from the emulator
    source: Receiver<SampleBuffer>,
    /// Asynchronous FIFO used to send samples to the backend
    async:  Arc<Async<T>>,
    /// offset into the next buffer
    offset: f32,
}

impl<T> AsyncResampler<T>
    where T: Copy + Send + Default + Int + FromPrimitive + 'static {

    pub fn new(source:      Receiver<SampleBuffer>,
               async:      Arc<Async<T>>) -> AsyncResampler<T> {

        AsyncResampler {
            source: source,
            async: async,
            offset: 0.,
        }
    }

    pub fn resample(&mut self) {
        let range: T   = Int::max_value();
        let sample_max = <T as FromPrimitive>::from_u8(SAMPLE_MAX).unwrap();

        while let Ok(buf) = self.source.recv() {
            let mut atomic = self.async.atomic.lock().unwrap();

            // Adapt the resampling ratio based on the current FIFO
            // usage. The objective is to aim for a 50% full
            // FIFO. This is the algorithm used by libretro.
            let fifo_depth = atomic.fifo.capacity() as f32;

            let adj = (fifo_depth - (2 * atomic.fifo.len()) as f32)
                / fifo_depth;
            let adj = 1. + adj * DEVIATION;

            let factor = atomic.ratio / adj;

            let mut pos = self.offset;

            while (pos as usize) < buf.len() {
                // Nearest resampling
                let sample = buf[pos as usize];

                // Convert u8 sample to the target range by
                // "upscaling" it.
                let sample = <T as FromPrimitive>::from_u8(sample).unwrap();
                let sample = sample * (range / sample_max);

                // Push the sample in the target FIFO
                while let Err(_) = atomic.fifo.push(sample) {
                    // Destination FIFO is full, notify the reader
                    // thread that there's something to read...
                    self.async.stall.notify_one();

                    // ... and wait for it to make some room
                    atomic = self.async.stall.wait(atomic).unwrap();
                }

                // Move on to the next sample
                pos += factor;
            }

            // Update offset into the next buffer
            self.offset = pos - buf.len() as f32;

            // Notify the reader that we put samples in the FIFO
            self.async.stall.notify_one();
        }
    }
}

/// This constant says how brutal the resampling rate can
/// change. Higher values will allow for a faster variation in sample
/// rate but may cause audible changes in pitch.
const DEVIATION: f32 = 0.005;
