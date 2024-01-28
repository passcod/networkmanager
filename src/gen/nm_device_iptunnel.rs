// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceIPTunnel {
    fn mode(&self) -> Result<u32, dbus::Error>;
    fn parent(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn local(&self) -> Result<String, dbus::Error>;
    fn remote(&self) -> Result<String, dbus::Error>;
    fn ttl(&self) -> Result<u8, dbus::Error>;
    fn tos(&self) -> Result<u8, dbus::Error>;
    fn path_mtu_discovery(&self) -> Result<bool, dbus::Error>;
    fn input_key(&self) -> Result<String, dbus::Error>;
    fn output_key(&self) -> Result<String, dbus::Error>;
    fn encapsulation_limit(&self) -> Result<u8, dbus::Error>;
    fn flow_label(&self) -> Result<u32, dbus::Error>;
    fn fw_mark(&self) -> Result<u32, dbus::Error>;
    fn flags(&self) -> Result<u32, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopNetworkManagerDeviceIPTunnel for blocking::Proxy<'a, C>
{
    fn mode(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "Mode",
        )
    }

    fn parent(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "Parent",
        )
    }

    fn local(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "Local",
        )
    }

    fn remote(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "Remote",
        )
    }

    fn ttl(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "Ttl",
        )
    }

    fn tos(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "Tos",
        )
    }

    fn path_mtu_discovery(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "PathMtuDiscovery",
        )
    }

    fn input_key(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "InputKey",
        )
    }

    fn output_key(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "OutputKey",
        )
    }

    fn encapsulation_limit(&self) -> Result<u8, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "EncapsulationLimit",
        )
    }

    fn flow_label(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "FlowLabel",
        )
    }

    fn fw_mark(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "FwMark",
        )
    }

    fn flags(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.IPTunnel",
            "Flags",
        )
    }
}
