use crate::kernel;
use crate::kernel::cmos::CMOS;
use core::hint::spin_loop;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

const PIT_FREQUENCY: f64 = 3_579_545.0 / 3.0;
const PIT_DIVIDER: usize = 1193;
const PIT_INTERVAL: f64 = (PIT_DIVIDER as f64) / PIT_FREQUENCY; // 1ms

static PIT_TICKS: AtomicUsize = AtomicUsize::new(0);
static LAST_RTC_UPDATE: AtomicUsize = AtomicUsize::new(0);
static CLOCKS_PER_NANOSECOND: AtomicU64 = AtomicU64::new(0);

pub fn ticks() -> usize {
    PIT_TICKS.load(Ordering::Relaxed)
}

pub fn time_between_ticks() -> f64 {
    PIT_INTERVAL
}

pub fn last_rtc_update() -> usize {
    LAST_RTC_UPDATE.load(Ordering::Relaxed)
}

pub fn halt() {
    x86_64::instructions::hlt();
}

fn rdtsc() -> u64 {
    unsafe {
        core::arch::x86_64::_mm_lfence();
        core::arch::x86_64::_rdtsc()
    }
}

pub fn nanowait(nanoseconds: u64) {
    let start = rdtsc();
    let delta = nanoseconds * CLOCKS_PER_NANOSECOND.load(Ordering::Relaxed);
    while rdtsc() - start < delta {
        spin_loop();
    }
}

fn set_pit_frequency_divider(divider: u16) {
    interrupts::without_interrupts(|| {
        let bytes = divider.to_le_bytes();
        let mut cmd: Port<u8> = Port::new(0x43);
        let mut data: Port<u8> = Port::new(0x40);
        unsafe {
            cmd.write(0x36);
            data.write(bytes[0]);
            data.write(bytes[1]);
        }
    });
}

pub fn sleep(seconds: f64) {
    let start = kernel::clock::uptime();
    while kernel::clock::uptime() - start < seconds {
        halt();
    }
}

pub fn pit_interrupt_handler() {
    PIT_TICKS.fetch_add(1, Ordering::Relaxed);
}

pub fn rtc_interrupt_handler() {
    LAST_RTC_UPDATE.store(ticks(), Ordering::Relaxed);
    CMOS::new().notify_end_of_interrupt();
}

pub fn init() {
    let divider = if PIT_DIVIDER < 65536 { PIT_DIVIDER } else { 0 };
    set_pit_frequency_divider(divider as u16);
    kernel::interrupts::set_irq_handler(0, pit_interrupt_handler); //设定软时钟

    kernel::interrupts::set_irq_handler(8, rtc_interrupt_handler);

    let calibration_time = 250_000;
    let a = rdtsc();
    sleep(calibration_time as f64 / 1e6);
    let b = rdtsc();
    CLOCKS_PER_NANOSECOND.store((b - a) / calibration_time, Ordering::Relaxed);
}