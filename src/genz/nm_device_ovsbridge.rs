//! # DBus interface proxy for: `org.freedesktop.NetworkManager.Device.OvsBridge`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.OvsBridge.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Device.OvsBridge",
    assume_defaults = true
)]
pub trait DeviceOvsBridge {
    /// Slaves property
    #[dbus_proxy(property)]
    fn slaves(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
}
