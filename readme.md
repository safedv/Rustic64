# Rustic64

**Rustic64** is a 64-bit, position-independent shellcode template inspired by the design principles of [Stardust](https://github.com/Cracked5pider/Stardust). Unlike traditional methods, this template adopts a fully **position-independent** architecture tailored for the **Windows** environment, providing a modern and flexible solution for position-independent implant development.

A common challenge with position-independent implants is managing global variables or raw strings. **Rustic64** addresses this by introducing a global instance to maintain state across different parts of the shellcode, enabling seamless access to APIs, modules, configuration data, and more.

In addition, **Rustic64** incorporates a custom allocator that uses the native NT Heap API. Initialized with `RtlCreateHeap` and managed through functions like `RtlAllocateHeap` and `RtlFreeHeap`, this allocator allows for the use of heap-allocated types like `Vec` and `String` in a position-independent context, integrated via a global instance.

This project is primarily a personal learning journey in modern implant development. It is shared in the spirit of collaboration and growth, inviting feedback, suggestions, and improvements from the community.

## Disclaimer

This project is intended **for educational and research purposes only**. It is designed to showcase a modern approach to implant design using Rust and should not be used for any illegal or unethical activities. The code provided here is a demonstration template, and the creators of this repository are not responsible for any misuse of this information.

Always make sure to follow ethical guidelines and legal frameworks when conducting any security research.

## Credits

- Inspired by [Stardust](https://github.com/Cracked5pider/Stardust) by [Cracked5pider](https://github.com/Cracked5pider). A big thanks to the creator for sharing his work.
- Thanks to [@0x64616e](https://x.com/0x64616e/status/1769723870867509531) for sharing the technique used for managing global context without syscalls.

## Contributions

Contributions are welcome to help enhance the capabilities of Rustic64. If you'd like to contribute new features or report bugs, feel free to open a pull request or an issue in the repository.

---
