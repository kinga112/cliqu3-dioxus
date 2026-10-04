use std::collections::HashMap;

use iroh::{Endpoint, endpoint::presets};
use iroh_live::{
    Live,
    media::{
        AudioBackend,
        capture::CameraCapturer,
        codec::{AudioCodec, VideoCodec},
        format::{AudioFormat, AudioPreset, DecoderBackend, PlaybackConfig, VideoPreset},
        subscribe::AudioTrack,
        traits::{AudioSink, AudioSource},
    },
    rooms::{Room, RoomEvent, RoomTicket},
    ticket::LiveTicket,
};
// use moq_media::publish::LocalBroadcast;
use iroh_gossip::ALPN as GOSSIP_ALPN;
use iroh_live::media::publish::LocalBroadcast;

pub async fn publish_audio() -> anyhow::Result<()> {
    let live = Live::from_env()
        .await
        .expect("failed to get live from env")
        .with_router()
        .spawn();
    let broadcast = LocalBroadcast::new();
    // let audio = broadcast.audio();
    // let source = AudioSourc{format: };
    // audio.set(source, AudioCodec::Opus, presets);
    let audio = AudioBackend::default();
    let mic = audio
        .default_input()
        .await
        .expect("failed to get audio mic");
    broadcast
        .audio()
        .set(mic, AudioCodec::Opus, [AudioPreset::Hq])
        .expect("failed to broad cast audio");
    live.publish("hello", &broadcast).await?;
    let ticket = LiveTicket::new(live.endpoint().addr(), "hello");
    Ok(())
}

pub async fn subscribe(ticket: LiveTicket, endpoint: Endpoint) -> anyhow::Result<()> {
    // let live = Live::from_env().await?.spawn();
    let live = Live::new(endpoint);
    // let sub = live
    //     .subscribe(ticket.endpoint, &ticket.broadcast_name)
    //     .await?;

    let audio = AudioBackend::default();
    // let tracks = sub.media(&audio, Default::default()).await?;
    // if let Some(mut audio_test) = tracks.audio {
    //     // let a = audio_test.
    //     println!("");
    // }
    // if let Some(mut video) = tracks.video {
    //     while let Some(frame) = video.next_frame().await {
    //         println!("frame {}x{}", frame.dimensions[0], frame.dimensions[1]);
    //     }
    // }
    let playback_config = PlaybackConfig {
        backend: DecoderBackend::Auto,
        ..Default::default()
    };
    // let tik = ticket.clone();

    let (session, track) = live
        .subscribe_media(
            ticket.endpoint,
            &ticket.broadcast_name,
            &audio,
            playback_config,
        )
        .await?;
    let a = track.audio;
    Ok(())
}

// pub async fn create_room(endpoint: Endpoint) -> anyhow::Result<()> {

pub async fn create_room() -> anyhow::Result<()> {
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
    tokio::task::spawn(async move {
        println!("CREATE: room task started");
        // keep every subscribed track alive here — this map's lifetime
        // IS the playback lifetime. If a track isn't in here, it's dead.
        let (mut events, handle) = room.split();
        handle
            .publish("hello", &local_broadcast)
            .await
            .expect("failed to public to room");
        println!("created room ticket: {:?}", handle.ticket().to_string());
        // let _handle = handle;
        let mut active_audio: HashMap<_, AudioTrack> = HashMap::new();
        while let Some(event) = events.recv().await {
            match event {
                RoomEvent::BroadcastSubscribed { session, broadcast } => {
                    match broadcast.audio(&audio).await {
                        Ok(audio_track) => {
                            println!("got audio track, stopped={}", audio_track.is_stopped());
                            audio_track.set_volume(1.0);
                            active_audio.insert(session.remote_id(), audio_track);
                        }
                        Err(e) => {
                            println!("failed to get audio track: {e:?}");
                        }
                    }
                }
                RoomEvent::PeerJoined {
                    remote,
                    display_name,
                } => {
                    println!("PeerJoined");
                }
                RoomEvent::PeerLeft { remote } => {
                    println!("PeerLeft");
                    // drop their track explicitly when they leave
                    active_audio.remove(&remote);
                }
                RoomEvent::ChatReceived { remote, message } => {
                    println!("ChatReceived");
                }
                RoomEvent::RemoteAnnounced { remote, broadcasts } => {
                    println!("RemoteAnnounced");
                }
                _ => {
                    println!("Other RoomEvent???");
                }
            }
        }
        println!("CREATE: room task ended — recv() loop exited");
    });
    Ok(())
}

// pub async fn join_room(endpoint: Endpoint, ticket: RoomTicket) -> anyhow::Result<()> {
pub async fn join_room(ticket: RoomTicket) -> anyhow::Result<()> {
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
    tokio::task::spawn(async move {
        println!("JOIN: room task started");
        // keep every subscribed track alive here — this map's lifetime
        // IS the playback lifetime. If a track isn't in here, it's dead.
        let (mut events, handle) = room.split();
        handle
            .publish("hello", &local_broadcast)
            .await
            .expect("failed to public to room");
        // let _handle = handle;
        let mut active_audio: HashMap<_, AudioTrack> = HashMap::new();
        while let Some(event) = events.recv().await {
            match event {
                RoomEvent::BroadcastSubscribed { session, broadcast } => {
                    match broadcast.audio(&audio).await {
                        Ok(audio_track) => {
                            println!("got audio track, stopped={}", audio_track.is_stopped());
                            audio_track.set_volume(1.0);
                            active_audio.insert(session.remote_id(), audio_track);
                        }
                        Err(e) => {
                            println!("failed to get audio track: {e:?}");
                        }
                    }
                }
                RoomEvent::PeerJoined {
                    remote,
                    display_name,
                } => {
                    println!("PeerJoined");
                }
                RoomEvent::PeerLeft { remote } => {
                    println!("PeerLeft");
                    // drop their track explicitly when they leave
                    active_audio.remove(&remote);
                }
                RoomEvent::ChatReceived { remote, message } => {
                    println!("ChatReceived");
                }
                RoomEvent::RemoteAnnounced { remote, broadcasts } => {
                    println!("RemoteAnnounced");
                }
                _ => {
                    println!("Other RoomEvent???");
                }
            }
        }
        println!("JOIN: room task ended — recv() loop exited");
    });
    Ok(())
}
