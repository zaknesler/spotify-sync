use super::playlist::PlaylistType;

pub struct Watcher {
    pub user_id: String,
    pub user_token: rspotify::Token,
    pub from_playlist: PlaylistType,
    pub to_playlist: PlaylistType,
    pub should_remove: bool,
}

impl Watcher {
    pub fn try_from_row_data(
        user_id: String,
        user_token: String,
        from_playlist: String,
        to_playlist: String,
        should_remove: bool,
    ) -> crate::Result<Self> {
        Ok(Self {
            user_id,
            user_token: serde_json::from_str(&user_token)?,
            from_playlist: from_playlist.try_into()?,
            to_playlist: to_playlist.try_into()?,
            should_remove,
        })
    }
}
