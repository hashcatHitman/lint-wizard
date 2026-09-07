<!--
SPDX-FileCopyrightText: © 2025 hashcatHitman

SPDX-License-Identifier: Apache-2.0 OR MIT
-->

# Lint Wizard

[![unsafe forbidden]][safety dance] [![dependency badge]][deps.rs]
[![Rust Quality Control status]][Rust Quality Control workflow]
[![CodeQL]][CodeQL workflow] [![Zizmor status]][Zizmor workflow]
[![REUSE status]][REUSE workflow]

---

A tool for maintaining lint configurations.

## Getting Started

You'll need to install Rust and its package manager, Cargo, on your system.
Please refer to the official [recommended Rust installation method] for your
system.

You should also have some version of git installed. You can refer to the
[Git documentation] if you need help with that.

Clone the repository and navigate inside it:

```bash
git clone https://github.com/hashcatHitman/lint-wizard.git
cd lint-wizard
```

If you'd like to read the documentation, the recommended way to do so is with:

```bash
cargo doc --document-private-items --open
```

Which will open the documentation in your browser.

To build the project, you can do:

```bash
cargo build --profile release --locked
```

Cargo will download the dependencies and compile the project. It will probably
be located at `./target/release/lint-wizard` or
`./target/release/lint-wizard.exe`, depending on your system.

## MSRV Policy

<!-- Adapted from Arti's MSRV policy -->

Our current Minimum Supported Rust Version (MSRV) is 1.97.1.

We may increase the patch level of the MSRV on any release.

Otherwise, we will not increase MSRV on PATCH releases, though our dependencies
might.

We won't increase MSRV just because we can: we'll only do so when we have a
reason. (We don't guarantee that you'll agree with our reasoning; only that
it will exist.)

[unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[safety dance]: https://github.com/rust-secure-code/safety-dance/

[dependency badge]: https://deps.rs/repo/github/hashcatHitman/lint-wizard/status.svg
[deps.rs]: https://deps.rs/repo/github/hashcatHitman/lint-wizard

[Rust Quality Control status]: https://github.com/hashcatHitman/lint-wizard/actions/workflows/rust.yml/badge.svg
[Rust Quality Control workflow]: https://github.com/hashcatHitman/lint-wizard/actions/workflows/rust.yml

[CodeQL]: https://github.com/hashcatHitman/lint-wizard/actions/workflows/github-code-scanning/codeql/badge.svg
[CodeQL workflow]: https://github.com/hashcatHitman/lint-wizard/actions/workflows/github-code-scanning/codeql

[Zizmor status]: https://github.com/hashcatHitman/lint-wizard/actions/workflows/zizmor.yml/badge.svg
[Zizmor workflow]: https://github.com/hashcatHitman/lint-wizard/actions/workflows/zizmor.yml

[REUSE status]: https://github.com/hashcatHitman/lint-wizard/actions/workflows/reuse.yml/badge.svg
[REUSE workflow]:https://github.com/hashcatHitman/lint-wizard/actions/workflows/reuse.yml

[recommended Rust installation method]: https://www.rust-lang.org/tools/install

[Git documentation]: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git
