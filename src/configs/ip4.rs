use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerIP4Config;

#[derive(Clone, Debug)]
pub struct Ip4Config {
    dbus_accessor: DBusAccessor,
}

type HashMapDBusVariant =
    std::collections::HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>>;

impl Ip4Config {
    pub(crate) fn new(dbus_accessor: DBusAccessor) -> Self {
        Ip4Config { dbus_accessor }
    }

    pub fn addresses(&self) -> Result<Vec<Vec<u32>>, Error> {
        Ok(proxy!(self).addresses()?)
    }
    pub fn address_data(&self) -> Result<Vec<HashMapDBusVariant>, Error> {
        Ok(proxy!(self).address_data()?)
    }
    pub fn gateway(&self) -> Result<String, Error> {
        Ok(proxy!(self).gateway()?)
    }
    pub fn routes(&self) -> Result<Vec<Vec<u32>>, Error> {
        Ok(proxy!(self).routes()?)
    }
    pub fn route_data(&self) -> Result<Vec<HashMapDBusVariant>, Error> {
        Ok(proxy!(self).route_data()?)
    }
    pub fn nameservers(&self) -> Result<Vec<u32>, Error> {
        Ok(proxy!(self).nameservers()?)
    }
    pub fn nameserver_data(&self) -> Result<Vec<HashMapDBusVariant>, Error> {
        Ok(proxy!(self).nameserver_data()?)
    }
    pub fn domains(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).domains()?)
    }
    pub fn searches(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).searches()?)
    }
    pub fn dns_options(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).dns_options()?)
    }
    pub fn dns_priority(&self) -> Result<i32, Error> {
        Ok(proxy!(self).dns_priority()?)
    }
    pub fn wins_servers(&self) -> Result<Vec<u32>, Error> {
        Ok(proxy!(self).wins_servers()?)
    }
    pub fn wins_server_data(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).wins_server_data()?)
    }
}
