use crate::types::{SessionMap, UUID};

pub async fn send_emote(
    sessions: SessionMap,
    uuid: UUID,
    emote: crate::types::EmoteRequest,
) -> Result<(), String> {
    let emote_event = crate::types::EventEmote {
        from: uuid.lock().await.clone(),
        emote: emote.emote,
    };

    for target in emote.targets {
        if let Some(sessions) = sessions.lock().await.get_mut(&target) {
            let emote_event = emote_event.clone();
            let mut bad_sessions = Vec::new();

            for s in sessions.iter() {
                if s.notify::<crate::methods::EmoteEvent>(emote_event.clone())
                    .await
                    .is_err()
                {
                    bad_sessions.push(s.clone());
                }
            }

            for s in bad_sessions {
                sessions.remove(&s);
            }
        }
    }

    Ok(())
}
