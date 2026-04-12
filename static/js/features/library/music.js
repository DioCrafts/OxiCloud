/**
 * OxiCloud - Music Library View
 * Playlist management with track listings and audio player
 */

const musicView = {
    playlists: [],
    currentPlaylist: null,
    currentTracks: [],
    loading: false,
    _container: null,
    _initialized: false,
    selected: new Set(),

    _headers(json = false) {
        const h = typeof getCsrfHeaders === 'function' ? { ...getCsrfHeaders() } : {};
        if (json) h['Content-Type'] = 'application/json';
        return h;
    },

    init() {
        if (!this._container) {
            const contentArea = document.querySelector('.content-area');
            if (!contentArea) return;
            const el = document.createElement('div');
            el.id = 'music-container';
            el.className = 'music-container';
            contentArea.appendChild(el);
            this._container = el;
        }
        musicPlayer.init();
        this._initialized = true;
    },

    show() {
        this.init();
        if (!this._container) return;
        this._container.classList.add('active');
        this.currentPlaylist = null;
        this.currentTracks = [];
        this._container.innerHTML = '';
        this._renderPlaylists();
        this._loadPlaylists();
    },

    hide() {
        if (this._container) {
            this._container.classList.remove('active');
        }
        this.currentPlaylist = null;
        this.currentTracks = [];
    },

    async _loadPlaylists() {
        if (this.loading) return;
        this.loading = true;
        this._showLoading(true);

        try {
            const resp = await fetch('/api/playlists', {
                credentials: 'include',
                headers: this._headers()
            });

            if (!resp.ok) throw new Error('Failed to load playlists');

            this.playlists = await resp.json();
            this._renderPlaylists();
        } catch (err) {
            console.error('Music load error:', err);
            this._showError(err.message);
        } finally {
            this.loading = false;
            this._showLoading(false);
        }
    },

    _renderPlaylists() {
        if (!this._container) return;

        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        // Empty state: no playlists at all — show full-width centered onboarding
        if (this.playlists.length === 0) {
            this._container.innerHTML = `
                <div class="music-empty-state">
                    <div class="music-empty-state-icon">
                        <i class="fas fa-music"></i>
                    </div>
                    <h3 class="music-empty-state-title">${t('music.no_playlists', 'No playlists yet')}</h3>
                    <p class="music-empty-state-desc">${t('music.empty_hint', 'Create your first playlist to start organizing your music')}</p>
                    <button class="btn btn-primary" id="music-create-playlist-btn">
                        <i class="fas fa-plus"></i>
                        <span>${t('music.create_playlist', 'Create Playlist')}</span>
                    </button>
                </div>
            `;
            const createBtn = document.getElementById('music-create-playlist-btn');
            if (createBtn) {
                createBtn.addEventListener('click', () => this._showCreatePlaylistDialog());
            }
            return;
        }

        // Normal layout: sidebar + main
        this._container.innerHTML = `
            <div class="music-content">
                <div class="music-sidebar">
                    <div class="music-sidebar-header">
                        <h3>${t('music.playlists', 'Playlists')}</h3>
                        <button class="music-sidebar-add-btn" id="music-create-playlist-btn" title="${t('music.create_playlist', 'Create Playlist')}">
                            <i class="fas fa-plus"></i>
                        </button>
                    </div>
                    <div class="music-playlist-list" id="music-playlist-list"></div>
                </div>
                <div class="music-main">
                    <div class="music-welcome">
                        <i class="fas fa-music"></i>
                        <h3>${t('music.select_playlist', 'Select a playlist')}</h3>
                        <p>${t('music.select_hint', 'Choose a playlist from the sidebar or create a new one')}</p>
                    </div>
                    <div class="music-playlist-detail hidden" id="music-playlist-detail">
                        <div class="music-playlist-header">
                            <div class="music-playlist-cover" id="music-playlist-cover" title="${t('music.set_cover', 'Set cover')}">
                                <i class="fas fa-music"></i>
                            </div>
                            <div class="music-playlist-info">
                                <h2 id="music-playlist-name"></h2>
                                <p id="music-playlist-meta"></p>
                                <span class="music-public-badge hidden" id="music-public-badge">
                                    <i class="fas fa-globe"></i> <span id="music-public-text">${t('music.public', 'Public')}</span>
                                </span>
                            </div>
                        </div>
                        <div class="music-playlist-actions">
                            <button class="btn btn-secondary" id="music-play-all-btn">
                                <i class="fas fa-play"></i>
                                <span>${t('music.play_all', 'Play All')}</span>
                            </button>
                            <button class="btn btn-secondary" id="music-shuffle-btn">
                                <i class="fas fa-shuffle"></i>
                            </button>
                            <button class="btn btn-secondary" id="music-add-tracks-btn">
                                <i class="fas fa-plus"></i>
                                <span>${t('music.add_tracks', 'Add Tracks')}</span>
                            </button>
                            <button class="btn btn-secondary" id="music-edit-playlist-btn" title="${t('music.edit', 'Edit')}">
                                <i class="fas fa-pen"></i>
                            </button>
                            <button class="btn btn-secondary" id="music-share-playlist-btn" title="${t('music.share', 'Share')}">
                                <i class="fas fa-share-alt"></i>
                            </button>
                            <button class="btn btn-secondary" id="music-manage-shares-btn" title="${t('music.manage_shares', 'Manage Shares')}">
                                <i class="fas fa-users"></i>
                            </button>
                            <button class="btn btn-secondary" id="music-toggle-public-btn" title="${t('music.toggle_public', 'Toggle public')}">
                                <i class="fas fa-globe"></i>
                            </button>
                            <button class="btn btn-secondary" id="music-delete-playlist-btn">
                                <i class="fas fa-trash"></i>
                            </button>
                        </div>
                        <div class="music-track-list" id="music-track-list"></div>
                    </div>
                </div>
            </div>
        `;

        this._bindPlaylistEvents();
        this._renderPlaylistList();
    },

    _renderPlaylistList() {
        const listEl = document.getElementById('music-playlist-list');
        if (!listEl) return;

        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (this.playlists.length === 0) {
            listEl.innerHTML = `
                <div class="music-empty">
                    <i class="fas fa-music"></i>
                    <p>${t('music.no_playlists', 'No playlists yet')}</p>
                </div>
            `;
            return;
        }

        listEl.innerHTML = this.playlists
            .map(
                (p) => `
            <div class="music-playlist-item" data-id="${p.id}">
                <div class="music-playlist-icon">
                    <i class="fas fa-music"></i>
                </div>
                <div class="music-playlist-item-info">
                    <span class="music-playlist-item-name">${this._escapeHtml(p.name)}</span>
                    <span class="music-playlist-item-count">${p.track_count || 0} ${t('music.tracks', 'tracks')}</span>
                </div>
            </div>
        `
            )
            .join('');

        listEl.querySelectorAll('.music-playlist-item').forEach((item) => {
            item.addEventListener('click', () => {
                const id = item.dataset.id;
                this._selectPlaylist(id);
            });
        });
    },

    _bindPlaylistEvents() {
        const createBtn = document.getElementById('music-create-playlist-btn');
        if (createBtn) {
            createBtn.addEventListener('click', () => this._showCreatePlaylistDialog());
        }

        const playAllBtn = document.getElementById('music-play-all-btn');
        if (playAllBtn) {
            playAllBtn.addEventListener('click', () => this._playAll());
        }

        const shuffleBtn = document.getElementById('music-shuffle-btn');
        if (shuffleBtn) {
            shuffleBtn.addEventListener('click', () => this._shufflePlay());
        }

        const deleteBtn = document.getElementById('music-delete-playlist-btn');
        if (deleteBtn) {
            deleteBtn.addEventListener('click', () => this._deletePlaylist());
        }

        const editBtn = document.getElementById('music-edit-playlist-btn');
        if (editBtn) {
            editBtn.addEventListener('click', () => this._showEditPlaylistDialog());
        }

        const shareBtn = document.getElementById('music-share-playlist-btn');
        if (shareBtn) {
            shareBtn.addEventListener('click', () => this._showSharePlaylistDialog());
        }

        const addTracksBtn = document.getElementById('music-add-tracks-btn');
        if (addTracksBtn) {
            addTracksBtn.addEventListener('click', () => this._showAddTracksDialog());
        }

        const manageSharesBtn = document.getElementById('music-manage-shares-btn');
        if (manageSharesBtn) {
            manageSharesBtn.addEventListener('click', () => this._showManageSharesDialog());
        }

        const togglePublicBtn = document.getElementById('music-toggle-public-btn');
        if (togglePublicBtn) {
            togglePublicBtn.addEventListener('click', () => this._togglePublic());
        }

        const coverEl = document.getElementById('music-playlist-cover');
        if (coverEl) {
            coverEl.addEventListener('click', () => this._showCoverPicker());
        }
    },

    async _selectPlaylist(playlistId) {
        const playlist = this.playlists.find((p) => p.id === playlistId);
        if (!playlist) return;

        this.currentPlaylist = playlist;

        const detailEl = document.getElementById('music-playlist-detail');
        const welcomeEl = this._container.querySelector('.music-welcome');
        const nameEl = document.getElementById('music-playlist-name');
        const metaEl = document.getElementById('music-playlist-meta');

        if (welcomeEl) welcomeEl.classList.add('hidden');
        if (detailEl) detailEl.classList.remove('hidden');
        if (nameEl) nameEl.textContent = playlist.name;
        if (metaEl) {
            const t = (key, fallback = '') => {
                return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
            };
            metaEl.textContent = `${playlist.track_count || 0} ${t('music.tracks', 'tracks')}`;
        }

        // Cover art
        const coverEl = document.getElementById('music-playlist-cover');
        if (coverEl) {
            if (playlist.cover_file_id) {
                coverEl.innerHTML = `<img src="/api/files/${encodeURIComponent(playlist.cover_file_id)}" alt="" class="music-cover-img"><div class="music-cover-overlay"><i class="fas fa-camera"></i></div>`;
            } else {
                coverEl.innerHTML = `<i class="fas fa-music"></i><div class="music-cover-overlay"><i class="fas fa-camera"></i></div>`;
            }
        }

        // Public badge
        const publicBadge = document.getElementById('music-public-badge');
        if (publicBadge) {
            publicBadge.classList.toggle('hidden', !playlist.is_public);
        }
        const togglePublicBtn = document.getElementById('music-toggle-public-btn');
        if (togglePublicBtn) {
            const t2 = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);
            togglePublicBtn.title = playlist.is_public ? t2('music.make_private', 'Make private') : t2('music.make_public', 'Make public');
            togglePublicBtn.classList.toggle('active', playlist.is_public);
        }

        document.querySelectorAll('.music-playlist-item').forEach((item) => {
            item.classList.toggle('active', item.dataset.id === playlistId);
        });

        await this._loadPlaylistTracks(playlistId);
    },

    async _loadPlaylistTracks(playlistId) {
        const trackListEl = document.getElementById('music-track-list');
        if (!trackListEl) return;

        trackListEl.innerHTML = '<div class="music-loading"><i class="fas fa-spinner fa-spin"></i></div>';

        try {
            const resp = await fetch(`/api/playlists/${playlistId}/tracks`, {
                credentials: 'include',
                headers: this._headers()
            });

            if (!resp.ok) throw new Error('Failed to load tracks');

            this.currentTracks = await resp.json();
            this._renderTracks();
        } catch (err) {
            console.error('Track load error:', err);
            trackListEl.innerHTML = `<div class="music-error">${err.message}</div>`;
        }
    },

    _renderTracks() {
        const trackListEl = document.getElementById('music-track-list');
        if (!trackListEl) return;

        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (this.currentTracks.length === 0) {
            trackListEl.innerHTML = `
                <div class="music-empty">
                    <i class="fas fa-music"></i>
                    <p>${t('music.no_tracks', 'No tracks in this playlist')}</p>
                </div>
            `;
            return;
        }

        trackListEl.innerHTML = `
            <div class="music-track-header">
                <span class="music-track-col music-track-drag"></span>
                <span class="music-track-col music-track-num">#</span>
                <span class="music-track-col music-track-title">${t('music.title', 'Title')}</span>
                <span class="music-track-col music-track-artist">${t('music.artist', 'Artist')}</span>
                <span class="music-track-col music-track-album">${t('music.album', 'Album')}</span>
                <span class="music-track-col music-track-duration"><i class="far fa-clock"></i></span>
                <span class="music-track-col music-track-actions"></span>
            </div>
            ${this.currentTracks
                .map(
                    (track, idx) => `
                <div class="music-track ${musicPlayer.currentTrack?.id === track.id ? 'playing' : ''}" data-idx="${idx}" data-id="${track.id}" data-file-id="${track.file_id}" draggable="true">
                    <span class="music-track-col music-track-drag"><i class="fas fa-grip-vertical"></i></span>
                    <span class="music-track-col music-track-num">
                        <span class="track-num-text">${idx + 1}</span>
                        <i class="fas fa-play track-play-icon hidden"></i>
                    </span>
                    <span class="music-track-col music-track-title">
                        <i class="fas fa-music music-track-icon"></i>
                        <span class="music-track-name">${this._escapeHtml(track.title || track.file_name || t('music.unknown_title', 'Unknown'))}</span>
                    </span>
                    <span class="music-track-col music-track-artist">${this._escapeHtml(track.artist || t('music.unknown_artist', 'Unknown Artist'))}</span>
                    <span class="music-track-col music-track-album">${this._escapeHtml(track.album || '-')}</span>
                    <span class="music-track-col music-track-duration">${this._formatDuration(track.duration_secs)}</span>
                    <span class="music-track-col music-track-actions">
                        <button class="music-track-remove-btn" title="${t('music.remove', 'Remove')}"><i class="fas fa-times"></i></button>
                    </span>
                </div>
            `
                )
                .join('')}
        `;
        trackListEl.querySelectorAll('.music-track').forEach((row) => {
            row.addEventListener('click', () => {
                const idx = parseInt(row.dataset.idx, 10);
                // Toggle selection
                trackListEl.querySelectorAll('.music-track').forEach((r) => {
                    if (r !== row) r.classList.remove('selected');
                });
                row.classList.toggle('selected');
                this.selected.clear();
                if (row.classList.contains('selected')) {
                    this.selected.add(idx);
                }
            });

            row.addEventListener('dblclick', (e) => {
                e.preventDefault();
                const idx = parseInt(row.dataset.idx, 10);
                this._playTrack(idx);
            });

            // Drag & drop
            row.addEventListener('dragstart', (e) => {
                row.classList.add('dragging');
                e.dataTransfer.effectAllowed = 'move';
                e.dataTransfer.setData('text/plain', row.dataset.idx);
            });
            row.addEventListener('dragend', () => {
                row.classList.remove('dragging');
                trackListEl.querySelectorAll('.music-track').forEach((r) => {
                    r.classList.remove('drag-over');
                });
            });
            row.addEventListener('dragover', (e) => {
                e.preventDefault();
                e.dataTransfer.dropEffect = 'move';
                const dragging = trackListEl.querySelector('.dragging');
                if (dragging && dragging !== row) {
                    row.classList.add('drag-over');
                }
            });
            row.addEventListener('dragleave', () => {
                row.classList.remove('drag-over');
            });
            row.addEventListener('drop', (e) => {
                e.preventDefault();
                row.classList.remove('drag-over');
                const fromIdx = parseInt(e.dataTransfer.getData('text/plain'), 10);
                const toIdx = parseInt(row.dataset.idx, 10);
                if (fromIdx !== toIdx) {
                    this._reorderTrack(fromIdx, toIdx);
                }
            });

            // Remove track button
            const removeBtn = row.querySelector('.music-track-remove-btn');
            if (removeBtn) {
                removeBtn.addEventListener('click', (e) => {
                    e.stopPropagation();
                    this._removeTrackFromPlaylist(row.dataset.id, row.dataset.fileId);
                });
            }
        });
    },

    _playTrack(idx) {
        if (!this.currentTracks[idx]) return;

        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };
        musicPlayer.setQueue(this.currentTracks, this.currentPlaylist?.name || t('music.playlists', 'Playlist'));
        musicPlayer.playTrack(idx);
    },

    _playAll() {
        if (this.currentTracks.length > 0) {
            this._playTrack(0);
        }
    },

    _shufflePlay() {
        if (this.currentTracks.length > 0) {
            const shuffled = [...this.currentTracks];
            for (let i = shuffled.length - 1; i > 0; i--) {
                const j = Math.floor(Math.random() * (i + 1));
                [shuffled[i], shuffled[j]] = [shuffled[j], shuffled[i]];
            }
            const t = (key, fallback = '') => {
                return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
            };
            musicPlayer.setQueue(shuffled, this.currentPlaylist?.name || t('music.shuffle', 'Shuffle'));
            musicPlayer.playTrack(0);
        }
    },

    async _showCreatePlaylistDialog() {
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (!window.Modal) return;
        const name = await window.Modal.prompt({
            title: t('music.create_playlist', 'Create Playlist'),
            label: t('music.playlist_name', 'Playlist name'),
            placeholder: t('music.playlist_name', 'Playlist name'),
            icon: 'fa-music',
            confirmText: t('music.create', 'Create')
        });
        if (!name?.trim()) return;

        this._createPlaylist(name.trim());
    },

    async _createPlaylist(name) {
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };
        const createBtn = document.getElementById('music-create-playlist-btn');
        if (createBtn) createBtn.disabled = true;
        try {
            const resp = await fetch('/api/playlists', {
                method: 'POST',
                credentials: 'include',
                headers: this._headers(true),
                body: JSON.stringify({ name, description: null })
            });

            if (!resp.ok) throw new Error('Failed to create playlist');

            const playlist = await resp.json();
            this.playlists.unshift(playlist);
            this._renderPlaylists();
            this._selectPlaylist(playlist.id);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-check-circle',
                    iconClass: 'upload',
                    title: t('music.create_playlist', 'Create Playlist'),
                    text: name
                });
            }
        } catch (err) {
            console.error('Create playlist error:', err);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
        } finally {
            if (createBtn) createBtn.disabled = false;
        }
    },

    async _deletePlaylist() {
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (!this.currentPlaylist) return;

        const confirmed = await new Promise((resolve) => {
            if (!window.Modal) {
                resolve(confirm(t('music.confirm_delete', 'Delete this playlist?')));
                return;
            }
            window.Modal.prompt({
                title: t('music.delete', 'Delete'),
                label: t('music.confirm_delete', 'Delete this playlist?'),
                placeholder: '',
                value: this.currentPlaylist.name,
                icon: 'fa-trash',
                confirmText: t('music.delete', 'Delete')
            }).then((val) => resolve(val !== null));
        });
        if (!confirmed) return;

        const deleteBtn = document.getElementById('music-delete-playlist-btn');
        if (deleteBtn) deleteBtn.disabled = true;
        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}`, {
                method: 'DELETE',
                credentials: 'include',
                headers: this._headers()
            });

            if (!resp.ok) throw new Error('Failed to delete playlist');

            const deletedName = this.currentPlaylist.name;
            this.playlists = this.playlists.filter((p) => p.id !== this.currentPlaylist.id);
            this.currentPlaylist = null;
            this.currentTracks = [];
            this._renderPlaylists();
            if (window.notifications) {
                window.notifications.addNotification({ icon: 'fa-check-circle', iconClass: 'upload', title: t('music.delete', 'Delete'), text: deletedName });
            }
        } catch (err) {
            console.error('Delete playlist error:', err);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
        } finally {
            if (deleteBtn) deleteBtn.disabled = false;
        }
    },

    _formatDuration(secs) {
        if (!secs) return '-';
        const mins = Math.floor(secs / 60);
        const s = Math.floor(secs % 60);
        return `${mins}:${s.toString().padStart(2, '0')}`;
    },

    _escapeHtml(str) {
        if (!str) return '';
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    },

    _showLoading(show) {
        const existing = this._container?.querySelector('.music-loading');
        if (show && !existing) {
            const loading = document.createElement('div');
            loading.className = 'music-loading';
            loading.innerHTML = '<i class="fas fa-spinner fa-spin"></i>';
            this._container?.appendChild(loading);
        } else if (!show && existing) {
            existing.remove();
        }
    },

    _showError(message) {
        if (!this._container) return;
        this._container.innerHTML = `
            <div class="music-error">
                <i class="fas fa-exclamation-circle"></i>
                <p>${this._escapeHtml(message)}</p>
            </div>
        `;
    },

    async _showEditPlaylistDialog() {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (!window.Modal) return;
        const newName = await window.Modal.prompt({
            title: t('music.edit', 'Edit'),
            label: t('music.playlist_name', 'Playlist name'),
            placeholder: t('music.playlist_name', 'Playlist name'),
            value: this.currentPlaylist.name,
            icon: 'fa-pen',
            confirmText: t('actions.confirm', 'Save')
        });
        if (!newName?.trim() || newName.trim() === this.currentPlaylist.name) return;

        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}`, {
                method: 'PUT',
                credentials: 'include',
                headers: this._headers(true),
                body: JSON.stringify({ name: newName.trim() })
            });

            if (!resp.ok) throw new Error('Failed to update playlist');

            this.currentPlaylist.name = newName.trim();
            const idx = this.playlists.findIndex((p) => p.id === this.currentPlaylist.id);
            if (idx !== -1) this.playlists[idx].name = newName.trim();

            const nameEl = document.getElementById('music-playlist-name');
            if (nameEl) nameEl.textContent = newName.trim();
            this._renderPlaylistList();
        } catch (err) {
            console.error('Edit playlist error:', err);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
        }
    },

    async _showSharePlaylistDialog() {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (!window.Modal) return;
        const userId = await window.Modal.prompt({
            title: t('music.share', 'Share'),
            label: t('music.share_with_user', 'User ID or email'),
            placeholder: t('music.share_with_user', 'User ID or email'),
            icon: 'fa-share-alt',
            confirmText: t('music.share', 'Share')
        });
        if (!userId?.trim()) return;

        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}/share`, {
                method: 'POST',
                credentials: 'include',
                headers: this._headers(true),
                body: JSON.stringify({ user_id: userId.trim(), can_write: false })
            });

            if (!resp.ok) throw new Error('Failed to share playlist');

            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-check-circle',
                    iconClass: 'upload',
                    title: t('music.share', 'Share'),
                    text: t('music.added', 'Added!')
                });
            }
        } catch (err) {
            console.error('Share playlist error:', err);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
        }
    },

    async _showAddTracksDialog() {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        // ── Build modal overlay ──
        const overlay = document.createElement('div');
        overlay.className = 'music-picker-overlay';
        overlay.innerHTML = `
            <div class="music-picker-modal">
                <div class="music-picker-header">
                    <h3><i class="fas fa-music"></i> ${t('music.add_tracks', 'Add Tracks')}</h3>
                    <button class="music-picker-close" title="${t('common.close', 'Close')}">&times;</button>
                </div>
                <div class="music-picker-search">
                    <i class="fas fa-search"></i>
                    <input type="text" id="music-picker-query"
                           placeholder="${t('music.search_audio', 'Search audio files…')}" autocomplete="off">
                </div>
                <div class="music-picker-list" id="music-picker-list">
                    <div class="music-picker-loading"><i class="fas fa-spinner fa-spin"></i> ${t('music.loading', 'Loading…')}</div>
                </div>
                <div class="music-picker-footer">
                    <span class="music-picker-selected-count" id="music-picker-count">0 ${t('music.selected', 'selected')}</span>
                    <div class="music-picker-actions">
                        <button class="btn btn-secondary music-picker-cancel">${t('common.cancel', 'Cancel')}</button>
                        <button class="btn btn-primary music-picker-add" id="music-picker-add-btn" disabled>
                            <i class="fas fa-plus"></i> ${t('music.add', 'Add')}
                        </button>
                    </div>
                </div>
            </div>
        `;
        document.body.appendChild(overlay);
        requestAnimationFrame(() => overlay.classList.add('active'));

        const listEl = document.getElementById('music-picker-list');
        const queryInput = document.getElementById('music-picker-query');
        const addBtn = document.getElementById('music-picker-add-btn');
        const countEl = document.getElementById('music-picker-count');
        const selectedIds = new Set();

        // ── Close helpers ──
        const close = () => {
            overlay.classList.remove('active');
            setTimeout(() => overlay.remove(), 200);
        };
        overlay.querySelector('.music-picker-close').addEventListener('click', close);
        overlay.querySelector('.music-picker-cancel').addEventListener('click', close);
        overlay.addEventListener('click', (e) => {
            if (e.target === overlay) close();
        });

        // ── Fetch & render audio files ──
        const AUDIO_EXTENSIONS = 'mp3,ogg,flac,wav,aac,m4a,wma,opus,webm';

        const fetchAudioFiles = async (query = '') => {
            listEl.innerHTML = `<div class="music-picker-loading"><i class="fas fa-spinner fa-spin"></i> ${t('music.loading', 'Loading…')}</div>`;
            try {
                const params = new URLSearchParams({ type_filter: AUDIO_EXTENSIONS, limit: '200', recursive: 'true' });
                if (query.trim()) params.set('query', query.trim());
                const resp = await fetch(`/api/search?${params}`, { credentials: 'include' });
                if (!resp.ok) throw new Error('Search failed');
                const data = await resp.json();
                renderFiles(data.files || []);
            } catch (err) {
                console.error('Audio search error:', err);
                listEl.innerHTML = `<div class="music-picker-empty"><i class="fas fa-exclamation-triangle"></i> ${t('music.search_error', 'Could not load audio files')}</div>`;
            }
        };

        const renderFiles = (files) => {
            if (files.length === 0) {
                listEl.innerHTML = `<div class="music-picker-empty"><i class="fas fa-folder-open"></i> ${t('music.no_audio_files', 'No audio files found')}</div>`;
                return;
            }
            listEl.innerHTML = '';
            for (const file of files) {
                const row = document.createElement('label');
                row.className = `music-picker-item${selectedIds.has(file.id) ? ' selected' : ''}`;
                const sizeStr = file.size != null && window.formatFileSize ? window.formatFileSize(file.size) : '';
                row.innerHTML = `
                    <input type="checkbox" value="${file.id}" ${selectedIds.has(file.id) ? 'checked' : ''}>
                    <i class="fas fa-file-audio"></i>
                    <span class="music-picker-name" title="${this._escapeHtml(file.name)}">${this._escapeHtml(file.name)}</span>
                    <span class="music-picker-size">${sizeStr}</span>
                `;
                const cb = row.querySelector('input');
                cb.addEventListener('change', () => {
                    if (cb.checked) {
                        selectedIds.add(file.id);
                        row.classList.add('selected');
                    } else {
                        selectedIds.delete(file.id);
                        row.classList.remove('selected');
                    }
                    countEl.textContent = `${selectedIds.size} ${t('music.selected', 'selected')}`;
                    addBtn.disabled = selectedIds.size === 0;
                });
                listEl.appendChild(row);
            }
        };

        // ── Debounced search ──
        let searchTimer = null;
        queryInput.addEventListener('input', () => {
            clearTimeout(searchTimer);
            searchTimer = setTimeout(() => fetchAudioFiles(queryInput.value), 300);
        });

        // ── Add button ──
        addBtn.addEventListener('click', async () => {
            if (selectedIds.size === 0) return;
            addBtn.disabled = true;
            addBtn.innerHTML = `<i class="fas fa-spinner fa-spin"></i> ${t('music.adding', 'Adding…')}`;

            try {
                const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}/tracks`, {
                    method: 'POST',
                    credentials: 'include',
                    headers: this._headers(true),
                    body: JSON.stringify({ file_ids: [...selectedIds] })
                });
                if (!resp.ok) throw new Error('Failed to add tracks');

                if (window.notifications) {
                    window.notifications.addNotification({
                        icon: 'fa-check-circle',
                        iconClass: 'upload',
                        title: t('music.add_tracks', 'Add Tracks'),
                        text: `${selectedIds.size} ${t('music.added_to_playlist', 'added to playlist')}`
                    });
                }
                close();
                await this._loadPlaylistTracks(this.currentPlaylist.id);
                const playlist = this.playlists.find((p) => p.id === this.currentPlaylist.id);
                if (playlist) {
                    playlist.track_count = (playlist.track_count || 0) + selectedIds.size;
                    this.currentPlaylist.track_count = playlist.track_count;
                    this._renderPlaylistList();
                    const metaEl = document.getElementById('music-playlist-meta');
                    if (metaEl) metaEl.textContent = `${playlist.track_count} ${t('music.tracks', 'tracks')}`;
                }
            } catch (err) {
                console.error('Add tracks error:', err);
                if (window.notifications) {
                    window.notifications.addNotification({
                        icon: 'fa-exclamation-circle',
                        iconClass: 'error',
                        title: t('music.error', 'Error'),
                        text: t('music.add_error', 'Could not add tracks to playlist')
                    });
                }
                addBtn.disabled = false;
                addBtn.innerHTML = `<i class="fas fa-plus"></i> ${t('music.add', 'Add')}`;
            }
        });

        // ── Initial load (all audio files) ──
        queryInput.focus();
        fetchAudioFiles();
    },

    async _removeTrackFromPlaylist(_trackId, fileId) {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);

        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}/tracks/${encodeURIComponent(fileId)}`, {
                method: 'DELETE',
                credentials: 'include',
                headers: this._headers()
            });
            if (!resp.ok) throw new Error('Failed to remove track');

            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-check-circle',
                    iconClass: 'upload',
                    title: t('music.remove', 'Remove'),
                    text: t('music.track_removed', 'Track removed')
                });
            }
            await this._loadPlaylistTracks(this.currentPlaylist.id);
            const playlist = this.playlists.find((p) => p.id === this.currentPlaylist.id);
            if (playlist) {
                playlist.track_count = Math.max(0, (playlist.track_count || 1) - 1);
                this.currentPlaylist.track_count = playlist.track_count;
                this._renderPlaylistList();
                const metaEl = document.getElementById('music-playlist-meta');
                if (metaEl) metaEl.textContent = `${playlist.track_count} ${t('music.tracks', 'tracks')}`;
            }
        } catch (err) {
            console.error('Remove track error:', err);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
        }
    },

    async _reorderTrack(fromIdx, toIdx) {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);

        const tracks = [...this.currentTracks];
        const [moved] = tracks.splice(fromIdx, 1);
        tracks.splice(toIdx, 0, moved);
        this.currentTracks = tracks;
        this._renderTracks();

        const itemIds = tracks.map((tr) => tr.id);
        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}/reorder`, {
                method: 'PUT',
                credentials: 'include',
                headers: this._headers(true),
                body: JSON.stringify({ item_ids: itemIds })
            });
            if (!resp.ok) throw new Error('Failed to reorder tracks');
        } catch (err) {
            console.error('Reorder error:', err);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
            await this._loadPlaylistTracks(this.currentPlaylist.id);
        }
    },

    async _showManageSharesDialog() {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);

        const existing = document.getElementById('music-shares-dialog');
        if (existing) existing.remove();

        const dialog = document.createElement('div');
        dialog.id = 'music-shares-dialog';
        dialog.className = 'music-shares-overlay';
        dialog.innerHTML = `
            <div class="music-shares-panel">
                <div class="music-shares-header">
                    <h3><i class="fas fa-users"></i> ${t('music.manage_shares', 'Manage Shares')}</h3>
                    <button class="music-shares-close-btn"><i class="fas fa-times"></i></button>
                </div>
                <div class="music-shares-body">
                    <div class="music-shares-loading"><i class="fas fa-spinner fa-spin"></i></div>
                </div>
                <div class="music-shares-add">
                    <input type="text" id="music-share-user-input" placeholder="${t('music.share_with_user', 'User ID or email')}" class="music-shares-input">
                    <label class="music-shares-write-label">
                        <input type="checkbox" id="music-share-write-input"> ${t('music.can_write', 'Can edit')}
                    </label>
                    <button class="btn btn-primary btn-sm" id="music-share-add-btn">
                        <i class="fas fa-plus"></i> ${t('music.share', 'Share')}
                    </button>
                </div>
            </div>
        `;
        document.body.appendChild(dialog);

        dialog.querySelector('.music-shares-close-btn').addEventListener('click', () => dialog.remove());
        dialog.addEventListener('click', (e) => {
            if (e.target === dialog) dialog.remove();
        });

        dialog.querySelector('#music-share-add-btn').addEventListener('click', async () => {
            const userInput = dialog.querySelector('#music-share-user-input');
            const writeInput = dialog.querySelector('#music-share-write-input');
            const userId = userInput.value.trim();
            if (!userId) return;

            try {
                const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}/share`, {
                    method: 'POST',
                    credentials: 'include',
                    headers: this._headers(true),
                    body: JSON.stringify({ user_id: userId, can_write: writeInput.checked })
                });
                if (!resp.ok) throw new Error('Failed to share');
                userInput.value = '';
                writeInput.checked = false;
                this._loadSharesList(dialog);
                if (window.notifications) {
                    window.notifications.addNotification({
                        icon: 'fa-check-circle',
                        iconClass: 'upload',
                        title: t('music.share', 'Share'),
                        text: t('music.added', 'Added!')
                    });
                }
            } catch (err) {
                if (window.notifications) {
                    window.notifications.addNotification({
                        icon: 'fa-exclamation-circle',
                        iconClass: 'error',
                        title: t('music.error', 'Error'),
                        text: err.message
                    });
                }
            }
        });

        this._loadSharesList(dialog);
    },

    async _loadSharesList(dialog) {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);
        const body = dialog.querySelector('.music-shares-body');
        if (!body) return;

        body.innerHTML = '<div class="music-shares-loading"><i class="fas fa-spinner fa-spin"></i></div>';

        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}/shares`, {
                credentials: 'include',
                headers: this._headers()
            });
            if (!resp.ok) throw new Error('Failed to load shares');
            const shares = await resp.json();

            if (shares.length === 0) {
                body.innerHTML = `<p class="music-shares-empty">${t('music.no_shares', 'No shares yet')}</p>`;
                return;
            }

            body.innerHTML = shares
                .map(
                    (s) => `
                <div class="music-share-item" data-user-id="${this._escapeHtml(s.user_id)}">
                    <span class="music-share-user"><i class="fas fa-user"></i> ${this._escapeHtml(s.user_id)}</span>
                    <span class="music-share-perm">${s.can_write ? t('music.can_write', 'Can edit') : t('music.read_only', 'Read only')}</span>
                    <button class="music-share-remove-btn" title="${t('music.remove_share', 'Remove share')}"><i class="fas fa-times"></i></button>
                </div>
            `
                )
                .join('');

            body.querySelectorAll('.music-share-remove-btn').forEach((btn) => {
                btn.addEventListener('click', async () => {
                    const item = btn.closest('.music-share-item');
                    const userId = item.dataset.userId;
                    await this._removeShare(userId, dialog);
                });
            });
        } catch (err) {
            body.innerHTML = `<p class="music-shares-empty">${this._escapeHtml(err.message)}</p>`;
        }
    },

    async _removeShare(userId, dialog) {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);

        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}/share/${encodeURIComponent(userId)}`, {
                method: 'DELETE',
                credentials: 'include',
                headers: this._headers()
            });
            if (!resp.ok) throw new Error('Failed to remove share');
            this._loadSharesList(dialog);
        } catch (err) {
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
        }
    },

    async _togglePublic() {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);
        const newValue = !this.currentPlaylist.is_public;

        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}`, {
                method: 'PUT',
                credentials: 'include',
                headers: this._headers(true),
                body: JSON.stringify({ is_public: newValue })
            });
            if (!resp.ok) throw new Error('Failed to update playlist');

            this.currentPlaylist.is_public = newValue;
            const idx = this.playlists.findIndex((p) => p.id === this.currentPlaylist.id);
            if (idx !== -1) this.playlists[idx].is_public = newValue;

            const badge = document.getElementById('music-public-badge');
            if (badge) badge.classList.toggle('hidden', !newValue);

            const btn = document.getElementById('music-toggle-public-btn');
            if (btn) {
                btn.title = newValue ? t('music.make_private', 'Make private') : t('music.make_public', 'Make public');
                btn.classList.toggle('active', newValue);
            }

            if (window.notifications) {
                const status = newValue ? t('music.public', 'Public') : t('music.private', 'Private');
                window.notifications.addNotification({
                    icon: 'fa-check-circle',
                    iconClass: 'upload',
                    title: t('music.toggle_public', 'Visibility'),
                    text: status
                });
            }
        } catch (err) {
            console.error('Toggle public error:', err);
            if (window.notifications) {
                window.notifications.addNotification({
                    icon: 'fa-exclamation-circle',
                    iconClass: 'error',
                    title: t('music.error', 'Error'),
                    text: err.message
                });
            }
        }
    },

    async _showCoverPicker() {
        if (!this.currentPlaylist) return;
        const t = (key, fallback = '') => (typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key);

        const input = document.createElement('input');
        input.type = 'file';
        input.accept = 'image/*';
        input.style.display = 'none';
        document.body.appendChild(input);

        input.addEventListener('change', async () => {
            const file = input.files[0];
            input.remove();
            if (!file) return;

            try {
                const formData = new FormData();
                formData.append('file', file);
                const folderId = window.app?.currentPath || window.app?.userHomeFolderId || '';
                formData.append('folder_id', folderId);

                const uploadResp = await fetch('/api/files/upload', {
                    method: 'POST',
                    credentials: 'include',
                    headers: typeof getCsrfHeaders === 'function' ? getCsrfHeaders() : {},
                    body: formData
                });
                if (!uploadResp.ok) throw new Error('Upload failed');
                const uploaded = await uploadResp.json();
                if (!uploaded.id) throw new Error('No file ID returned');

                const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}`, {
                    method: 'PUT',
                    credentials: 'include',
                    headers: this._headers(true),
                    body: JSON.stringify({ cover_file_id: uploaded.id })
                });
                if (!resp.ok) throw new Error('Failed to set cover');

                this.currentPlaylist.cover_file_id = uploaded.id;
                const plIdx = this.playlists.findIndex((p) => p.id === this.currentPlaylist.id);
                if (plIdx !== -1) this.playlists[plIdx].cover_file_id = uploaded.id;

                const coverEl = document.getElementById('music-playlist-cover');
                if (coverEl) {
                    coverEl.innerHTML = `<img src="/api/files/${encodeURIComponent(uploaded.id)}" alt="" class="music-cover-img"><div class="music-cover-overlay"><i class="fas fa-camera"></i></div>`;
                }

                if (window.notifications) {
                    window.notifications.addNotification({
                        icon: 'fa-check-circle',
                        iconClass: 'upload',
                        title: t('music.set_cover', 'Set cover'),
                        text: t('music.cover_updated', 'Cover updated')
                    });
                }
            } catch (err) {
                console.error('Cover upload error:', err);
                if (window.notifications) {
                    window.notifications.addNotification({
                        icon: 'fa-exclamation-circle',
                        iconClass: 'error',
                        title: t('music.error', 'Error'),
                        text: err.message
                    });
                }
            }
        });

        input.click();
    }
};

/**
 * Music Player Module
 * Handles audio playback, queue, and controls
 */
const musicPlayer = {
    audio: null,
    queue: [],
    currentIndex: -1,
    currentTrack: null,
    isPlaying: false,
    volume: 0.7,
    isMuted: false,
    shuffle: false,
    repeat: 'none',
    playlistName: '',
    _initialized: false,

    init() {
        if (this._initialized) return;

        this.audio = new Audio();
        this.audio.volume = this.volume;

        this.audio.addEventListener('ended', () => this._onEnded());
        this.audio.addEventListener('timeupdate', () => this._onTimeUpdate());
        this.audio.addEventListener('loadedmetadata', () => this._onLoadedMetadata());
        this.audio.addEventListener('play', () => this._onPlay());
        this.audio.addEventListener('pause', () => this._onPause());
        this.audio.addEventListener('error', (e) => this._onError(e));
        this.audio.addEventListener('canplay', () => this._onCanPlay());

        this._createPlayerUI();
        this._bindEvents();
        this._initialized = true;
    },

    _createPlayerUI() {
        if (document.getElementById('music-player')) return;

        const playerEl = document.createElement('div');
        playerEl.id = 'music-player';
        playerEl.className = 'music-player';
        playerEl.innerHTML = `
            <div class="player-track-info">
                <div class="player-album-art">
                    <i class="fas fa-music"></i>
                </div>
                <div class="player-track-details">
                    <span class="player-track-name">${i18n?.t('music.not_playing', 'Not playing') || 'Not playing'}</span>
                    <span class="player-track-artist"></span>
                </div>
            </div>
            <div class="player-controls">
                <div class="player-buttons">
                    <button class="player-btn" id="player-shuffle-btn" title="${i18n?.t('music.shuffle', 'Shuffle') || 'Shuffle'}">
                        <i class="fas fa-shuffle"></i>
                    </button>
                    <button class="player-btn" id="player-prev-btn" title="${i18n?.t('music.previous', 'Previous') || 'Previous'}">
                        <i class="fas fa-backward"></i>
                    </button>
                    <button class="player-btn player-btn-main" id="player-play-btn" title="${i18n?.t('music.play', 'Play') || 'Play'}">
                        <i class="fas fa-play"></i>
                    </button>
                    <button class="player-btn" id="player-next-btn" title="${i18n?.t('music.next', 'Next') || 'Next'}">
                        <i class="fas fa-forward"></i>
                    </button>
                    <button class="player-btn" id="player-repeat-btn" title="${i18n?.t('music.repeat', 'Repeat') || 'Repeat'}">
                        <i class="fas fa-repeat"></i>
                    </button>
                </div>
                <div class="player-progress">
                    <span class="player-time player-time-current" id="player-current-time">0:00</span>
                    <div class="player-progress-bar" id="player-progress-bar">
                        <div class="player-progress-fill" id="player-progress-fill"></div>
                        <div class="player-progress-handle" id="player-progress-handle"></div>
                    </div>
                    <span class="player-time player-time-total" id="player-total-time">0:00</span>
                </div>
            </div>
            <div class="player-extra">
                <button class="player-btn player-btn-small" id="player-playlist-btn" title="${i18n?.t('music.queue', 'Queue') || 'Queue'}">
                    <i class="fas fa-list"></i>
                </button>
                <button class="player-btn player-btn-small" id="player-vol-btn" title="${i18n?.t('music.volume', 'Volume') || 'Volume'}">
                    <i class="fas fa-volume-up"></i>
                </button>
                <div class="player-volume-slider" id="player-volume-slider">
                    <input type="range" min="0" max="100" value="70" id="player-volume-input">
                </div>
                <button class="player-btn player-btn-small player-close-btn" id="player-close-btn" title="${i18n?.t('actions.close', 'Close') || 'Close'}">
                    <i class="fas fa-times"></i>
                </button>
            </div>
            <div class="player-queue hidden" id="player-queue">
                <div class="player-queue-header">
                    <h3>${i18n?.t('music.queue', 'Queue') || 'Queue'}</h3>
                    <button class="player-btn player-btn-small" id="player-close-queue-btn">
                        <i class="fas fa-times"></i>
                    </button>
                </div>
                <div class="player-queue-list" id="player-queue-list"></div>
            </div>
        `;
        document.body.appendChild(playerEl);
    },

    _bindEvents() {
        const playBtn = document.getElementById('player-play-btn');
        const prevBtn = document.getElementById('player-prev-btn');
        const nextBtn = document.getElementById('player-next-btn');
        const shuffleBtn = document.getElementById('player-shuffle-btn');
        const repeatBtn = document.getElementById('player-repeat-btn');
        const progressBar = document.getElementById('player-progress-bar');
        const volumeInput = document.getElementById('player-volume-input');
        const volBtn = document.getElementById('player-vol-btn');
        const playlistBtn = document.getElementById('player-playlist-btn');
        const closeQueueBtn = document.getElementById('player-close-queue-btn');
        const queue = document.getElementById('player-queue');

        if (playBtn) {
            playBtn.addEventListener('click', () => this.togglePlay());
        }

        if (prevBtn) {
            prevBtn.addEventListener('click', () => this.prev());
        }

        if (nextBtn) {
            nextBtn.addEventListener('click', () => this.next());
        }

        if (shuffleBtn) {
            shuffleBtn.addEventListener('click', () => this.toggleShuffle());
        }

        if (repeatBtn) {
            repeatBtn.addEventListener('click', () => this.toggleRepeat());
        }

        if (progressBar) {
            progressBar.addEventListener('click', (e) => this._seek(e));
        }

        if (volumeInput) {
            volumeInput.addEventListener('input', (e) => {
                this.setVolume(e.target.value / 100);
            });
        }

        if (volBtn) {
            volBtn.addEventListener('click', () => this.toggleMute());
        }

        if (playlistBtn) {
            playlistBtn.addEventListener('click', () => this._toggleQueue());
        }

        if (closeQueueBtn) {
            closeQueueBtn.addEventListener('click', () => this._toggleQueue(false));
        }

        if (queue) {
            queue.addEventListener('click', (e) => {
                if (e.target === queue) {
                    this._toggleQueue(false);
                }
            });
        }

        const closeBtn = document.getElementById('player-close-btn');
        if (closeBtn) {
            closeBtn.addEventListener('click', () => this.closePlayer());
        }
    },

    closePlayer() {
        this.audio.pause();
        this.audio.src = '';
        this.isPlaying = false;
        this.currentTrack = null;
        this.currentIndex = -1;
        this.queue = [];
        this._updateUI();
        this._updateQueueUI();
        this._toggleQueue(false);
        document.body.classList.remove('music-player-active');
    },

    setQueue(tracks, playlistName = '') {
        this.queue = [...tracks];
        this.playlistName = playlistName;
        this._updateQueueUI();
    },

    playTrack(index) {
        if (index < 0 || index >= this.queue.length) return;

        this.currentIndex = index;
        this.currentTrack = this.queue[index];
        this._loadAndPlay();
    },

    _loadAndPlay() {
        if (!this.currentTrack) return;

        const fileId = this.currentTrack.file_id;
        this.audio.src = `/api/files/${fileId}`;
        this.audio.play().catch((err) => {
            console.error('Playback error:', err);
        });
        this.isPlaying = true;
        this._updateUI();
    },

    togglePlay() {
        if (!this.currentTrack) {
            if (this.queue.length > 0) {
                this.playTrack(0);
            }
            return;
        }

        if (this.isPlaying) {
            this.audio.pause();
        } else {
            this.audio.play();
        }
    },

    play() {
        if (this.currentTrack && !this.isPlaying) {
            this.audio.play();
        }
    },

    pause() {
        if (this.isPlaying) {
            this.audio.pause();
        }
    },

    next() {
        if (this.queue.length === 0) return;

        let nextIndex;
        if (this.shuffle) {
            nextIndex = Math.floor(Math.random() * this.queue.length);
        } else {
            nextIndex = this.currentIndex + 1;
            if (nextIndex >= this.queue.length) {
                nextIndex = 0;
            }
        }
        this.playTrack(nextIndex);
    },

    prev() {
        if (this.queue.length === 0) return;

        if (this.audio.currentTime > 3) {
            this.audio.currentTime = 0;
            return;
        }

        let prevIndex = this.currentIndex - 1;
        if (prevIndex < 0) {
            prevIndex = this.queue.length - 1;
        }
        this.playTrack(prevIndex);
    },

    toggleShuffle() {
        this.shuffle = !this.shuffle;
        const btn = document.getElementById('player-shuffle-btn');
        if (btn) {
            btn.classList.toggle('active', this.shuffle);
        }
    },

    toggleRepeat() {
        const modes = ['none', 'all', 'one'];
        const currentIdx = modes.indexOf(this.repeat);
        this.repeat = modes[(currentIdx + 1) % modes.length];

        const btn = document.getElementById('player-repeat-btn');
        if (btn) {
            btn.classList.remove('active', 'repeat-one');
            if (this.repeat === 'all') {
                btn.classList.add('active');
            } else if (this.repeat === 'one') {
                btn.classList.add('active', 'repeat-one');
            }
        }
    },

    setVolume(vol) {
        this.volume = Math.max(0, Math.min(1, vol));
        this.audio.volume = this.volume;
        this.isMuted = this.volume === 0;
        this._updateVolumeIcon();

        const input = document.getElementById('player-volume-input');
        if (input) {
            input.value = this.volume * 100;
        }
    },

    toggleMute() {
        this.isMuted = !this.isMuted;
        this.audio.muted = this.isMuted;
        this._updateVolumeIcon();
    },

    _updateVolumeIcon() {
        const btn = document.getElementById('player-vol-btn');
        if (!btn) return;

        let icon;
        if (this.isMuted || this.volume === 0) {
            icon = 'fa-volume-mute';
        } else if (this.volume < 0.5) {
            icon = 'fa-volume-down';
        } else {
            icon = 'fa-volume-up';
        }

        btn.querySelector('i').className = `fas ${icon}`;
    },

    _seek(e) {
        const bar = document.getElementById('player-progress-bar');
        if (!bar) return;

        const rect = bar.getBoundingClientRect();
        const percent = (e.clientX - rect.left) / rect.width;
        if (this.audio.duration) {
            this.audio.currentTime = percent * this.audio.duration;
        }
    },

    _onEnded() {
        if (this.repeat === 'one') {
            this.audio.currentTime = 0;
            this.audio.play();
        } else if (this.repeat === 'all' || this.currentIndex < this.queue.length - 1) {
            this.next();
        } else {
            this.isPlaying = false;
            this._updateUI();
        }
    },

    _onTimeUpdate() {
        if (!this.audio.duration) return;

        const current = this.audio.currentTime;
        const total = this.audio.duration;
        const percent = (current / total) * 100;

        const fill = document.getElementById('player-progress-fill');
        const handle = document.getElementById('player-progress-handle');
        const currentTimeEl = document.getElementById('player-current-time');

        if (fill) fill.style.width = `${percent}%`;
        if (handle) handle.style.left = `${percent}%`;
        if (currentTimeEl) currentTimeEl.textContent = this._formatTime(current);
    },

    _onLoadedMetadata() {
        const totalTimeEl = document.getElementById('player-total-time');
        if (totalTimeEl) {
            totalTimeEl.textContent = this._formatTime(this.audio.duration);
        }

        if (this.currentTrack && this.audio.duration) {
            this.currentTrack.duration_secs = Math.round(this.audio.duration);

            const trackDurationEl = document.querySelector(`.music-track[data-idx="${this.currentIndex}"] .music-track-duration`);
            if (trackDurationEl) {
                trackDurationEl.textContent = this._formatDuration(this.audio.duration);
            }

            const queueDurationEl = document.querySelector(`.queue-item[data-idx="${this.currentIndex}"] .queue-item-duration`);
            if (queueDurationEl) {
                queueDurationEl.textContent = this._formatDuration(this.audio.duration);
            }
        }
    },

    _onPlay() {
        this.isPlaying = true;
        this._updateUI();
    },

    _onPause() {
        this.isPlaying = false;
        this._updateUI();
    },

    _onCanPlay() {
        this._updateUI();
    },

    _onError(e) {
        console.error('Audio error:', e);
        this.isPlaying = false;
        this._updateUI();
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };
        if (window.notifications) {
            const trackName = this.currentTrack?.title || this.currentTrack?.file_name || t('music.unknown_title', 'Unknown');
            window.notifications.addNotification({
                icon: 'fa-exclamation-circle',
                iconClass: 'error',
                title: t('music.error', 'Error'),
                text: `${t('music.playback_error', 'Playback failed')}: ${trackName}`
            });
        }
    },

    _updateUI() {
        const playBtn = document.getElementById('player-play-btn');
        const trackName = document.querySelector('.player-track-name');
        const trackArtist = document.querySelector('.player-track-artist');

        if (playBtn) {
            const icon = playBtn.querySelector('i') || playBtn.querySelector('svg');
            if (icon) {
                const iconName = this.isPlaying ? 'pause' : 'play';
                const extraClass = 'player-btn-main';
                if (window.oxiIcon) {
                    icon.outerHTML = window.oxiIcon(iconName, extraClass);
                } else {
                    icon.className = `fas fa-${iconName} ${extraClass}`;
                }
            }
        }

        if (trackName) {
            const t = (key, fallback = '') => {
                return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
            };
            trackName.textContent = this.currentTrack
                ? this.currentTrack.title || this.currentTrack.file_name || t('music.unknown_title', 'Unknown')
                : t('music.not_playing', 'Not playing');
        }

        if (trackArtist) {
            trackArtist.textContent = this.currentTrack?.artist || '';
        }

        if (musicView.currentTracks.length > 0) {
            document.querySelectorAll('.music-track').forEach((row) => {
                const idx = parseInt(row.dataset.idx, 10);
                row.classList.toggle('playing', idx === this.currentIndex && this.isPlaying);

                const numText = row.querySelector('.track-num-text');
                const playIcon = row.querySelector('.track-play-icon');

                if (idx === this.currentIndex) {
                    if (numText) numText.classList.add('hidden');
                    if (playIcon) playIcon.classList.remove('hidden');
                    if (playIcon) {
                        const iconName = this.isPlaying ? 'pause' : 'play';
                        if (window.oxiIcon) {
                            playIcon.outerHTML = window.oxiIcon(iconName, 'track-play-icon');
                        } else {
                            playIcon.className = `fas fa-${iconName} track-play-icon`;
                        }
                    }
                } else {
                    if (numText) numText.classList.remove('hidden');
                    if (playIcon) playIcon.classList.add('hidden');
                }
            });
        }

        const player = document.getElementById('music-player');
        if (player) {
            const hadTrack = player.classList.contains('has-track');
            const hasTrack = !!this.currentTrack;
            player.classList.toggle('has-track', hasTrack);
            if (hasTrack && !hadTrack) {
                document.body.classList.add('music-player-active');
            } else if (!hasTrack && hadTrack) {
                document.body.classList.remove('music-player-active');
            }
        }
    },

    _updateQueueUI() {
        const queueList = document.getElementById('player-queue-list');
        if (!queueList) return;

        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (this.queue.length === 0) {
            queueList.innerHTML = `
                <div class="player-queue-empty">
                    <i class="fas fa-music"></i>
                    <p>${t('music.queue_empty', 'Queue is empty')}</p>
                </div>
            `;
            return;
        }

        queueList.innerHTML = this.queue
            .map(
                (track, idx) => `
            <div class="player-queue-item ${idx === this.currentIndex ? 'active' : ''}" data-idx="${idx}">
                <span class="queue-item-num">${idx + 1}</span>
                <span class="queue-item-info">
                    <span class="queue-item-name">${this._escapeHtml(track.title || track.file_name || t('music.unknown_title', 'Unknown'))}</span>
                    <span class="queue-item-artist">${this._escapeHtml(track.artist || t('music.unknown_artist', 'Unknown Artist'))}</span>
                </span>
                <span class="queue-item-duration">${this._formatDuration(track.duration_secs)}</span>
                <button class="queue-item-remove" data-idx="${idx}">
                    <i class="fas fa-times"></i>
                </button>
            </div>
        `
            )
            .join('');

        queueList.querySelectorAll('.player-queue-item').forEach((item) => {
            item.addEventListener('click', (e) => {
                if (e.target.closest('.queue-item-remove')) return;
                const idx = parseInt(item.dataset.idx, 10);
                this.playTrack(idx);
            });
        });

        queueList.querySelectorAll('.queue-item-remove').forEach((btn) => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                const idx = parseInt(btn.dataset.idx, 10);
                this._removeFromQueue(idx);
            });
        });
    },

    _removeFromQueue(idx) {
        if (idx === this.currentIndex) {
            if (this.queue.length === 1) {
                this.audio.pause();
                this.queue.splice(idx, 1);
                this.currentTrack = null;
                this.currentIndex = -1;
            } else {
                this.queue.splice(idx, 1);
                if (idx >= this.queue.length) {
                    this.currentIndex = 0;
                } else {
                    this.currentIndex = idx;
                }
                this.currentTrack = this.queue[this.currentIndex];
                this._loadAndPlay();
            }
        } else {
            this.queue.splice(idx, 1);
            if (idx < this.currentIndex) {
                this.currentIndex--;
            }
        }
        this._updateQueueUI();
        this._updateUI();
    },

    _toggleQueue(show) {
        const queue = document.getElementById('player-queue');
        if (queue) {
            if (show === undefined) {
                queue.classList.toggle('hidden');
            } else {
                queue.classList.toggle('hidden', !show);
            }
        }
    },

    _formatTime(secs) {
        if (!secs || Number.isNaN(secs)) return '0:00';
        const mins = Math.floor(secs / 60);
        const s = Math.floor(secs % 60);
        return `${mins}:${s.toString().padStart(2, '0')}`;
    },

    _formatDuration(secs) {
        return this._formatTime(secs);
    },

    _escapeHtml(str) {
        if (!str) return '';
        return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;');
    }
};

window.musicView = musicView;
