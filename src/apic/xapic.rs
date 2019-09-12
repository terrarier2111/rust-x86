//! Information about the xAPIC for the local APIC.
//!
//! Table 10-1 Local APIC Register Address Map
//! the MMIO base values are found in this file.

use bit_field::BitField;

///	Local APIC ID register. Read-only. See Section 10.12.5.1 for initial values.
pub const XAPIC_ID: u32 = 0x020;

///	Local APIC Version register. Read-only. Same version used in xAPIC mode and x2APIC mode.
pub const XAPIC_VERSION: u32 = 0x030;

///	Task Priority Register (TPR). Read/write. Bits 31:8 are reserved.
pub const XAPIC_TPR: u32 = 0x080;

///	Processor Priority Register (PPR). Read-only.
pub const XAPIC_PPR: u32 = 0x0A0;

///	EOI register. Write-only.
pub const XAPIC_EOI: u32 = 0x0B0;

///	Logical Destination Register (LDR). Read/write in xAPIC mode.
pub const XAPIC_LDR: u32 = 0x0D0;

/// Spurious Interrupt Vector Register (SVR). Read/write. See Section 10.9 for reserved bits.
pub const XAPIC_SVR: u32 = 0x0F0;

/// In-Service Register (ISR); bits 31:0. Read-only.
pub const XAPIC_ISR0: u32 = 0x100;

/// ISR bits 63:32. Read-only.
pub const XAPIC_ISR1: u32 = 0x110;

/// ISR bits 95:64. Read-only.
pub const XAPIC_ISR2: u32 = 0x120;

/// ISR bits 127:96. Read-only.
pub const XAPIC_ISR3: u32 = 0x130;

/// ISR bits 159:128. Read-only.
pub const XAPIC_ISR4: u32 = 0x140;

/// ISR bits 191:160. Read-only.
pub const XAPIC_ISR5: u32 = 0x150;

/// ISR bits 223:192. Read-only.
pub const XAPIC_ISR6: u32 = 0x160;

/// ISR bits 255:224. Read-only.
pub const XAPIC_ISR7: u32 = 0x170;

/// Trigger Mode Register (TMR); bits 31:0. Read-only.
pub const XAPIC_TMR0: u32 = 0x180;

/// TMR bits 63:32. Read-only.
pub const XAPIC_TMR1: u32 = 0x190;

/// TMR bits 95:64. Read-only.
pub const XAPIC_TMR2: u32 = 0x1A0;

/// TMR bits 127:96. Read-only.
pub const XAPIC_TMR3: u32 = 0x1B0;

/// TMR bits 159:128. Read-only.
pub const XAPIC_TMR4: u32 = 0x1C0;

/// TMR bits 191:160. Read-only.
pub const XAPIC_TMR5: u32 = 0x1D0;

/// TMR bits 223:192. Read-only.
pub const XAPIC_TMR6: u32 = 0x1E0;

/// TMR bits 255:224. Read-only.
pub const XAPIC_TMR7: u32 = 0x1F0;

/// Interrupt Request Register (IRR); bits 31:0. Read-only.
pub const XAPIC_IRR0: u32 = 0x200;

/// IRR bits 63:32. Read-only.
pub const XAPIC_IRR1: u32 = 0x210;

/// IRR bits 95:64. Read-only.
pub const XAPIC_IRR2: u32 = 0x220;

/// IRR bits 127:96. Read-only.
pub const XAPIC_IRR3: u32 = 0x230;

/// IRR bits 159:128. Read-only.
pub const XAPIC_IRR4: u32 = 0x240;

/// IRR bits 191:160. Read-only.
pub const XAPIC_IRR5: u32 = 0x250;

/// IRR bits 223:192. Read-only.
pub const XAPIC_IRR6: u32 = 0x260;

/// IRR bits 255:224. Read-only.
pub const XAPIC_IRR7: u32 = 0x270;

/// Error Status Register (ESR). Read/write. See Section 10.5.3.
pub const XAPIC_ESR: u32 = 0x280;

/// LVT CMCI register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_CMCI: u32 = 0x2F0;

/// Interrupt Command Register (ICR). Read/write. See Figure 10-28 for reserved bits
pub const XAPIC_ICR0: u32 = 0x300;

/// Interrupt Command Register (ICR). Read/write. See Figure 10-28 for reserved bits
pub const XAPIC_ICR1: u32 = 0x310;

/// LVT Timer register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_TIMER: u32 = 0x320;

/// LVT Thermal Sensor register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_THERMAL: u32 = 0x330;

/// LVT Performance Monitoring register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_PMI: u32 = 0x340;

/// LVT LINT0 register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_LINT0: u32 = 0x350;

/// LVT LINT1 register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_LINT1: u32 = 0x360;

/// LVT Error register. Read/write. See Figure 10-8 for reserved bits.
pub const XAPIC_LVT_ERROR: u32 = 0x370;

/// Initial Count register (for Timer). Read/write.
pub const XAPIC_TIMER_INIT_COUNT: u32 = 0x380;

/// Current Count register (for Timer). Read-only.
pub const XAPIC_TIMER_CURRENT_COUNT: u32 = 0x390;

/// Divide Configuration Register (DCR; for Timer). Read/write. See Figure 10-10 for reserved bits.
pub const XAPIC_TIMER_DIV_CONF: u32 = 0x3E0;

use super::*;
use crate::msr::{rdmsr, wrmsr, IA32_APIC_BASE, IA32_TSC_DEADLINE};
use core::intrinsics::{volatile_load, volatile_store};

#[derive(Copy, Clone)]
#[allow(dead_code, non_camel_case_types)]
enum ApicRegister {
    XAPIC_ID = XAPIC_ID as isize,
    XAPIC_VERSION = XAPIC_VERSION as isize,
    XAPIC_TPR = XAPIC_TPR as isize,
    XAPIC_PPR = XAPIC_PPR as isize,
    XAPIC_EOI = XAPIC_EOI as isize,
    XAPIC_LDR = XAPIC_LDR as isize,
    XAPIC_SVR = XAPIC_SVR as isize,
    XAPIC_ISR0 = XAPIC_ISR0 as isize,
    XAPIC_ISR1 = XAPIC_ISR1 as isize,
    XAPIC_ISR2 = XAPIC_ISR2 as isize,
    XAPIC_ISR3 = XAPIC_ISR3 as isize,
    XAPIC_ISR4 = XAPIC_ISR4 as isize,
    XAPIC_ISR5 = XAPIC_ISR5 as isize,
    XAPIC_ISR6 = XAPIC_ISR6 as isize,
    XAPIC_ISR7 = XAPIC_ISR7 as isize,
    XAPIC_TMR0 = XAPIC_TMR0 as isize,
    XAPIC_TMR1 = XAPIC_TMR1 as isize,
    XAPIC_TMR2 = XAPIC_TMR2 as isize,
    XAPIC_TMR3 = XAPIC_TMR3 as isize,
    XAPIC_TMR4 = XAPIC_TMR4 as isize,
    XAPIC_TMR5 = XAPIC_TMR5 as isize,
    XAPIC_TMR6 = XAPIC_TMR6 as isize,
    XAPIC_TMR7 = XAPIC_TMR7 as isize,
    XAPIC_IRR0 = XAPIC_IRR0 as isize,
    XAPIC_IRR1 = XAPIC_IRR1 as isize,
    XAPIC_IRR2 = XAPIC_IRR2 as isize,
    XAPIC_IRR3 = XAPIC_IRR3 as isize,
    XAPIC_IRR4 = XAPIC_IRR4 as isize,
    XAPIC_IRR5 = XAPIC_IRR5 as isize,
    XAPIC_IRR6 = XAPIC_IRR6 as isize,
    XAPIC_IRR7 = XAPIC_IRR7 as isize,
    XAPIC_ESR = XAPIC_ESR as isize,
    XAPIC_LVT_CMCI = XAPIC_LVT_CMCI as isize,
    XAPIC_ICR0 = XAPIC_ICR0 as isize,
    XAPIC_ICR1 = XAPIC_ICR1 as isize,
    XAPIC_LVT_TIMER = XAPIC_LVT_TIMER as isize,
    XAPIC_LVT_THERMAL = XAPIC_LVT_THERMAL as isize,
    XAPIC_LVT_PMI = XAPIC_LVT_PMI as isize,
    XAPIC_LVT_LINT0 = XAPIC_LVT_LINT0 as isize,
    XAPIC_LVT_LINT1 = XAPIC_LVT_LINT1 as isize,
    XAPIC_LVT_ERROR = XAPIC_LVT_ERROR as isize,
    XAPIC_TIMER_INIT_COUNT = XAPIC_TIMER_INIT_COUNT as isize,
    XAPIC_TIMER_CURRENT_COUNT = XAPIC_TIMER_CURRENT_COUNT as isize,
    XAPIC_TIMER_DIV_CONF = XAPIC_TIMER_DIV_CONF as isize,
}

/// State for the XAPIC driver.
#[derive(Debug)]
pub struct XAPIC {
    /// Reference to the xAPCI region
    mmio_region: &'static mut [u32],
    /// Initial APIC Base register value.
    base: u64,
}

impl XAPIC {
    /// Create a new xAPIC object for the local CPU.
    ///
    /// Pass the xAPCI region which is at XXX unless you have
    /// relocated the region.
    pub fn new(apic_region: &'static mut [u32]) -> XAPIC {
        unsafe {
            XAPIC {
                mmio_region: apic_region,
                base: rdmsr(IA32_APIC_BASE),
            }
        }
    }

    /// Attach driver to the xAPIC (enables device).
    pub fn attach(&mut self) {
        // Enable
        unsafe {
            self.base = rdmsr(IA32_APIC_BASE);
            self.base.set_bit(11, true); // Enable xAPIC
            wrmsr(IA32_APIC_BASE, self.base);
        }
    }

    /// Detach driver form the xAPIC (disables device).
    pub fn detach(&mut self) {
        unsafe {
            self.base = rdmsr(IA32_APIC_BASE);
            self.base.set_bit(11, false); // Disable xAPIC
            wrmsr(IA32_APIC_BASE, self.base);
        }
    }

    /// Read a register from the MMIO region.
    fn read(&self, offset: ApicRegister) -> u32 {
        assert!(offset as usize % 4 == 0);
        let index = offset as usize / 4;
        unsafe { volatile_load(&self.mmio_region[index]) }
    }

    /// write a register in the MMIO region.
    fn write(&mut self, offset: ApicRegister, val: u32) {
        assert!(offset as usize % 4 == 0);
        let index = offset as usize / 4;
        unsafe { volatile_store(&mut self.mmio_region[index], val) }
    }

    /// Is this the bootstrap core?
    pub fn bsp(&self) -> bool {
        (self.base & (1 << 8)) > 0
    }

    /// Read local APIC ID.
    pub fn id(&self) -> u32 {
        self.read(ApicRegister::XAPIC_ID)
    }

    /// Read APIC version.
    pub fn version(&self) -> u32 {
        self.read(ApicRegister::XAPIC_VERSION)
    }

    /// Acknowledge interrupt delivery.
    pub fn eoi(&mut self) {
        self.write(ApicRegister::XAPIC_EOI, 0);
    }

    /// Enable TSC timer.
    pub unsafe fn tsc_enable(&mut self) {
        let mut lvt: u32 = self.read(ApicRegister::XAPIC_LVT_TIMER);
        lvt.set_bit(17, false);
        lvt.set_bit(18, true);
        self.write(ApicRegister::XAPIC_LVT_TIMER, lvt);
    }

    /// Set TSC deadline value.
    pub unsafe fn tsc_set(&self, value: u64) {
        wrmsr(IA32_TSC_DEADLINE, value);
    }

    /// Send an init IPI.
    ///
    /// TODO: hard-coded target core.
    /// TODO: Interface will change.
    pub unsafe fn ipi_init(&mut self) {
        let icr = Icr::new(
            0,
            1,
            DestinationShorthand::NoShorthand,
            DeliveryMode::Init,
            DestinationMode::Physical,
            DeliveryStatus::Idle,
            Level::Assert,
            TriggerMode::Level,
        );
        self.send_ipi(icr);
    }

    /// De-assert init IPI.
    ///
    /// TODO: hard-coded target core.
    /// TODO: probably interface will change.
    pub unsafe fn ipi_init_deassert(&mut self) {
        let icr = Icr::new(
            0,
            0,
            // INIT deassert is always sent to everyone, so we are supposed to specify:
            DestinationShorthand::AllIncludingSelf,
            DeliveryMode::Init,
            DestinationMode::Physical,
            DeliveryStatus::Idle,
            Level::Deassert,
            TriggerMode::Level,
        );
        self.send_ipi(icr);
    }

    /// Send startup IPI.
    ///
    /// TODO: hard-coded target core.
    /// TODO: probably interface will change.
    pub unsafe fn ipi_startup(&mut self, start_page: u8) {
        let icr = Icr::new(
            start_page,
            1,
            DestinationShorthand::NoShorthand,
            DeliveryMode::StartUp,
            DestinationMode::Physical,
            DeliveryStatus::Idle,
            Level::Assert,
            TriggerMode::Edge,
        );
        self.send_ipi(icr);
    }

    /// Send generic IPI.
    unsafe fn send_ipi(&mut self, icr: Icr) {
        self.write(ApicRegister::XAPIC_ESR, 0);
        self.write(ApicRegister::XAPIC_ESR, 0);

        // 10.6 ISSUING INTERPROCESSOR INTERRUPTS
        self.write(ApicRegister::XAPIC_ICR1, icr.upper());
        self.write(ApicRegister::XAPIC_ICR0, icr.lower());

        loop {
            let icr = self.read(ApicRegister::XAPIC_ICR0);
            if (icr >> 12 & 0x1) == 0 {
                break;
            }
            if self.read(ApicRegister::XAPIC_ESR) > 0 {
                break;
            }
        }
    }
}