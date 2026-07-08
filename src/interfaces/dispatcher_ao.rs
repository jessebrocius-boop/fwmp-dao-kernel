// src/interfaces/dispatcher_ao.rs
use crate::interfaces::aura_telemetry::IonicFlux;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::mem::MaybeUninit;

pub const BUFFER_CAPACITY: usize = 1024;
// Mask for O(1) bitwise wrapping instead of modulo
pub const BUFFER_MASK: usize = BUFFER_CAPACITY - 1;

/// A lock-free, statically allocated ring buffer for zero-overhead telemetry handoff.
pub struct DispatcherRingBuffer {
    pub buffer: [MaybeUninit<IonicFlux>; BUFFER_CAPACITY],
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
}

impl DispatcherRingBuffer {
    pub fn new() -> Self {
        // Bypass boot-time zero-initialization overhead entirely
        let buffer = unsafe { MaybeUninit::uninit().assume_init() };
        Self {
            buffer,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn push(&mut self, flux: IonicFlux) -> Result<(), &'static str> {
        let current_tail = self.tail.load(Ordering::Relaxed);
        let next_tail = current_tail.wrapping_add(1);

        // Sync with consumer head to prevent buffer overflow
        if next_tail.wrapping_sub(self.head.load(Ordering::Acquire)) > BUFFER_CAPACITY {
            return Err("BUFFER_OVERFLOW: Ring buffer maximum capacity reached. Dropping payload.");
        }

        // O(1) bitwise index wrapping
        let index = current_tail & BUFFER_MASK;
        self.buffer[index] = MaybeUninit::new(flux);

        // Release semantic ensures memory is completely written before tail updates
        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<IonicFlux> {
        let current_head = self.head.load(Ordering::Relaxed);
        
        // Sync with producer tail to check for available payloads
        if current_head == self.tail.load(Ordering::Acquire) {
            return None;
        }

        let index = current_head & BUFFER_MASK;
        
        // Read initialized data
        let flux = unsafe { self.buffer[index].assume_init_read() };
        
        // Release head increment so the producer can reclaim the slot
        self.head.store(current_head.wrapping_add(1), Ordering::Release);
        Some(flux)
    }
}
