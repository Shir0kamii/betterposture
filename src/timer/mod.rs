use std::thread::sleep_ms;

pub fn sleep_secs(seconds: u32) {
    let ms = 1000 * seconds;
    sleep_ms(ms);
}

pub fn sleep_mn(minutes: u32) {
    let seconds = 60 * minutes;
    sleep_secs(seconds);
}
