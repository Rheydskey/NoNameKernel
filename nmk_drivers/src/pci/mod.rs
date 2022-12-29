use bit_field::BitField;
use nmk_utils::asm;

const PCI_ADDRESS: u16 = 0xCF8;
const PCI_PORT: u16 = 0xCFC;

pub struct Pci(u32);

impl Pci {
    pub fn new(bus: u8, slot: u8, func: u8) -> Self {
        let mut result: u32 = 0;

        result.set_bits(0..3, func as u32);
        result.set_bits(3..8, slot as u32);
        result.set_bits(8..16, bus as u32);
        result.set_bits(16..32, 0);

        Self(result)
    }

    pub fn bus(&self) -> u8 {
        self.0.get_bits(8..16) as u8
    }

    pub fn slot(&self) -> u8 {
        self.0.get_bits(3..8) as u8
    }

    pub fn func(&self) -> u8 {
        self.0.get_bits(0..3) as u8
    }

    pub fn read(&self, offset: u32) -> u32 {
        let bus = self.bus() as u32;
        let slot = self.slot() as u32;
        let func = self.func() as u32;

        let address =
            ((bus << 16) | (slot << 11) | (func << 8) | (offset & 0xfc) as u32 | (0x80000000));

        asm::outl(PCI_ADDRESS, address);

        (asm::inl(PCI_PORT) >> ((offset & 2) * 8)) & 0xffff
    }

    pub fn print_vendor_device_id(&self) {
        let vendor = self.read(0);
        let device = self.read(8);
        let subclass = device.get_bits(8..16);
        crate::print!(
            "\nDevice: {:#X}, SC: {:#X} VID: {:#X}",
            device,
            subclass,
            vendor
        );
    }
}

impl core::fmt::Debug for Pci {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut result = f.debug_struct("PCI");
        result.field("Slot", &format_args!("{:X}", self.slot()));
        result.field("Bus", &format_args!("{:X}", self.bus()));
        result.field("Func", &format_args!("{:X}", self.func()));
        result.finish()
    }
}

pub fn pci_config_read_word(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let lbus = bus as u32;
    let lslot = slot as u32;
    let lfunc = func as u32;

    let address =
        ((lbus << 16) | (lslot << 11) | (lfunc << 8) | (offset & 0xfc) as u32 | (0x80000000));

    asm::outl(PCI_ADDRESS, address);

    ((asm::inl(PCI_PORT) >> ((offset & 2) * 8)) & 0xffff) as u16
}

pub fn pci_check_vendor(bus: u8, slot: u8) -> u16 {
    let device: u16;
    let vendor: u16 = pci_config_read_word(bus, slot, 0, 0);

    if vendor != 0xFFFF {
        device = pci_config_read_word(bus, slot, 0, 2);
        crate::println!("\n{:x}", vendor);
        crate::println!("{:x}", device);
    }

    vendor
}
