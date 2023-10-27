use observable::Observable;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    time::Duration,
};

async fn throttle_if_cpu_temp_high(cpu: &Cpu, cpu_temperature: Arc<Observable<f32>>) -> ! {
    loop {
        cpu_temperature
            .wait_until(|temperature| *temperature > 90.0)
            .await;

        cpu.throttle();

        cpu_temperature
            .wait_until(|temperature| *temperature < 80.0)
            .await;

        cpu.un_throttle();
    }
}

#[tokio::test]
async fn run_cpu_test() {
    let cpu_temperature = Arc::new(Observable::new(20.0));
    static CPU: Cpu = Cpu {
        throttled: AtomicBool::new(false),
    };

    tokio::spawn(throttle_if_cpu_temp_high(&CPU, cpu_temperature.clone()));

    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(!CPU.is_throttled());

    cpu_temperature.set(50.0).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(!CPU.is_throttled());

    cpu_temperature.set(95.0).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(CPU.is_throttled());

    cpu_temperature.set(85.0).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(CPU.is_throttled());

    cpu_temperature.set(75.0).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(!CPU.is_throttled());
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
