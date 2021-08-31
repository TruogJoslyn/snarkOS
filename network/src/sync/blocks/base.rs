// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use std::{net::SocketAddr, time::Duration};

use futures::{FutureExt, pin_mut, select};
use snarkos_storage::Digest;
use snarkvm_dpc::BlockHeaderHash;
use tokio::{sync::mpsc, time::Instant};

use crate::{Node, Peer, SyncInbound};
use anyhow::*;

pub struct SyncBase {
    pub node: Node,
    pub incoming: mpsc::Receiver<SyncInbound>,
}

impl SyncBase {
    pub fn new(node: Node) -> (Self, mpsc::Sender<SyncInbound>) {
        let (sender, receiver) = mpsc::channel(256);
        let new = Self {
            node,
            incoming: receiver,
        };
        (new, sender)
    }

    pub async fn find_sync_nodes(&self) -> Result<Vec<Peer>> {
        let our_block_height = self.node.storage.canon().await?.block_height;
        let mut interesting_peers = vec![];
        for mut node in self.node.peer_book.connected_peers_snapshot().await {
            let judge_bad = node.judge_bad();
            if !judge_bad && node.quality.block_height as usize > our_block_height + 1 {
                interesting_peers.push(node);
            }
        }
        interesting_peers.sort_by(|x, y| y.quality.block_height.cmp(&x.quality.block_height));

        // trim nodes close to us if any are > 10 blocks ahead
        if let Some(i) = interesting_peers
            .iter()
            .position(|x| x.quality.block_height as usize <= our_block_height + 10)
        {
            interesting_peers.truncate(i + 1);
        }

        if !interesting_peers.is_empty() {
            info!("found {} interesting peers for sync", interesting_peers.len());
            trace!("sync interesting peers = {:?}", interesting_peers);
        }

        Ok(interesting_peers)
    }

    /// receives an arbitrary amount of inbound sync messages with a given timeout.
    /// if the passed `handler` callback returns `true`, then the loop is terminated early.
    /// if the sync stream closes, the loop is also terminated early.
    pub async fn receive_messages<F: FnMut(SyncInbound) -> bool> (
        &mut self,
        timeout_sec: u64,
        moving_timeout_sec: u64,
        mut handler: F,
    ) {
        let end = Instant::now() + Duration::from_secs(timeout_sec);
        let mut moving_end = Instant::now() + Duration::from_secs(moving_timeout_sec);
        loop {
            let timeout = tokio::time::sleep_until(end.min(moving_end)).fuse();
            pin_mut!(timeout);
            select! {
                msg = self.incoming.recv().fuse() => {
                    if msg.is_none() {
                        break;
                    }
                    metrics::decrement_gauge!(snarkos_metrics::queues::SYNC_ITEMS, 1.0);
                    if handler(msg.unwrap()) {
                        break;
                    }
                    moving_end += Duration::from_secs(moving_timeout_sec);
                },
                _ = timeout => {
                    break;
                }
            }
        }
    }

    pub async fn cancel_outstanding_syncs(&self, addresses: &[SocketAddr]) {
        let mut future_set = vec![];
        for addr in addresses {
            if let Some(peer) = self.node.peer_book.get_peer_handle(*addr) {
                future_set.push(async move {
                    peer.cancel_sync().await;
                });
            }
        }
        futures::future::join_all(future_set).await;
    }
}