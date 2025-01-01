# Isaka\_James Operating System

**Isaka\_James** is a hobby operating system written entirely in **Rust**, crafted during my free time as a personal project to understand the underlying processes of how operating systems work. The goal is to write everything from scratch, keeping the fun alive in exploring the basics of operating system development.

This is not a serious production-grade OS but rather a learning adventure. However, it works! Currently, the OS is still in its infancy, and there is a lot of room for exploration and contribution.

---

## **Current Features**

- **VGA Screen Initialization**: The OS boots up and initializes the VGA screen.
- **Login Prompt**: A basic username and login prompt is displayed on the screen.

---

## **What's Missing?**

- **Memory Management**: Not implemented yet. This is a big area where contributions are welcome.
- **File System**: Nothing yet! Feel free to dive in and propose ideas.
- **User Interaction**: Beyond the login prompt, user interaction is minimal.
- **Kernel Extensions**: Features like multi-tasking, device drivers, and interrupt handling are still ideas on the horizon.

---

## **How to Contribute**

This project thrives on collaboration! Whether you’re an experienced systems programmer or just starting out, you’re welcome to join in the fun.

### **Getting Started**

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/isaka-jame/isaka_james.git
   cd isaka_james
   ```
2. **Set Up Your Environment**:
   - Install the Rust compiler and toolchain by following instructions at [rust-lang.org](https://www.rust-lang.org/).
   - Install `bootimage` for building bootable images:
     ```bash
     cargo install bootimage
     ```
   - Install `qemu` for running the OS in a virtual machine.
3. **Build and Run**:
   - To build the bootable image:
     ```bash
     cargo bootimage
     ```
   - To run the OS:
     ```bash
     qemu-system-x86_64 -drive format=raw,file=target/x86_64-isaka-james/debug/bootimage-isaka_james.bin
     ```

### **Pull Requests**

Feel free to fork the repository and make pull requests. Contributions can include:

- Adding basic features (e.g., memory allocation, input handling).
- Fixing bugs.
- Optimizing existing code.
- Documentation improvements.

### **Issues**

Check the [Issues](https://github.com/username/isaka_james_os/issues) tab for tasks or report new bugs and suggestions.

---

## **Philosophy**

The main idea is to write everything from scratch to understand the inner workings of an operating system. If you’re someone who enjoys learning by doing and isn’t afraid of diving deep into the fundamentals, this is the project for you.

---

## **Roadmap**

Here’s what’s planned:

- Implement basic memory management.
- Develop a basic shell for user interaction.
- Experiment with file systems.
- Implement basic multitasking support.
- Extend hardware support.

---

## **Acknowledgments**

This project is inspired by the many hobby OS projects out there and is built on the foundation of learning and experimentation.

---

## **License**

This project is licensed under the MIT License. Feel free to use, modify, and distribute it as you wish.

---

Come join the fun and learn along the way!
<p>
  <img src="https://komarev.com/ghpvc/?username=isakaos&label=Visitors%20&color=0e75b6&style=flat" alt="since 1 Jan,2025" />
</p>
