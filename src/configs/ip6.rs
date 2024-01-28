use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerIP6Config;

type Ip6Address = Vec<u8>;
type Ip6Route = Vec<u8>;
type Ip6Prefix = u32;
type Ip6Gateway = Vec<u8>;
type Ip6NextHop = Vec<u8>;
type Ip6Metric = u32;
type Ip6AddressProperty = (Ip6Address, Ip6Prefix, Ip6Gateway);
type Ip6RouteProperty = (Ip6Route, Ip6Prefix, Ip6NextHop, Ip6Metric);

type HashMapDBusVariant =
    std::collections::HashMap<String, dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>>;

#[derive(Clone, Debug)]
pub struct Ip6Config {
    dbus_accessor: DBusAccessor,
}

impl Ip6Config {
    pub(crate) fn new(dbus_accessor: DBusAccessor) -> Self {
        Ip6Config { dbus_accessor }
    }

    pub fn addresses(&self) -> Result<Vec<Ip6AddressProperty>, Error> {
        Ok(proxy!(self).addresses()?)
    }
    pub fn address_data(&self) -> Result<Vec<HashMapDBusVariant>, Error> {
        Ok(proxy!(self).address_data()?)
    }
    pub fn gateway(&self) -> Result<String, Error> {
        Ok(proxy!(self).gateway()?)
    }
    pub fn routes(&self) -> Result<Vec<Ip6RouteProperty>, Error> {
        Ok(proxy!(self).routes()?)
    }
    pub fn route_data(&self) -> Result<Vec<HashMapDBusVariant>, Error> {
        Ok(proxy!(self).route_data()?)
    }
    pub fn nameservers(&self) -> Result<Vec<Ip6Address>, Error> {
        Ok(proxy!(self).nameservers()?)
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
}
