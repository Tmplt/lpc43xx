# `lpc43xx`

> Peripheral access API for LPC43xx and LPC43Sxx microcontrollers

Original SVD file acquired from [ARM Keil](https://www.keil.com/dd2/nxp/lpc43s50/eula-container).
(Extract `SVD/LPC43xx.svd`-file from the zip archive, sha256 sum is `c4e1f82dc60a6d334db0077b0f36968be8948b4cd636d6215126aaae896d08e8`.)

Note: original SVD file has truncated enum names, some of which had to be manually made unique so that this crate compiles.
Unfortunately, there appears to be no mention of the full names in related documentation;
I've tried my best to give them unique, meaningful names.

Aside from enums, some `sfsp` registers were also not unique.
Naming scheme for these lacked rhyme or reason so a `_FIXn` suffix has been added to make them unique, `n` denoting in which order the register is listed in the file (e.g. `FIX1` appearing before `FIX2`).

If you have any improvements for the SVD file, please do submit them.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
