use crate::drivers::gpu::gop::Color;

pub fn array_return() -> [Color; 500] {
let mut array: [Color; 500] = [Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x223123), Color::hex(0x323123), Color::hex(0x123123), Color::hex(0x123123), Color::hex(0x148323), Color::hex(0x123123), Color::hex(0x134123), Color::hex(0x123123), Color::hex(0x123123)];
return array;
}