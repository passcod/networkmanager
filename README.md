# NetworkManager bindings for Rust (fork)

> Fork by @passcod to rewrite a few APIs to be more ergonomic.

[NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) bindings for Rust using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)

## Status

This project is still under development. Currently implemented parts can be found in the docs.

- NetworkManager D-Bus API >= v1.42.2

## Usage

Add networkmanager to your `Cargo.toml` with:

```toml
[dependencies]
networkmanager = { package = "passcod-networkmanager", version = "0.5.0" }
```

## Example

```rust,no_run
use networkmanager::devices::{Any, Device, Wired, Wireless};
use networkmanager::{Error, NetworkManager};

fn main() -> Result<(), Error> {
    let nm = NetworkManager::new()?;

    for dev in nm.get_devices()? {
        match dev {
            Device::Ethernet(x) => {
                println!("Is autoconnected: {:?}", x.autoconnect()?);
                println!("Speed: {:?}", x.speed()?);
                println!("S390 Subchannels: {:?}", x.s390_subchannels()?);
                println!("Carrier: {:?}", x.carrier()?);
            }
            Device::WiFi(x) => {
                println!("Access Point: {:?}", x.access_points()?);
            }
            _ => {}
        }
    }

    let enp0s2 = nm.get_device_by_ip_iface("enp0s2")?;
    match enp0s2 {
        Device::Ethernet(x) => {
            // NetworkManager >= 1.24
            // println!("Hardware Address: {:?}", Any::hw_address(&x)?);

            // NetworkManager < 1.24
            // println!("Hardware Address: {:?}", Wired::hw_address(&x)?);

            println!("Speed: {:?}", x.speed()?);
        }
        _ => {}
    }

    Ok(())
}
```

## Build prerequisites

- ### Debian and its derivatives (e.g. Ubuntu)

  - network-manager
  - libdbus-1-dev
  - pkg-config

- ### Fedora

  - NetworkManager
  - dbus-devel
  - pkg-config

## Todo

- Implementations
  - Devices
    - [x] Any
    - [x] Generic
    - [x] Wireless
    - [x] Wired
    - [ ] ADSL
    - [ ] Bluetooth
    - [ ] Bond
    - [x] Bridge
    - [ ] Dummy
    - [ ] Infiniband
    - [ ] IpTunnel
    - [ ] Lowpan
    - [ ] Macsec
    - [ ] MacVLAN
    - [ ] Modem
    - [ ] OLPCMesh
    - [ ] OVSBridge
    - [ ] OVSInterface
    - [ ] OVSPort
    - [ ] PPP
    - [ ] Statistics
    - [ ] Team
    - [ ] TUN/TAP
    - [x] VETH
    - [ ] VLAN
    - [ ] VRF
    - [ ] VXLAN
    - [ ] WifiP2P
    - [ ] WiMax
    - [ ] Wireguard
    - [ ] Wpan
  - Configs
    - [x] IP4
    - [x] IP6
    - [x] DHCP4
    - [x] DHCP6
  - [x] Accesspoint
  - [x] ConnectionActive
  - [ ] NetworkManager (partially implemented)
  - [ ] AgentManager
  - [ ] Checkpoint
  - [ ] DNSManager
  - [ ] PPP
  - [ ] SecretAgent
  - [ ] Settings
  - [ ] Settings Connection
  - [ ] VPN Connection
  - [ ] VPN Plugin
  - [ ] WifiP2P
  - [ ] Wimax NSP
- General
  - [ ] Improve Error handling
  - [ ] dbus::arg::Variants conversion

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
