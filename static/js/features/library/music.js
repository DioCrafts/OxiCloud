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

        this._container.innerHTML = `
            <div class="music-toolbar">
                <button class="btn btn-primary" id="music-create-playlist-btn">
                    <i class="fas fa-plus"></i>
                    <span>${t('music.create_playlist', 'Create Playlist')}</span>
                </button>
            </div>
            <div class="music-content">
                <div class="music-sidebar">
                    <div class="music-sidebar-header">
                        <h3>${t('music.playlists', 'Playlists')}</h3>
                    </div>
                    <div class="music-playlist-list" id="music-playlist-list">
                        ${this.playlists.length === 0 ? `
                            <div class="music-empty">
                                <i class="fas fa-music"></i>
                                <p>${t('music.no_playlists', 'No playlists yet')}</p>
                            </div>
                        ` : ''}
                    </div>
                </div>
                <div class="music-main">
                    <div class="music-welcome">
                        <i class="fas fa-music"></i>
                        <h3>${t('music.select_playlist', 'Select a playlist')}</h3>
                        <p>${t('music.select_hint', 'Choose a playlist from the sidebar or create a new one')}</p>
                    </div>
                    <div class="music-playlist-detail hidden" id="music-playlist-detail">
                        <div class="music-playlist-header">
                            <div class="music-playlist-cover">
                                <i class="fas fa-music"></i>
                            </div>
                            <div class="music-playlist-info">
                                <h2 id="music-playlist-name"></h2>
                                <p id="music-playlist-meta"></p>
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
                            <button class="btn btn-secondary" id="music-edit-playlist-btn">
                                <i class="fas fa-edit"></i>
                            </button>
                            <button class="btn btn-secondary" id="music-share-playlist-btn">
                                <i class="fas fa-share"></i>
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

        if (this.playlists.length === 0) {
            listEl.innerHTML = `
                <div class="music-empty">
                    <i class="fas fa-music"></i>
                    <p>No playlists yet</p>
                </div>
            `;
            return;
        }

        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        listEl.innerHTML = this.playlists.map(p => `
            <div class="music-playlist-item" data-id="${p.id}">
                <div class="music-playlist-icon">
                    <i class="fas fa-list"></i>
                </div>
                <div class="music-playlist-item-info">
                    <span class="music-playlist-item-name">${this._escapeHtml(p.name)}</span>
                    <span class="music-playlist-item-count">${p.track_count || 0} ${t('music.tracks', 'tracks')}</span>
                </div>
            </div>
        `).join('');

        listEl.querySelectorAll('.music-playlist-item').forEach(item => {
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
    },

    async _selectPlaylist(playlistId) {
        const playlist = this.playlists.find(p => p.id === playlistId);
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
            metaEl.textContent = `${playlist.track_count || 0} tracks`;
        }

        document.querySelectorAll('.music-playlist-item').forEach(item => {
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
                <span class="music-track-col music-track-num">#</span>
                <span class="music-track-col music-track-title">${t('music.title', 'Title')}</span>
                <span class="music-track-col music-track-artist">${t('music.artist', 'Artist')}</span>
                <span class="music-track-col music-track-album">${t('music.album', 'Album')}</span>
                <span class="music-track-col music-track-duration"><i class="far fa-clock"></i></span>
            </div>
            ${this.currentTracks.map((track, idx) => `
                <div class="music-track ${musicPlayer.currentTrack?.id === track.id ? 'playing' : ''}" data-idx="${idx}" data-id="${track.id}" data-file-id="${track.file_id}">
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
                </div>
            `).join('')}
        `;

        const self = this;
        trackListEl.querySelectorAll('.music-track').forEach(row => {
            row.addEventListener('click', () => {
                const idx = parseInt(row.dataset.idx);
                self._playTrack(idx);
            });
            
            row.addEventListener('dblclick', () => {
                const idx = parseInt(row.dataset.idx);
                self._playTrack(idx);
            });
        });
    },

    _playTrack(idx) {
        if (!this.currentTracks[idx]) return;
        
        musicPlayer.setQueue(this.currentTracks, this.currentPlaylist?.name || 'Playlist');
        musicPlayer.playTrack(idx);
    },

    _playAll() {
        if (this.currentTracks.length > 0) {
            this._playTrack(0);
        }
    },

    _shufflePlay() {
        if (this.currentTracks.length > 0) {
            const shuffled = [...this.currentTracks].sort(() => Math.random() - 0.5);
            musicPlayer.setQueue(shuffled, this.currentPlaylist?.name || 'Shuffle');
            musicPlayer.playTrack(0);
        }
    },

    _showCreatePlaylistDialog() {
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        const name = prompt(t('music.playlist_name', 'Playlist name:'));
        if (!name || !name.trim()) return;

        this._createPlaylist(name.trim());
    },

    async _createPlaylist(name) {
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
            this._renderPlaylistList();
            this._selectPlaylist(playlist.id);
        } catch (err) {
            console.error('Create playlist error:', err);
            alert('Failed to create playlist: ' + err.message);
        }
    },

    async _deletePlaylist() {
        const t = (key, fallback = '') => {
            return typeof i18n !== 'undefined' && i18n.t ? i18n.t(key) : fallback || key;
        };

        if (!this.currentPlaylist) return;
        if (!confirm(t('music.confirm_delete', 'Delete this playlist?'))) return;

        try {
            const resp = await fetch(`/api/playlists/${this.currentPlaylist.id}`, {
                method: 'DELETE',
                credentials: 'include',
                headers: this._headers()
            });

            if (!resp.ok) throw new Error('Failed to delete playlist');

            this.playlists = this.playlists.filter(p => p.id !== this.currentPlaylist.id);
            this.currentPlaylist = null;
            this.currentTracks = [];
            this._renderPlaylists();
        } catch (err) {
            console.error('Delete playlist error:', err);
            alert('Failed to delete playlist: ' + err.message);
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
                    <i class="fas fa-volume"></i>
                </button>
                <div class="player-volume-slider" id="player-volume-slider">
                    <input type="range" min="0" max="100" value="70" id="player-volume-input">
                </div>
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
        this.audio.play().catch(err => {
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
            trackName.textContent = this.currentTrack 
                ? (this.currentTrack.title || this.currentTrack.file_name || 'Unknown')
                : (i18n?.t('music.not_playing', 'Not playing') || 'Not playing');
        }

        if (trackArtist) {
            trackArtist.textContent = this.currentTrack?.artist || '';
        }

        if (musicView.currentTracks.length > 0) {
            document.querySelectorAll('.music-track').forEach(row => {
                const idx = parseInt(row.dataset.idx);
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
            player.classList.toggle('has-track', !!this.currentTrack);
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

        queueList.innerHTML = this.queue.map((track, idx) => `
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
        `).join('');

        queueList.querySelectorAll('.player-queue-item').forEach(item => {
            item.addEventListener('click', (e) => {
                if (e.target.closest('.queue-item-remove')) return;
                const idx = parseInt(item.dataset.idx);
                this.playTrack(idx);
            });
        });

        queueList.querySelectorAll('.queue-item-remove').forEach(btn => {
            btn.addEventListener('click', (e) => {
                e.stopPropagation();
                const idx = parseInt(btn.dataset.idx);
                this._removeFromQueue(idx);
            });
        });
    },

    _removeFromQueue(idx) {
        if (idx === this.currentIndex) {
            if (this.queue.length === 1) {
                this.audio.pause();
                this.currentTrack = null;
                this.currentIndex = -1;
            } else {
                this.next();
            }
        }
        this.queue.splice(idx, 1);
        if (idx < this.currentIndex) {
            this.currentIndex--;
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
        if (!secs || isNaN(secs)) return '0:00';
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

document.addEventListener('DOMContentLoaded', () => {
    musicPlayer.init();
});

window.musicView = musicView;
