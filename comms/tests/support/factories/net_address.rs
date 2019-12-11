//  Copyright 2019 The Tari Project
//
//  Redistribution and use in source and binary forms, with or without modification, are permitted provided that the
//  following conditions are met:
//
//  1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following
//  disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the
//  following disclaimer in the documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its contributors may be used to endorse or promote
//  products derived from this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES,
//  INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
//  DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
//  SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
//  SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY,
//  WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
//  USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

use super::{TestFactory, TestFactoryError};
use multiaddr::{AddrComponent, Multiaddr};
use std::iter::repeat_with;
use tari_test_utils::address::get_next_local_port;

pub fn create_many(n: usize) -> NetAddressesFactory {
    NetAddressesFactory::default().with_count(n)
}

pub fn create() -> NetAddressFactory {
    NetAddressFactory::default()
}

#[derive(Default, Clone)]
pub struct NetAddressesFactory {
    count: Option<usize>,
    net_address_factory: NetAddressFactory,
}

impl NetAddressesFactory {
    factory_setter!(with_count, count, Option<usize>);

    factory_setter!(with_net_address_factory, net_address_factory, NetAddressFactory);

    fn make_one(&self) -> Multiaddr {
        self.net_address_factory.clone().build().unwrap()
    }
}

impl TestFactory for NetAddressesFactory {
    type Object = Vec<Multiaddr>;

    fn build(self) -> Result<Self::Object, TestFactoryError> {
        Ok(repeat_with(|| self.make_one())
            .take(self.count.or(Some(1)).unwrap())
            .collect())
    }
}

//---------------------------------- NetAddressFactory --------------------------------------------//

#[derive(Clone)]
pub struct NetAddressFactory {
    port: Option<u16>,
    host: Option<String>,
    is_use_os_port: bool,
}

impl Default for NetAddressFactory {
    fn default() -> Self {
        Self {
            port: None,
            host: None,
            is_use_os_port: false,
        }
    }
}

impl NetAddressFactory {
    factory_setter!(with_port, port, Option<u16>);

    factory_setter!(with_host, host, Option<String>);

    pub fn use_os_port(mut self) -> Self {
        self.is_use_os_port = true;
        self
    }
}

impl TestFactory for NetAddressFactory {
    type Object = Multiaddr;

    fn build(self) -> Result<Self::Object, TestFactoryError> {
        let mut addr = format!("/ip4/{}", self.host.unwrap_or("127.0.0.1".to_string()))
            .parse::<Multiaddr>()
            .map_err(TestFactoryError::build_failed())?;

        let is_use_os_port = self.is_use_os_port;
        addr.append(AddrComponent::TCP(self.port.unwrap_or_else(|| {
            if is_use_os_port {
                0
            } else {
                get_next_local_port()
            }
        })));

        Ok(addr)
    }
}
