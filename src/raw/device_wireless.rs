//! # DBus interface proxy for: `org.freedesktop.NetworkManager.Device.Wireless`
//!
//! This code was generated by `zbus-xmlgen` `3.1.1` from DBus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Wireless.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Device.Wireless",
    assume_defaults = true
)]
pub trait DeviceWireless {
    /// GetAccessPoints method
    fn get_access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// GetAllAccessPoints method
    fn get_all_access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// RequestScan method
    fn request_scan(
        &self,
        options: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
    ) -> zbus::Result<()>;

    /// AccessPointAdded signal
    #[dbus_proxy(signal)]
    fn access_point_added(&self, access_point: zbus::zvariant::ObjectPath<'_>) -> zbus::Result<()>;

    /// AccessPointRemoved signal
    #[dbus_proxy(signal)]
    fn access_point_removed(
        &self,
        access_point: zbus::zvariant::ObjectPath<'_>,
    ) -> zbus::Result<()>;

    /// AccessPoints property
    #[dbus_proxy(property)]
    fn access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;

    /// ActiveAccessPoint property
    #[dbus_proxy(property)]
    fn active_access_point(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Bitrate property
    #[dbus_proxy(property)]
    fn bitrate(&self) -> zbus::Result<u32>;

    /// HwAddress property
    #[dbus_proxy(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// LastScan property
    #[dbus_proxy(property)]
    fn last_scan(&self) -> zbus::Result<i64>;

    /// Mode property
    #[dbus_proxy(property)]
    fn mode(&self) -> zbus::Result<u32>;

    /// PermHwAddress property
    #[dbus_proxy(property)]
    fn perm_hw_address(&self) -> zbus::Result<String>;

    /// WirelessCapabilities property
    #[dbus_proxy(property)]
    fn wireless_capabilities(&self) -> zbus::Result<u32>;
}
