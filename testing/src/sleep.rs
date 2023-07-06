use std::time::Duration;

pub fn sleep()
{
    std::thread::sleep(Duration::from_millis(500));
}