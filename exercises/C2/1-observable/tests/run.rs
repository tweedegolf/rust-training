use std::{sync::atomic::{AtomicBool, Ordering}, time::Duration};
use observable::Observable;

pub static CPU_TEMPERATURE: Observable<f32> = Observable::new(20.0);

async fn throttle_if_cpu_temp_high(cpu: &Cpu) -> ! {
    loop {
        CPU_TEMPERATURE
            .wait_until(|temperature| *temperature > 90.0)
            .await;

        cpu.throttle();

        CPU_TEMPERATURE
            .wait_until(|temperature| *temperature < 80.0)
            .await;

        cpu.un_throttle();
    }
}

#[tokio::test]
async fn run_cpu_test() {
    static CPU: Cpu = Cpu { throttled: AtomicBool::new(false) };

    tokio::spawn(throttle_if_cpu_temp_high(&CPU));

    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(!CPU.is_throttled());

    CPU_TEMPERATURE.set(50.0); 
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(!CPU.is_throttled());

    CPU_TEMPERATURE.set(95.0); 
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(CPU.is_throttled());

    CPU_TEMPERATURE.set(85.0); 
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(CPU.is_throttled());

    CPU_TEMPERATURE.set(75.0); 
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(CPU.is_throttled());
}

struct Cpu {
    throttled: AtomicBool,
}

impl Cpu {
    pub fn throttle(&self) {
        assert!(!self.throttled.swap(true, Ordering::Relaxed));
    }

    pub fn un_throttle(&self) {
        assert!(self.throttled.swap(false, Ordering::Relaxed));
    }

    fn is_throttled(&self) -> bool {
        self.throttled.load(Ordering::Relaxed)
    }
}
