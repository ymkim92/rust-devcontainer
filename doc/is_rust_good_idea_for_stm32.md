Using Rust for firmware development on STM32F7 is a promising option, but whether it is a good idea depends on several factors. Below are the key considerations for deciding if Rust is a good fit for your production firmware:

### 1. **Safety and Reliability**
- **Memory Safety**: One of the strongest arguments for using Rust in embedded systems is its focus on memory safety. Rust's ownership model helps prevent common issues like null pointer dereferences, data races, and buffer overflows, which are critical in firmware.
- **Concurrency**: Rust’s ownership and concurrency model ensure that data races are avoided at compile time, which is particularly useful when working with interrupts and real-time systems on STM32F7.

**Pros**: Safer, less error-prone code, especially for memory management and concurrency.

### 2. **Performance**
- **Low-Level Control**: Rust is a systems programming language that provides the same level of low-level control as C/C++. Its performance is comparable to C because of its zero-cost abstractions. Optimizations in Rust can result in very efficient binaries, suitable for resource-constrained embedded environments.
- **Bare-Metal Programming**: The Rust embedded ecosystem (`cortex-m`, `cortex-m-rt`, and HAL crates) allows you to write bare-metal code without a standard library, just like C, while still benefiting from Rust’s safety features.

**Pros**: High performance with safety guarantees.

### 3. **Ecosystem Maturity**
- **Rust Embedded Crates**: The Rust embedded ecosystem is growing rapidly, and for STM32F7, there are several well-supported libraries like `stm32f7xx-hal`, `cortex-m`, and `cortex-m-rt`. These provide high-level access to hardware peripherals, but they may not cover everything you need.
- **Community and Support**: While Rust has strong community support, C/C++ has decades of ecosystem development, including extensive vendor support, tools, and libraries. For STM32, the C ecosystem is still much more mature with tools like STM32CubeMX for peripheral initialization, extensive driver libraries, and widespread community usage.

**Cons**: Rust's ecosystem for embedded development is less mature than C/C++, especially for advanced STM32 peripherals.

### 4. **Tooling**
- **Debugging**: Tools like `probe-rs` and `cargo-embed` provide good support for flashing, debugging, and profiling firmware. However, the Rust tooling isn't as mature as the C/C++ counterparts (e.g., STM32CubeIDE or Keil), especially for features like real-time debugging and performance profiling.
- **Integration with STM32 Tools**: C/C++ development integrates seamlessly with vendor tools like STM32CubeMX, which is widely used to configure peripherals and generate initialization code. Rust lacks native integration with these tools, so you may need to manually configure the hardware peripherals.

**Cons**: Tooling for Rust is improving but still behind C/C++ for embedded development.

### 5. **Learning Curve**
- **Familiarity**: If your team is already familiar with Rust, adopting Rust for firmware development is easier. However, if your team has extensive experience with C/C++ and limited Rust knowledge, the learning curve might be steep.
- **RTIC**: Rust's RTIC (Real-Time Interrupt-driven Concurrency) framework is a modern way to manage concurrency in embedded systems, but it may require a new way of thinking compared to FreeRTOS or CMSIS-RTOS used in C.

**Cons**: Rust has a steeper learning curve for embedded development, especially if your team is used to C/C++.

### 6. **Interfacing with C/C++ Libraries**
If you need to use existing C/C++ libraries, Rust has good FFI (Foreign Function Interface) support, allowing you to call C functions from Rust. This can be useful if you want to reuse certain STM32 peripheral drivers written in C.

**Pros**: Rust can call C functions, so you can reuse existing libraries.

### 7. **Long-Term Support**
- **Vendor Support**: STM32 development is heavily centered around C, with official STMicroelectronics documentation, examples, and libraries being primarily written in C. Rust is community-driven in this space and lacks official vendor support from STMicroelectronics.
- **Community and Longevity**: While the Rust embedded community is growing, C/C++ has been the standard for decades, and its continued use in embedded systems is guaranteed for the foreseeable future. Depending on how much you value long-term support and stability, this might be a concern.

**Cons**: Less vendor support compared to C/C++.

### Conclusion: Should You Use Rust for Production Firmware on STM32F7?

- **When Rust is a Good Idea**:
  - If safety (especially memory safety) is a top priority for your project.
  - If your team is proficient with Rust, or you're building a new team that could benefit from learning Rust.
  - If you want to avoid common issues with concurrency and low-level memory management.
  - If you're working on projects that demand high reliability (e.g., safety-critical systems).

- **When C/C++ Might Be a Better Fit**:
  - If your project depends heavily on vendor support (e.g., STM32CubeMX or HAL libraries).
  - If your team already has extensive experience with C/C++.
  - If you need robust debugging and development tools that are not yet fully available for Rust.

In summary, **Rust can be a very good choice for firmware development on STM32F7**, but you should weigh the benefits of memory safety and modern language features against the maturity of the ecosystem and tooling support compared to C/C++. For production systems, it would make sense to evaluate your team's skillset and the complexity of your firmware needs before making the switch to Rust.