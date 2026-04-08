use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::playlist_dto::{
    AddTracksDto, AudioMetadataDto, CreatePlaylistDto, PlaylistDto, PlaylistItemDto,
    PlaylistQueryDto, PlaylistShareInfoDto, ReorderTracksDto, SharePlaylistDto, UpdatePlaylistDto,
};
use crate::application::ports::music_ports::{MusicStoragePort, MusicUseCase};
use crate::common::errors::{DomainError, ErrorKind};
use crate::infrastructure::adapters::music_storage_adapter::MusicStorageAdapter;

pub struct MusicService {
    storage: Arc<MusicStorageAdapter>,
}

impl MusicService {
    pub fn new(storage: Arc<MusicStorageAdapter>) -> Self {
        Self { storage }
    }
}

impl MusicUseCase for MusicService {
    async fn create_playlist(
        &self,
        dto: CreatePlaylistDto,
        user_id: Uuid,
    ) -> Result<PlaylistDto, DomainError> {
        self.storage.create_playlist(dto, user_id).await
    }

    async fn update_playlist(
        &self,
        playlist_id: &str,
        dto: UpdatePlaylistDto,
        user_id: Uuid,
    ) -> Result<PlaylistDto, DomainError> {
        let has_access = self.storage.user_has_access(playlist_id, user_id).await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You don't have permission to update this playlist",
            ));
        }
        let can_write = self.storage.user_can_write(playlist_id, user_id).await?;
        if !can_write {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You need write access to update this playlist",
            ));
        }
        self.storage.update_playlist(playlist_id, dto).await
    }

    async fn delete_playlist(&self, playlist_id: &str, user_id: Uuid) -> Result<(), DomainError> {
        let playlist = self.storage.get_playlist(playlist_id).await?;
        let playlist = match playlist {
            Some(p) => p,
            None => {
                return Err(DomainError::new(
                    ErrorKind::NotFound,
                    "Playlist",
                    "Playlist not found",
                ));
            }
        };
        if playlist.owner_id != user_id.to_string() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "Only the owner can delete this playlist",
            ));
        }
        self.storage.delete_playlist(playlist_id).await
    }

    async fn get_playlist(
        &self,
        playlist_id: &str,
        user_id: Uuid,
    ) -> Result<PlaylistDto, DomainError> {
        let has_access = self.storage.user_has_access(playlist_id, user_id).await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You don't have permission to view this playlist",
            ));
        }
        let playlist = self.storage.get_playlist(playlist_id).await?;
        match playlist {
            Some(p) => Ok(p),
            None => Err(DomainError::new(
                ErrorKind::NotFound,
                "Playlist",
                "Playlist not found",
            )),
        }
    }

    async fn list_playlists(
        &self,
        query: PlaylistQueryDto,
        user_id: Uuid,
    ) -> Result<Vec<PlaylistDto>, DomainError> {
        let include_shared = query.include_shared.unwrap_or(true);
        let include_public = query.include_public.unwrap_or(false);
        let limit = query.limit.unwrap_or(100);
        let offset = query.offset.unwrap_or(0);

        let mut playlists = Vec::new();

        let owned = self.storage.list_playlists_by_owner(user_id).await?;
        playlists.extend(owned);

        if include_shared {
            let shared = self.storage.list_shared_with_user(user_id).await?;
            for s in shared {
                if !playlists.iter().any(|p: &PlaylistDto| p.id == s.id) {
                    playlists.push(s);
                }
            }
        }

        if include_public {
            let public = self.storage.list_public_playlists(limit, offset).await?;
            for p in public {
                if !playlists.iter().any(|pl: &PlaylistDto| pl.id == p.id) {
                    playlists.push(p);
                }
            }
        }

        Ok(playlists)
    }

    async fn add_tracks(
        &self,
        playlist_id: &str,
        dto: AddTracksDto,
        user_id: Uuid,
    ) -> Result<Vec<PlaylistItemDto>, DomainError> {
        let playlist_uuid = Uuid::parse_str(playlist_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid playlist ID")
        })?;

        let has_access = self.storage.user_has_access(playlist_id, user_id).await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You don't have permission to modify this playlist",
            ));
        }
        let can_write = self.storage.user_can_write(playlist_id, user_id).await?;
        if !can_write {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You need write access to add tracks",
            ));
        }

        let file_ids: Result<Vec<Uuid>, _> =
            dto.file_ids.iter().map(|id| Uuid::parse_str(id)).collect();
        let file_ids = file_ids.map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid file ID")
        })?;

        self.storage.add_tracks(&playlist_uuid, &file_ids).await
    }

    async fn remove_track(
        &self,
        playlist_id: &str,
        file_id: &str,
        user_id: Uuid,
    ) -> Result<(), DomainError> {
        let playlist_uuid = Uuid::parse_str(playlist_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid playlist ID")
        })?;
        let file_uuid = Uuid::parse_str(file_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid file ID")
        })?;

        let has_access = self.storage.user_has_access(playlist_id, user_id).await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You don't have permission to modify this playlist",
            ));
        }
        let can_write = self.storage.user_can_write(playlist_id, user_id).await?;
        if !can_write {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You need write access to remove tracks",
            ));
        }

        self.storage.remove_track(&playlist_uuid, &file_uuid).await
    }

    async fn reorder_tracks(
        &self,
        playlist_id: &str,
        dto: ReorderTracksDto,
        user_id: Uuid,
    ) -> Result<(), DomainError> {
        let playlist_uuid = Uuid::parse_str(playlist_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid playlist ID")
        })?;

        let has_access = self.storage.user_has_access(playlist_id, user_id).await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You don't have permission to modify this playlist",
            ));
        }
        let can_write = self.storage.user_can_write(playlist_id, user_id).await?;
        if !can_write {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You need write access to reorder tracks",
            ));
        }

        let item_ids: Result<Vec<Uuid>, _> =
            dto.item_ids.iter().map(|id| Uuid::parse_str(id)).collect();
        let item_ids = item_ids.map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid item ID")
        })?;

        self.storage.reorder_tracks(&playlist_uuid, &item_ids).await
    }

    async fn list_playlist_tracks(
        &self,
        playlist_id: &str,
        user_id: Uuid,
    ) -> Result<Vec<PlaylistItemDto>, DomainError> {
        let playlist_uuid = Uuid::parse_str(playlist_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid playlist ID")
        })?;

        let has_access = self.storage.user_has_access(playlist_id, user_id).await?;
        if !has_access {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "You don't have permission to view this playlist",
            ));
        }

        self.storage.list_playlist_tracks(&playlist_uuid).await
    }

    async fn share_playlist(
        &self,
        playlist_id: &str,
        dto: SharePlaylistDto,
        caller_id: Uuid,
    ) -> Result<(), DomainError> {
        let playlist = self.storage.get_playlist(playlist_id).await?;
        let playlist = match playlist {
            Some(p) => p,
            None => {
                return Err(DomainError::new(
                    ErrorKind::NotFound,
                    "Playlist",
                    "Playlist not found",
                ));
            }
        };
        if playlist.owner_id != caller_id.to_string() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "Only the owner can share this playlist",
            ));
        }

        let playlist_uuid = Uuid::parse_str(playlist_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid playlist ID")
        })?;
        let target_user_id = Uuid::parse_str(&dto.user_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid user ID")
        })?;
        let can_write = dto.can_write.unwrap_or(false);

        self.storage
            .share_playlist(&playlist_uuid, target_user_id, can_write)
            .await
    }

    async fn remove_share(
        &self,
        playlist_id: &str,
        target_user_id: &str,
        caller_id: Uuid,
    ) -> Result<(), DomainError> {
        let playlist = self.storage.get_playlist(playlist_id).await?;
        let playlist = match playlist {
            Some(p) => p,
            None => {
                return Err(DomainError::new(
                    ErrorKind::NotFound,
                    "Playlist",
                    "Playlist not found",
                ));
            }
        };
        if playlist.owner_id != caller_id.to_string() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "Only the owner can manage sharing",
            ));
        }

        let playlist_uuid = Uuid::parse_str(playlist_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid playlist ID")
        })?;
        let target_uuid = Uuid::parse_str(target_user_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid user ID")
        })?;

        self.storage.remove_share(&playlist_uuid, target_uuid).await
    }

    async fn get_playlist_shares(
        &self,
        playlist_id: &str,
        user_id: Uuid,
    ) -> Result<Vec<PlaylistShareInfoDto>, DomainError> {
        let playlist = self.storage.get_playlist(playlist_id).await?;
        let playlist = match playlist {
            Some(p) => p,
            None => {
                return Err(DomainError::new(
                    ErrorKind::NotFound,
                    "Playlist",
                    "Playlist not found",
                ));
            }
        };
        if playlist.owner_id != user_id.to_string() {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "Playlist",
                "Only the owner can view sharing info",
            ));
        }

        let playlist_uuid = Uuid::parse_str(playlist_id).map_err(|_| {
            DomainError::new(ErrorKind::InvalidInput, "Playlist", "Invalid playlist ID")
        })?;

        let shares = self.storage.get_shares(&playlist_uuid).await?;
        Ok(shares
            .into_iter()
            .map(|(uid, can_write)| PlaylistShareInfoDto {
                user_id: uid.to_string(),
                can_write,
            })
            .collect())
    }

    async fn get_audio_metadata(
        &self,
        file_id: &str,
        _user_id: Uuid,
    ) -> Result<Option<AudioMetadataDto>, DomainError> {
        let file_uuid = Uuid::parse_str(file_id)
            .map_err(|_| DomainError::new(ErrorKind::InvalidInput, "Music", "Invalid file ID"))?;
        self.storage.get_audio_metadata(&file_uuid).await
    }
}
