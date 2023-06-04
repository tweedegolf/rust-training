use lis3dh::{
    Configuration as AccConfig, DataRate, Duration, IrqPin1Config, Lis3dh, Range, SlaveAddr,
    Threshold,
};
pub use lis3dh::{Interrupt1, Lis3dhI2C};

use embedded_hal::blocking::i2c::{self, WriteRead};

/// Configure a LIS3DH accelerometer connected to a Twim peripheral.
/// After configuration, the LIS3DH will start sampling with a data rate of 400Hz,
/// and raise an interrupt on pin INT1 when one of the axes' acceleration
/// exceed 1.1g.
/// Taken from https://github.com/tweedegolf/lis3dh-interrupt-demo/blob/main/src/acc.rs
pub fn config_acc<Twim, E>(
    twim: Twim,
) -> Result<Lis3dh<Lis3dhI2C<Twim>>, lis3dh::Error<E, core::convert::Infallible>>
where
    Twim: WriteRead<Error = E> + i2c::Write<Error = E>,
{
    let config = AccConfig {
        datarate: DataRate::PowerDown,
        ..AccConfig::default()
    };

    let data_rate = DataRate::Hz_400;

    // Initialize accelerometer using the config
    let mut lis3dh = Lis3dh::new_i2c_with_config(twim, SlaveAddr::Default, config)?;

    let threshold = Threshold::g(Range::default(), 1.1);

    // Configure the threshold value for interrupt 1 to 1.1g
    lis3dh.configure_irq_threshold(Interrupt1, threshold)?;

    // The time in 1/ODR an axis value should be above threshold in order for an
    // interrupt to be raised
    let duration = Duration::miliseconds(data_rate, 0.0);
    lis3dh.configure_irq_duration(Interrupt1, duration)?;

    // Congfigure IRQ source for interrupt 1
    lis3dh.configure_irq_src(
        Interrupt1,
        lis3dh::InterruptMode::Movement,
        lis3dh::InterruptConfig::high_and_low(),
    )?;

    // Configure IRQ pin 1
    lis3dh.configure_interrupt_pin(IrqPin1Config {
        // Raise if interrupt 1 is raised
        ia1_en: true,
        // Disable for all other interrupts
        ..IrqPin1Config::default()
    })?;

    // Go to low power mode and wake up for 25ms if measurement above 1.1g is done
    let duration = Duration::miliseconds(data_rate, 2.5);
    lis3dh.configure_switch_to_low_power(threshold, duration)?;

    lis3dh.set_datarate(data_rate)?;

    Ok(lis3dh)
}
