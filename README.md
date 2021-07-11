![Spinix Image](./images/spinix-banner-hq.png)

[![Issues](https://img.shields.io/github/issues/limeyteam/spinix?color=yellow)](https://github.com/limeyteam/spinix/issues) [![License: GPL v3](https://img.shields.io/github/license/limeyteam/spinix?color=green)](https://github.com/limeyteam/spinix/blob/master/LICENSE) [![Star the project!](https://img.shields.io/github/stars/limeyteam/spinix)](https://github.com/limeyteam/spinix/blob/master/LICENSE) 

## Contributors
[@actuallyexeon](https://github.com/actuallyexeon) <br>
[@itslap](https://github.com/itslap)
<br>
[@datkat21](https://github.com/datkat21)


## About the project
UEFI Branch!
A unix-like kernel written in the Rust Language!

## Specs / Features

- UEFI support
- GPU: Basic Framebuffer support
- Small Rust panic support

<details>
  <summary>Tested PC's</summary>
  
  Alienware Alpha (by [@datkat21](https://github.com/datkat21)): <br>
  Computer model: Alienware Alpha<br>
  Processor:<br>
      4th Generation Intel Dual Core i3<br>
      4th Generation Intel Quad Core i5<br>
      4th Generation Intel Quad Core i7 <br>
  Chipset: Intel H81<br>
  DMI speed: 5.0 GT/s<br>
  Processor data width: 64 bits<br>
  GPU: <br>


  HP Pavillion (by @actuallyexeon): <br>
  Boots but graphics are completely broken<br>
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
  Works<br>
  
  Samsung Chromebook (by @actuallyexeon): <br>
  Computer model: 3<br>
  Memory: 4gb<br>
  Works perfectly using the BIOS version<br>
</details>

## Changelog
• Added some startup text saying if things have been loaded properly, src/lib.rs 25-33

• Added a "concept" of what low-level text collection would kind of look like, src/lib.rs 37-40, src/main.rs 42

• Added Interrupts, src/interrupts.rs, src/gdt.rs

• Added some page file getting, src/memory.rs

• Added serial to output to terminal when running 
``cargo test``, src/serial.rs, test/

• Added lib.rs for better file management, src/lib.rs 

• Added hello world task in lib.rs, src/lib.rs 44-46

• Added new panic handler for tests, src/lib.rs 90-93

• Added new QEMU Closer for tests, src/lib.rs 97-109

• Added CPU halter so CPU does not stay at 100%, src/lib.rs 111-114