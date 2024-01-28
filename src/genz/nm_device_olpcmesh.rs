//! # DBus interface proxy for: `org.freedesktop.NetworkManager.Device.OlpcMesh`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.OlpcMesh.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Device.OlpcMesh",
    assume_defaults = true
)]
pub trait DeviceOlpcMesh {
    /// ActiveChannel property
    #[dbus_proxy(property)]
    fn active_channel(&self) -> zbus::Result<u32>;

    /// Companion property
    #[dbus_proxy(property)]
    fn companion(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;
}
