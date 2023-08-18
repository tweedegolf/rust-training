# Module C2 - Async foundations

<a href="/slides/C-advanced-rust" target="_blank">Slides</a>

## C2.1

We're going to build an observable variable, a bit similar in idea to a condvar.

It should have the following use:

```rust
pub static CPU_TEMPERATURE: Observable<f32> = Observable::new(20.0);

async fn throttle_if_cpu_temp_high(cpu: &mut Cpu) -> ! {
    loop {
        CPU_TEMPERATURE
            .wait_until(|temperature| temperature > 90.0)
            .await;

        cpu.throttle();

        CPU_TEMPERATURE
            .wait_until(|temperature| temperature < 80.0)
            .await;

        cpu.un_throttle();
    }
}
```
