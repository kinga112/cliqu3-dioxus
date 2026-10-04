use std::collections::HashMap;

use anyhow::Result;
use dioxus::{
    core::{Runtime, RuntimeGuard},
    signals::ReadableExt,
};
use iroh::{Endpoint, endpoint::presets};
use iroh_gossip::ALPN as GOSSIP_ALPN;
use iroh_live::{
    Live,
    media::{
        AudioBackend, audio_backend::DeviceId, codec::AudioCodec, format::AudioPreset,
        publish::LocalBroadcast, source_spec::AudioSourceSpec::Device, subscribe::AudioTrack,
    },
    rooms::{Room, RoomEvent, RoomHandle, RoomTicket},
};

use crate::{
    modules::docs::db::VoiceChannel,
    states::{server_states::VOICE_CHANNELS, user_states::USER},
};

pub struct CallHandler {
    live: Live,
    pub audio: AudioBackend,
    display_name: String,
    pub active_rooms: Vec<RoomHandle>,
}

impl CallHandler {
    pub async fn new() -> Result<Self> {
        let endpoint = Endpoint::builder(presets::N0)
            .alpns(vec![iroh_live::ALPN.to_owned(), GOSSIP_ALPN.to_owned()])
            .bind()
            .await?;

        let live = Live::builder(endpoint).with_gossip().with_router().spawn();
        let audio = AudioBackend::default();
        let display_name = USER
            .read()
            .profile
            .clone()
            .expect("No user profile initialized?")
            .address;

        Ok(Self {
            live,
            audio,
            display_name,
            active_rooms: Vec::new(),
        })
    }

    pub async fn create_room(&self, voice_channel_name: String) -> anyhow::Result<String> {
        let endpoint = Endpoint::builder(presets::N0)
            .alpns(vec![iroh_live::ALPN.to_owned(), GOSSIP_ALPN.to_owned()])
            .bind()
            .await?;
        let live = Live::builder(endpoint).with_gossip().with_router().spawn();
        let ticket = RoomTicket::generate();
        let room = Room::new(&live, ticket.clone())
            .await
            .expect("failed to create room");

        let local_broadcast = LocalBroadcast::new();
        let audio = AudioBackend::default();
        let input = audio
            .default_input()
            .await
            .expect("failed to get audio mic");
        local_broadcast
            .audio()
            .set(input, AudioCodec::Opus, [AudioPreset::Hq])
            .expect("failed to broadcast audio");
        let (mut events, handle) = room.split();
        handle
            .set_display_name("The Creator")
            .await
            .expect("failed to set display name");
        let ticket = handle.ticket().to_string();
        // println!("created room ticket: {:?}", handle.ticket().to_string());
        // send room events to dioxus runtime for state updates
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<RoomEvent>();

        tokio::task::spawn(async move {
            println!("CREATE: room task started");
            handle
                .publish("hello", &local_broadcast)
                .await
                .expect("failed to public to room");
            // let _handle = handle;
            let mut active_audio: HashMap<_, AudioTrack> = HashMap::new();
            while let Some(event) = events.recv().await {
                if let RoomEvent::BroadcastSubscribed { session, broadcast } = &event {
                    if let Ok(audio_track) = broadcast.audio(&audio).await {
                        audio_track.set_volume(1.0);
                        active_audio.insert(session.remote_id(), audio_track);
                    }
                }
                let _ = tx.send(event);
            }
            println!("CREATE: room task ended — recv() loop exited");
        });

        let name_clone = voice_channel_name.clone();
        dioxus::prelude::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    RoomEvent::PeerJoined {
                        remote,
                        display_name,
                    } => {
                        // MY_GLOBAL_SIGNAL.write().push(remote);
                        // CallHandler::add_user_to_call(
                        //     name_clone.clone(),
                        //     ticket.to_string(),
                        //     remote.to_string(),
                        //     display_name.expect("failed to get display name"),
                        // );
                    }
                    RoomEvent::PeerLeft { remote } => {
                        // MY_GLOBAL_SIGNAL.write().retain(|r| r != &remote);
                        // CallHandler::remove_user_from_call(
                        //     name_clone.clone(),
                        //     ticket.to_string(),
                        //     remote.to_string(),
                        // );
                    }
                    // ... other variants you care about in the UI
                    _ => {}
                }
            }
        });

        Ok(ticket)
    }

    // pub async fn join_room(endpoint: Endpoint, ticket: RoomTicket) -> anyhow::Result<()> {
    pub async fn join_room(
        &self,
        voice_channel_name: String,
        ticket: RoomTicket,
    ) -> anyhow::Result<()> {
        // let endpoint = Endpoint::bind(presets::N0)
        //     .await
        //     .expect("failed to create endpoint in create room");

        let endpoint = Endpoint::builder(presets::N0)
            .alpns(vec![iroh_live::ALPN.to_owned(), GOSSIP_ALPN.to_owned()])
            .bind()
            .await?;

        let live = Live::builder(endpoint).with_gossip().with_router().spawn();
        let room = Room::new(&live, ticket.clone())
            .await
            .expect("failed to create room");
        let local_broadcast = LocalBroadcast::new();
        let audio = AudioBackend::default();
        let input = audio
            .default_input()
            .await
            .expect("failed to get audio mic");
        local_broadcast
            .audio()
            .set(input, AudioCodec::Opus, [AudioPreset::Hq])
            .expect("failed to broadcast audio");
        // println!("JOIN: room task started");
        let (mut events, handle) = room.split();
        let name = self.display_name.clone();
        handle
            .set_display_name(name)
            .await
            .expect("failed to set display name");
        // send room events to dioxus runtime for state updates
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<RoomEvent>();
        tokio::task::spawn(async move {
            println!("JOIN: room task started");
            handle
                .publish("hello", &local_broadcast)
                .await
                .expect("failed to public to room");
            // let _handle = handle;
            let mut active_audio: HashMap<_, AudioTrack> = HashMap::new();
            while let Some(event) = events.recv().await {
                if let RoomEvent::BroadcastSubscribed { session, broadcast } = &event {
                    if let Ok(audio_track) = broadcast.audio(&audio).await {
                        audio_track.set_volume(1.0);
                        active_audio.insert(session.remote_id(), audio_track);
                    }
                }
                let _ = tx.send(event);
            }
            println!("JOIN: room task ended — recv() loop exited");
        });

        let name_clone = voice_channel_name.clone();
        dioxus::prelude::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    RoomEvent::PeerJoined {
                        remote,
                        display_name,
                    } => {
                        // MY_GLOBAL_SIGNAL.write().push(remote);
                        CallHandler::add_user_to_call(
                            name_clone.clone(),
                            ticket.to_string(),
                            remote.to_string(),
                            display_name.expect("failed to get display name"),
                        );
                    }
                    RoomEvent::PeerLeft { remote } => {
                        // MY_GLOBAL_SIGNAL.write().retain(|r| r != &remote);
                        CallHandler::remove_user_from_call(
                            name_clone.clone(),
                            ticket.to_string(),
                            remote.to_string(),
                        );
                    }
                    // ... other variants you care about in the UI
                    _ => {}
                }
            }
        });

        Ok(())
    }

    pub fn add_call(name: String, ticket: String) {
        println!("Adding call!!");
        let mut voice_channels = VOICE_CHANNELS.write();
        // voice_channels.insert(ticket, HashMap::new());
        let voice_channel = VoiceChannel {
            name: name.clone(),
            ticket,
            active_users: HashMap::new(),
        };
        voice_channels.insert(name, voice_channel);
    }

    // Display name will be users wallet address
    fn add_user_to_call(
        name: String,
        ticket: String,
        user_public_key: String,
        user_display_name: String,
    ) {
        println!("Adding user to call: {:?}", user_display_name);
        let mut voice_channels = VOICE_CHANNELS.write();
        if let Some(voice_channel) = voice_channels.get_mut(&name) {
            println!("ADDING TO CALL! voice channel: {:?}", voice_channel.name);
            voice_channel
                .active_users
                .insert(user_public_key, user_display_name);
            // voice_channel.insert(user_public_key, user_display_name);
        }
    }

    fn remove_user_from_call(name: String, ticket: String, user_public_key: String) {
        println!("Removing user to call: {:?}", user_public_key);
        let mut voice_channels = VOICE_CHANNELS.write();
        if let Some(voice_channel) = voice_channels.get_mut(&name) {
            voice_channel.active_users.remove(&user_public_key);
        }
    }
}
