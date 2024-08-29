// BP Node: sovereign bitcoin wallet backend.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2024 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2020-2024 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2020-2024 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::net::{SocketAddr, TcpListener};

use cyphernet::addr::{HostName, NetAddr};
use cyphernet::ed25519::PublicKey;
use cyphernet::Sha256;
use netservices::node::{DisconnectReason, Metrics, NodeController, NodeCtl};
use netservices::session::CypherSession;
use netservices::{Artifact, Direction, Frame, NetAccept, NetTransport};
use reactor::{Action, ResourceId, Timestamp};

pub type RemoteAddr = NetAddr<HostName>;
pub type Session = CypherSession<PublicKey, Sha256>;

pub struct Controller {}

impl NodeController<RemoteAddr, Session, TcpListener> for Controller {
    type Frame = ();

    fn extract_actions(
        &mut self,
    ) -> impl IntoIterator<Item=Action<NetAccept<Session>, NetTransport<Session>>> {
        todo!()
    }

    fn should_accept(&mut self, remote: &RemoteAddr, time: Timestamp) -> bool { todo!() }

    fn accept(
        &mut self,
        remote: RemoteAddr,
        connection: Session::Connection,
        time: Timestamp,
    ) -> Result<Session, impl Error> {
        todo!()
    }

    fn on_listening(&mut self, socket: SocketAddr) { todo!() }

    fn on_listener_failure(&mut self, res_id: ResourceId, err: io::Error, time: Timestamp) {
        todo!()
    }

    fn on_established(
        &mut self,
        remote_id: <Session::Artifact as Artifact>::NodeId,
        addr: RemoteAddr,
        direction: Direction,
        time: Timestamp,
    ) {
        todo!()
    }

    fn on_disconnected(
        &mut self,
        remote_id: <Session::Artifact as Artifact>::NodeId,
        direction: Direction,
        reason: &DisconnectReason,
    ) {
        todo!()
    }

    fn on_unbound(&mut self, listener: NetAccept<Session, NetAccept<Session>>) { todo!() }

    fn on_tick(
        &mut self,
        time: Timestamp,
        metrics: &HashMap<<Session::Artifact as Artifact>::NodeId, Metrics>,
    ) {
        todo!()
    }

    fn on_command(&mut self, cmd: NodeCtl) { todo!() }

    fn on_timer(&mut self) { todo!() }

    fn on_frame(&mut self, req: Self::Frame) { todo!() }

    fn on_frame_unparsable(&mut self, err: &<Self::Frame as Frame>::Error) { todo!() }
}
