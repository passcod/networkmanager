// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceVeth {
    fn peer(&self) -> Result<dbus::Path<'static>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopNetworkManagerDeviceVeth for blocking::Proxy<'a, C>
{
    fn peer(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Veth",
            "Peer",
        )
    }
}
