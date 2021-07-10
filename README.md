# spinix
A unix-like kernel written in the Rust Language!

I'm also following a [tutorial](https://os.phil-opp.com/) if anybody wants to follow the same one.

>The name "`Spinix`" originates from my older attempt at writing a kernel named "`Spice Kernel`", a kernel made using [Philipp Oppermann's](https://github.com/phil-opp) [Rust kernel tutorial](https://os.phil-opp.com/). The term spice originates from MemorySpice, a Cheat Engine like rewrite in Rust, though it was never made due to the [winapi crate](https://crates.io/crates/winapi) being broken at the time. Ever since then I've decided to follow along with [Philipp Oppermann's](https://github.com/phil-opp) [Rust kernel tutorial](https://os.phil-opp.com/) again.

## Specs / Features

- Keyboard: PS/2 Only, USB may work depending on your BIOS settings
- Timer: Full backward-compatibilty PIC Integration
- GPU: VGA Text buffer only, GUI coming soon
- CPU Interrupts: Full compatibilty with basic interrupts, Fault, Double Fault, Timer, Keyboard
- Little Paging Support

<details>
  <summary>Tested PC's</summary>
  
  Alienware Alpha (by @datkat21): <br>
  Computer model: Alienware Alpha<br>
  Processor:<br>
      4th Generation Intel Dual Core i3<br>
      4th Generation Intel Quad Core i5 ✓<br>
      4th Generation Intel Quad Core i7 <br>
  Chipset: Intel H81<br>
  DMI speed: 5.0 GT/s<br>
  Processor data width: 64 bits<br>
  GPU: <br>


  HP Pavillion (by @actuallyexeon): <br>
  Computer model: dm1<br>
  Processor: AMD Dual-Core Processor E-350<br>
  GPU: Radeon HD 6310 M Graphics, up to 1460 MB total graphics memory<br>
  Memory: 3 GB DDR3 System Memory (2 DIMM)<br>
  Memory Max: 8 GB<br>
  For more info [click here](https://support.hp.com/us-en/document/c02830923/)<br>


  Custom Build (by @actuallyexeon): <br>
  Processor: AMD Ryzen 7 2700X 8-core<br>
  GPU: Nvidia GTX 1060, 6gb of VRAM<br>
  Memory: 16GB of DDR4 Ram<br>
  <br>
  
</details>