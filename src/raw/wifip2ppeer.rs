//! # DBus interface proxy for: `org.freedesktop.NetworkManager.WifiP2PPeer`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `org.freedesktop.NetworkManager.WifiP2PPeer.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.WifiP2PPeer",
    assume_defaults = true
)]
pub trait WifiP2PPeer {
    /// Flags property
    #[dbus_proxy(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// LastSeen property
    #[dbus_proxy(property)]
    fn last_seen(&self) -> zbus::Result<i32>;

    /// Manufacturer property
    #[dbus_proxy(property)]
    fn manufacturer(&self) -> zbus::Result<String>;

    /// Model property
    #[dbus_proxy(property)]
    fn model(&self) -> zbus::Result<String>;

    /// ModelNumber property
    #[dbus_proxy(property)]
    fn model_number(&self) -> zbus::Result<String>;

    /// Name property
    #[dbus_proxy(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Serial property
    #[dbus_proxy(property)]
    fn serial(&self) -> zbus::Result<String>;

    /// Strength property
    #[dbus_proxy(property)]
    fn strength(&self) -> zbus::Result<u8>;

    /// WfdIEs property
    #[dbus_proxy(property, name = "WfdIEs")]
    fn wfd_ies(&self) -> zbus::Result<Vec<u8>>;
}
