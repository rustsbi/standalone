# rustsbi/standalone

RustSBI Standalone is the main module of the RustSBI Prototyping System.
It targets to provide product grade SBI binary packages
to allow SoC development fast and effective.
Other than an environment with fine grained control to features, this project
aims to get your product working in an afternoon.

## Download binary

You will find download link here: TODO. You may either download the Universal
Package, or find your mother board to download a dedicated one.

To download your package into target motherboard, you may find help here: TODO.
It includes detailed description of how to run and debug SBI binary on those
motherboards.

If you are considering to link this project with following bootloading steps,
you may need to build this project. See next section to find out how to achieve this.

## Build this project

This project use the Rust programming language and Cargo package manager.
If you are new to Rust, you should prepare a Rust environment first before working on.

### Install Rust and Cargo

To build Rust projects you need to have Rust compiler installed.
Install them using Rustup [here](https://rustup.rs),
or install Rustc and Cargo package using tools provided by your system distribution.

To check if you have Rust installed and configured, use:

```shell
rustc -V && cargo -V
```

Clone this repository before further steps:

```shell
git clone git@github.com:rustsbi/standalone.git
cd standalone
```

### Build Universal support package 

Use following command:

```shell
cargo make
```

In this command you are not targeting any mainboards, so a universal binary
is built. After completed, you will find your SBI binary in `target` directory.

### Build package for dedicated board

Use this command:

```shell
cargo make --board <BOARD> [--jump <ADDRESS>]
```

You need to provide board name only, vendor name is not used here.
You will find your SBI binary in `target` directory.

The SBI binary will jump to certain address after initialized. Board code would
provide default jump address, or you may use `--jump` to override it using
hex or decimal address.

### Link external packages and build

Use this command:

```shell
cargo make [--board <BOARD>] --next <PACKAGE>
```

You may provide path to package as static linked library, like `.o` on Unix.
Build your custom library first, and this command will link it to this project.
If the command does not provide `--board`, it will build a universal package.
Since the following bootloading steps would be highly relevant to platforms,
providing `--board` will allow further optimizations and trims on RustSBI layer.

## Project scope

This project will include extensions, features and open source drivers
targeted to SBI services. Although named 'Prototyping', this solution still
aims at production grade thus *does not only focus on prototypes*.
The name 'Prototyping' is to compared with discrete bootloaders or
RustSBI implementation crates, which would be complete bootloading projects
who allow to get vendor specific modules and finer grained control of features.

RustSBI Standalone can be used as bootloading environment or one bootloading
step of the complete environment.
In this case, the target of RustSBI Standalone is to provide binary distribution as:

- Universal binary package. We fix in *common* drivers useful in SBI environment
  to get it work on any platforms with those drivers. We limit build target to
  RV32IM/RV64IM to ensure more RISC-V devices are capable to run this package;
- Platform specific jump package. We include drivers for a specific platform
  to allow loading following bootloading steps in a fixed address. This will
  allow binary concatenation with following bootloading steps or kernels;
- Package with externally linked steps. We use a library for following steps,
  link it to RustSBI/standalone to provide a bootable package.

To build hypervisors, emulators or bootloading environment projects in Rust,
you may consult `rustsbi` crate other than this project. The `rustsbi` crate
acts as a dependency to save time on SBI structures and constants if you are
working on such a project.
