extern crate betterposture;

fn main() {
    loop {
        betterposture::timer::sleep_mn(20);
        betterposture::notification::persistant_notification("It's time to stand up",
                                                             "Come back in 5 minutes",
                                                             "");
        betterposture::timer::sleep_mn(5);
        betterposture::notification::persistant_notification("You're safe",
                                                             "You can resume working",
                                                             "");
    }
}
