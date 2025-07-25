//! Audio passthrough using sai peripheral and imxrt-hal.
//!
//! Line-in to Line-out with the SAI peripheral
//! and an SGTL5000 codec. Tested with Teensy 4.1 and its own audio board (rev D).
//!
//! The audio stream itself is expected to be a 48000Hz 16bit stereo signal.
//!
//! Pinout:
//! Teensy       PCM5102
//! --------------------
//! GND          GND
//! 3.3V         VIN
//! Pin7         DIN
//! Pin18        SDA
//! Pin19        SCL
//! Pin20        LRCK
//! Pin21        BCK
//! Pin23        MCLK
//!

#![no_main]
#![no_std]

#[rtic::app(device = board, peripherals = false, dispatchers = [BOARD_SWTASK0])]
mod app {

    //
    // Configure the demo below.
    //

    const FRONTEND: board::logging::Frontend = board::logging::Frontend::Log;
    const BACKEND: board::logging::Backend = board::logging::BACKEND;

    const LPUART_POLL_INTERVAL_MS: u32 = board::PIT_FREQUENCY / 1_000 * 4;

    /// How frequently (milliseconds) should we poll audio
    const AUDIO_POLL_MS: u32 = 1000 * (board::PIT_FREQUENCY / 1_000);

    #[cfg(feature = "real-hal")]
    use imxrt_hal::{self as hal};
    use sgtl5000::*;
    type SaiTx = hal::sai::Tx<1, 16, 2, hal::sai::PackingNone>;
    type SaiRx = hal::sai::Rx<1, 16, 2, hal::sai::PackingNone>;

    //
    // End configurations.
    //

    #[local]
    struct Local {
        led: board::Led,
        poll_log: hal::pit::Pit<1>,

        /// This timer tells us how frequently work on audio.
        audio_pit: hal::pit::Pit<2>,

        /// Sample counter for the wave generation
        counter: u32,
        dac_cp: SGTL5000<board::I2c>,
    }

    #[shared]
    struct Shared {
        /// Serial audio interface
        sai1_tx: SaiTx,
        sai1_rx: SaiRx,
        poller: board::logging::Poller,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut cortex_m = cx.core;
        let (
            board::Common {
                pit: (_, mut poll_log, mut audio_pit, _),
                usb1,
                usbnc1,
                usbphy1,

                mut dma,
                ..
            },
            board::Specifics {
                led,
                sai1,
                console,
                mut i2c,
                ..
            },
        ) = board::new();

        if BACKEND == board::logging::Backend::Lpuart {
            poll_log.set_load_timer_value(LPUART_POLL_INTERVAL_MS);
            poll_log.set_interrupt_enable(true);
            poll_log.enable();
        } else {
            poll_log.disable();
        }

        let usbd = hal::usbd::Instances {
            usb: usb1,
            usbnc: usbnc1,
            usbphy: usbphy1,
        };

        let dma_a = dma[board::BOARD_DMA_A_INDEX].take().unwrap();
        let poller = board::logging::init(FRONTEND, BACKEND, console, dma_a, usbd);

        let mut sai_config = hal::sai::SaiConfig::i2s(hal::sai::bclk_div(8));
        sai_config.sync_mode = hal::sai::SyncMode::RxFollowTx;
        let (Some(sai1_tx), Some(sai1_rx)) = sai1.split(&sai_config) else {
            panic!("Unexpected return from sai split");
        };

        let mut sai1_tx: SaiTx = sai1_tx;
        let mut sai1_rx: SaiRx = sai1_rx;

        let regs = sai1_tx.reg_dump();
        defmt::println!(
            "Regdump of config: TCR1: {:b}, TCR2 {:b}, TCR3 {:b}, TCR4 {:b}, TCR5 {:b}, TCSR: {:b}",
            regs[0],
            regs[1],
            regs[2],
            regs[3],
            regs[4],
            regs[5]
        );

        cortex_m.DCB.enable_trace();
        cortex_m::peripheral::DWT::unlock();
        cortex_m.DWT.enable_cycle_counter();

        audio_pit.set_load_timer_value(AUDIO_POLL_MS);
        audio_pit.set_interrupt_enable(true);
        audio_pit.enable();

        let mut counter: u32 = 0;
        for _i in 0..31 {
            sai1_tx.write_frame(0, [0, 0]);
            counter += 1;
        }
        sai1_tx.set_enable(true);
        sai1_tx.set_interrupts(
            hal::sai::Interrupts::FIFO_WARNING | hal::sai::Interrupts::FIFO_REQUEST,
        );
        for _i in 0..31 {
            let mut rx_data = [0u16; 2];
            sai1_rx.read_frame(0, &mut rx_data);
        }
        sai1_rx.set_enable(true);
        sai1_rx.set_interrupts(
            hal::sai::Interrupts::FIFO_WARNING | hal::sai::Interrupts::FIFO_REQUEST,
        );

        sai1_tx.set_enable(true);
        sai1_rx.set_enable(true);

        i2c.set_controller_enable(true);

        let mut dac_cp = SGTL5000::new(i2c, 0x0A);
        _ = dac_cp.init(
            crate::app::SampleRate::Hz48000,
            crate::app::I2sDataLength::Bits16,
        );

        (
            Shared {
                sai1_tx,
                sai1_rx,
                poller,
            },
            Local {
                led,
                poll_log,
                audio_pit,
                dac_cp,
                counter,
            },
        )
    }

    #[task(binds = BOARD_SAI1, shared = [sai1_tx, sai1_rx], local = [counter, led, received: [u16;2] = [0u16;2]], priority = 2)]
    fn sai1_interrupt(mut cx: sai1_interrupt::Context) {
        let sai1_interrupt::LocalResources {
            counter,
            led,
            received,
            ..
        } = cx.local;

        cx.shared.sai1_rx.lock(|sai1_rx| {
            sai1_rx.clear_status(hal::sai::Status::FIFO_ERROR | hal::sai::Status::WORD_START); //TODO: figure out why FIFO error happens
            while sai1_rx.status().contains(hal::sai::Status::FIFO_REQUEST) {
                sai1_rx.read_frame(0, received);
            }
        });
        cx.shared.sai1_tx.lock(|sai1_tx| {
            sai1_tx.clear_status(hal::sai::Status::FIFO_ERROR | hal::sai::Status::WORD_START); //TODO: figure out why FIFO error happens
            while sai1_tx.status().contains(hal::sai::Status::FIFO_REQUEST) {
                sai1_tx.write_frame(0, *received);
                *counter = (*counter).wrapping_add(1);
            }
            if (*counter % 10000) == 0 {
                led.toggle();
            }
        });
    }

    #[task(binds = BOARD_USB1, priority = 1)]
    fn usb_interrupt(_: usb_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    #[task(binds = BOARD_DMA_A, priority = 1)]
    fn dma_interrupt(_: dma_interrupt::Context) {
        poll_logger::spawn().unwrap();
    }

    /// Actually performs the poll call.
    #[task(shared = [poller], priority = 2)]
    async fn poll_logger(mut cx: poll_logger::Context) {
        cx.shared.poller.lock(|poller| poller.poll());
    }

    #[task(binds = BOARD_PIT, shared = [sai1_tx, sai1_rx], local = [audio_pit, poll_log, dac_cp], priority = 1)]
    fn pit_interrupt(cx: pit_interrupt::Context) {
        let pit_interrupt::LocalResources {
            audio_pit,
            poll_log,
            dac_cp,
            ..
        } = cx.local;

        while audio_pit.is_elapsed() {
            audio_pit.clear_elapsed();
        }

        dac_cp.dump_device_config();

        // Is it time for us to poll the logger?
        // This only happens for the LPUART backend.
        if poll_log.is_elapsed() {
            while poll_log.is_elapsed() {
                poll_log.clear_elapsed();
            }
            poll_logger::spawn().unwrap();
        }
    }
}
