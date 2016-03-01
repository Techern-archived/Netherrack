### Netherrack (Server)
## A Minecraft server implementation programmed in Rust

[![Build Status](https://travis-ci.org/Techern/Netherrack.svg?branch=master)](https://travis-ci.org/Techern/Netherrack)  [![Crates.io](https://img.shields.io/crates/v/netherrack.svg)](https://crates.io/crates/netherrack)
[![Coverage Status](https://coveralls.io/repos/Techern/Netherrack/badge.svg?branch=master&service=github)](https://coveralls.io/github/Techern/Netherrack?branch=master) [![Minecraft target](https://img.shields.io/badge/MC%20Target-1.8-green.svg)]()

## About Netherrack

### What is Netherrack?

Netherrack started as a project to recreate Minecraft's server using Java (and later D 2.0), but was ultimately cancelled.

Prismarine was started in 2014 for the same reason, but due to personal issues, it never got off the ground.

Netherrack has now been resurrected as a recreation of the Minecraft server using Rust, and is required to gain the experience required to create a new generation of sandbox utilities and games.

### Can I use it?

You can certainly try! Prepare to be disappointed, though, as most functionality is not yet implemented.

By the time v0.0.5 is released, it should be playable. From there, we will keep growing bigger and better!

#### I want to use Netherrack as an embedded server. Can I do that?

Sure! Netherrack will fully support starting an embedded server. All you have to do is set the global logger and then run it :)

You may eventually have to set more options, but for now it will be nice and simple

### Are there plugins?

Not yet, but we're drafting a process that will allow a Netherrack server with plugins to interact with both vanilla and modified (with Forge, initially) clients without losing support for either.

Of course, some plugins will require a Forge client (for example, an IronChests port), which will disable vanilla clients.

### Can I help out...

#### As a programmer?

Yes, please! There is still a **lot** of work required, and even a tiny bit helps immensely.

#### As a sponsor?

Wow, I'm flattered, but we're not quite at that stage yet.

#### As a beta tester?

Of course! All versions in the 0.x.y range can be considered pre-release, but official alpha and beta builds will be introduced before 1.0.0.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
