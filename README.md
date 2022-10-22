# rustsbi/standalone

RustSBI Standalone is the main module of the RustSBI Prototyping System.
Main target of this project is to provide product grade SBI binary package
to allow system development fast and effective. Other than a fine grained
function control, this project aims to get your product working in an afternoon.

## Project scope

This project will include extensions, features and open source drivers
targeted to SBI services. Although named 'Prototyping', this solution still
aims at production grade thus *does not only focus on prototypes*.
The name 'Prototyping' is to compared with discrete bootloaders or
RustSBI implementation crates, which would be complete bootloading projects
who allow to get vendor specific modules and finer grained control features.

RustSBI Standalone can be used as bootloading environment or one bootloading
step of the complete environment.
In this case, the target of RustSBI Standalone is to provide binary distribution as:

- Universal binary package. We fix in *common* drivers useful in SBI environment
  to get it work on any platforms with those drivers. We limit this package in
  RV32IM/RV64IM to ensure more RISC-V devices are capable to run this package;
- Platform specific jump package. We include drivers for a specific platform
  to allow loading following bootloading steps in a fixed address. This will
  allow binary concatenation with following bootloading steps or kernels;
- Package with externally linked steps. We use a library for following steps，
  link it to RustSBI/standalone to provide a bootable package.

To build hypervisors, emulators or bootloading environment projects in Rust,
you may consult `rustsbi` crate other than this project. The `rustsbi` crate
acts as a dependency to save time on SBI structures and constants if you are
working on such a project.
