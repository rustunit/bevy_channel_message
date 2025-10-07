# bevy_channel_message

[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_channel_message)](https://crates.io/crates/bevy_channel_message)
[![docs.rs](https://docs.rs/bevy_channel_message/badge.svg)](https://docs.rs/bevy_channel_message)

Send events via a channel from anywhere (eg. web-dom, c-ffi) to Bevy Observers.
Based on the original [bevy_crossbeam_event](https://github.com/johanhelsing/bevy_crossbeam_event) but reverting to the buffered message system instead of migrating to observer/triggers as it did in `0.9`.

# example

```rust

```

## Our Other Crates

- [bevy_channel_trigger](https://github.com/rustunit/bevy_channel_trigger)
- [bevy_debug_log](https://github.com/rustunit/bevy_debug_log)
- [bevy_device_lang](https://github.com/rustunit/bevy_device_lang)
- [bevy_web_popups](https://github.com/rustunit/bevy_web_popups)
- [bevy_libgdx_atlas](https://github.com/rustunit/bevy_libgdx_atlas)
- [bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap)
- [bevy_ios_review](https://github.com/rustunit/bevy_ios_review)
- [bevy_ios_gamecenter](https://github.com/rustunit/bevy_ios_gamecenter)
- [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts)
- [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications)
- [bevy_ios_impact](https://github.com/rustunit/bevy_ios_impact)
- [bevy_ios_safearea](https://github.com/rustunit/bevy_ios_safearea)

## Compatible Bevy Versions

|bevy|our version|
|-|-|
|0.17|0.1,main|

## License

this crate is dual-licensed under either [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0), at your option.
