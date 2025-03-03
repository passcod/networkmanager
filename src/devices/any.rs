use super::{BridgeDevice, EthernetDevice, GenericDevice, VethDevice, WiFiDevice};
use crate::configs::{Dhcp4Config, Dhcp6Config, Ip4Config, Ip6Config};
use crate::connection::Connection;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDevice;
use crate::types::{CapabilityFlags, ConnectivityState, DeviceInterfaceFlags, DeviceType};
use num_traits::FromPrimitive;

type HashMapDBusVariant =
    std::collections::HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>>;

type HashMapDBusVariantStr<'a> = std::collections::HashMap<
    &'a str,
    std::collections::HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>,
>;

pub trait Any {
    fn reapply(
        &self,
        connection: HashMapDBusVariantStr<'_>,
        version_id: u64,
        flags: u32,
    ) -> Result<(), Error>;
    fn get_applied_connection(
        &self,
        flags: u32,
    ) -> Result<(::std::collections::HashMap<String, HashMapDBusVariant>, u64), Error>;
    fn disconnect(&self) -> Result<(), Error>;
    fn delete(&self) -> Result<(), Error>;
    fn udi(&self) -> Result<String, Error>;
    fn interface(&self) -> Result<String, Error>;
    fn ip_interface(&self) -> Result<String, Error>;
    fn driver(&self) -> Result<String, Error>;
    fn driver_version(&self) -> Result<String, Error>;
    fn firmware_version(&self) -> Result<String, Error>;
    fn capabilities(&self) -> Result<CapabilityFlags, Error>;
    fn ip4_address(&self) -> Result<u32, Error>;
    fn state(&self) -> Result<u32, Error>;
    fn state_reason(&self) -> Result<(u32, u32), Error>;
    fn active_connection(&self) -> Result<Connection, Error>;
    fn ip4_config(&self) -> Result<Ip4Config, Error>;
    fn dhcp4_config(&self) -> Result<Dhcp4Config, Error>;
    fn ip6_config(&self) -> Result<Ip6Config, Error>;
    fn dhcp6_config(&self) -> Result<Dhcp6Config, Error>;
    fn managed(&self) -> Result<bool, Error>;
    fn set_managed(&self, value: bool) -> Result<(), Error>;
    fn autoconnect(&self) -> Result<bool, Error>;
    fn set_autoconnect(&self, value: bool) -> Result<(), Error>;
    fn firmware_missing(&self) -> Result<bool, Error>;
    fn nm_plugin_missing(&self) -> Result<bool, Error>;
    fn device_type(&self) -> Result<DeviceType, Error>;
    fn available_connections(&self) -> Result<Vec<Connection>, Error>;
    fn physical_port_id(&self) -> Result<String, Error>;
    fn mtu(&self) -> Result<u32, Error>;
    fn metered(&self) -> Result<u32, Error>;
    fn lldp_neighbors(&self) -> Result<Vec<HashMapDBusVariant>, Error>;
    fn real(&self) -> Result<bool, Error>;
    fn ip4_connectivity(&self) -> Result<ConnectivityState, Error>;
    fn ip6_connectivity(&self) -> Result<ConnectivityState, Error>;
    fn interface_flags(&self) -> Result<DeviceInterfaceFlags, Error>;
    fn hw_address(&self) -> Result<String, Error>;
}

macro_rules! impl_any {
    ($name:ty) => {
        impl Any for $name {
            fn reapply(
                &self,
                connection: HashMapDBusVariantStr<'_>,
                version_id: u64,
                flags: u32,
            ) -> Result<(), Error> {
                Ok(proxy!(self).reapply(connection, version_id, flags)?)
            }
            fn get_applied_connection(
                &self,
                flags: u32,
            ) -> Result<(std::collections::HashMap<String, HashMapDBusVariant>, u64), Error> {
                Ok(proxy!(self).get_applied_connection(flags)?)
            }
            fn disconnect(&self) -> Result<(), Error> {
                Ok(proxy!(self).disconnect()?)
            }
            fn delete(&self) -> Result<(), Error> {
                Ok(proxy!(self).delete()?)
            }
            fn udi(&self) -> Result<String, Error> {
                Ok(proxy!(self).udi()?)
            }
            fn interface(&self) -> Result<String, Error> {
                Ok(proxy!(self).interface()?)
            }
            fn ip_interface(&self) -> Result<String, Error> {
                Ok(proxy!(self).ip_interface()?)
            }
            fn driver(&self) -> Result<String, Error> {
                Ok(proxy!(self).driver()?)
            }
            fn driver_version(&self) -> Result<String, Error> {
                Ok(proxy!(self).driver_version()?)
            }
            fn firmware_version(&self) -> Result<String, Error> {
                Ok(proxy!(self).firmware_version()?)
            }
            fn capabilities(&self) -> Result<CapabilityFlags, Error> {
                let cap = proxy!(self).capabilities()?;
                Ok(CapabilityFlags::from_bits_retain(cap))
            }
            fn ip4_address(&self) -> Result<u32, Error> {
                Ok(proxy!(self).ip4_address()?)
            }
            fn state(&self) -> Result<u32, Error> {
                Ok(proxy!(self).state()?)
            }
            fn state_reason(&self) -> Result<(u32, u32), Error> {
                Ok(proxy!(self).state_reason()?)
            }
            fn active_connection(&self) -> Result<Connection, Error> {
                let path = proxy!(self).active_connection()?;
                Ok(Connection::new(self.dbus_accessor.with_path(path)))
            }
            fn ip4_config(&self) -> Result<Ip4Config, Error> {
                let path = proxy!(self).ip4_config()?;
                Ok(Ip4Config::new(self.dbus_accessor.with_path(path)))
            }
            fn dhcp4_config(&self) -> Result<Dhcp4Config, Error> {
                let path = proxy!(self).dhcp4_config()?;
                Ok(Dhcp4Config::new(self.dbus_accessor.with_path(path)))
            }
            fn ip6_config(&self) -> Result<Ip6Config, Error> {
                let path = proxy!(self).ip6_config()?;
                Ok(Ip6Config::new(self.dbus_accessor.with_path(path)))
            }
            fn dhcp6_config(&self) -> Result<Dhcp6Config, Error> {
                let path = proxy!(self).dhcp6_config()?;
                Ok(Dhcp6Config::new(self.dbus_accessor.with_path(path)))
            }
            fn managed(&self) -> Result<bool, Error> {
                Ok(proxy!(self).managed()?)
            }
            fn set_managed(&self, value: bool) -> Result<(), Error> {
                Ok(proxy!(self).set_managed(value)?)
            }
            fn autoconnect(&self) -> Result<bool, Error> {
                Ok(proxy!(self).autoconnect()?)
            }
            fn set_autoconnect(&self, value: bool) -> Result<(), Error> {
                Ok(proxy!(self).set_autoconnect(value)?)
            }
            fn firmware_missing(&self) -> Result<bool, Error> {
                Ok(proxy!(self).firmware_missing()?)
            }
            fn nm_plugin_missing(&self) -> Result<bool, Error> {
                Ok(proxy!(self).nm_plugin_missing()?)
            }
            fn device_type(&self) -> Result<DeviceType, Error> {
                let dev_type = proxy!(self).device_type()?;
                match FromPrimitive::from_u32(dev_type) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn available_connections(&self) -> Result<Vec<Connection>, Error> {
                let paths = proxy!(self).available_connections()?;
                let mut connections = Vec::with_capacity(paths.len());
                for path in paths {
                    connections.push(Connection::new(self.dbus_accessor.with_path(path)));
                }
                Ok(connections)
            }
            fn physical_port_id(&self) -> Result<String, Error> {
                Ok(proxy!(self).physical_port_id()?)
            }
            fn mtu(&self) -> Result<u32, Error> {
                Ok(proxy!(self).mtu()?)
            }
            fn lldp_neighbors(&self) -> Result<Vec<HashMapDBusVariant>, Error> {
                Ok(proxy!(self).lldp_neighbors()?)
            }
            fn metered(&self) -> Result<u32, Error> {
                Ok(proxy!(self).metered()?)
            }
            fn real(&self) -> Result<bool, Error> {
                Ok(proxy!(self).real()?)
            }
            fn ip4_connectivity(&self) -> Result<ConnectivityState, Error> {
                let con = proxy!(self).ip4_connectivity()?;
                match FromPrimitive::from_u32(con) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn ip6_connectivity(&self) -> Result<ConnectivityState, Error> {
                let con = proxy!(self).ip6_connectivity()?;
                match FromPrimitive::from_u32(con) {
                    Some(x) => Ok(x),
                    None => Err(Error::UnsupportedType),
                }
            }
            fn interface_flags(&self) -> Result<DeviceInterfaceFlags, Error> {
                let interface_flag = proxy!(self).interface_flags()?;
                Ok(DeviceInterfaceFlags::from_bits_retain(interface_flag))
            }
            fn hw_address(&self) -> Result<String, Error> {
                Ok(proxy!(self).hw_address()?)
            }
        }
    };
}

impl_any!(VethDevice);
impl_any!(BridgeDevice);
impl_any!(WiFiDevice);
impl_any!(EthernetDevice);
impl_any!(GenericDevice);
