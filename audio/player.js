// Smart Non-Repeating Audio Player for Infinite GPU Fractals
(function() {
    const PLAYLIST = [
        { id: 1, title: "Fire In My Soul", artist: "Oliver Heldens feat. Shungudzo", file: "audio/tracks/track_01_fire_in_my_soul.mp3" },
        { id: 2, title: "No Limits (Vocal Mix)", artist: "Danism, Train & DJ Rae", file: "audio/tracks/track_02_no_limits_vocal_mix.mp3" },
        { id: 3, title: "Say My Name (Sub Focus Remix)", artist: "Morgan Seatree, Sub Focus", file: "audio/tracks/track_03_say_my_name_sub_focus_remix.mp3" },
        { id: 4, title: "Bambou (Original Mix)", artist: "Sebastien Leger", file: "audio/tracks/track_04_bambou_original_mix.mp3" },
        { id: 5, title: "TRONCE", artist: "Sili", file: "audio/tracks/track_05_tronce.mp3" },
        { id: 6, title: "Beautiful", artist: "Brookes Brothers feat. Robert Owens", file: "audio/tracks/track_06_beautiful.mp3" },
        { id: 7, title: "Escapism (Original Mix)", artist: "cYsmix", file: "audio/tracks/track_07_escapism_original_mix.mp3" },
        { id: 8, title: "Unity", artist: "TheFatRat", file: "audio/tracks/track_08_unity.mp3" },
        { id: 9, title: "Underground", artist: "Tantrum Desire", file: "audio/tracks/track_09_underground.mp3" },
        { id: 10, title: "Rhyme Dust (Dimension Remix)", artist: "MK, Dom Dolla", file: "audio/tracks/track_10_rhyme_dust_dimension_remix.mp3" },
        { id: 11, title: "Horizon", artist: "1991, Poppy Baskcomb", file: "audio/tracks/track_11_horizon.mp3" },
        { id: 12, title: "Colours & Lights (Remix)", artist: "GoldFish & Cat Dealers", file: "audio/tracks/track_12_colours_lights_clément_leroux_.mp3" },
        { id: 13, title: "Mend Your Ways", artist: "PSYQUI", file: "audio/tracks/track_13_mend_your_ways.mp3" },
        { id: 14, title: "Nights Introlude", artist: "Nightmares On Wax", file: "audio/tracks/track_14_nights_introlude.mp3" },
        { id: 15, title: "Genesis", artist: "Subsonic", file: "audio/tracks/track_15_genesis.mp3" },
        { id: 16, title: "Get To Me", artist: "Culture Shock", file: "audio/tracks/track_16_get_to_me.mp3" },
        { id: 17, title: "Focused", artist: "Soulfreq", file: "audio/tracks/track_17_focused.mp3" },
        { id: 18, title: "King Of The Swingers", artist: "Krushed & Sorted", file: "audio/tracks/track_18_king_of_the_swingers_gettin_ma.mp3" }
    ];

    const STORAGE_KEY = 'valinfinite_played_tracks_v2';

    function getAudioPath(relPath) {
        // Resolve path relative to current document location
        if (window.location.pathname.endsWith('/') || window.location.pathname.endsWith('index.html')) {
            const depth = (window.location.pathname.match(/\//g) || []).length;
            // If in a subfolder like /celestial-heart/, prefix with ../
            const inSubdir = window.location.pathname.split('/').filter(Boolean).length > (window.location.hostname.includes('github.io') ? 1 : 0);
            return inSubdir ? '../' + relPath : relPath;
        }
        return '../' + relPath;
    }

    function getUnplayedTracks() {
        let played = [];
        try {
            played = JSON.parse(localStorage.getItem(STORAGE_KEY) || '[]');
        } catch (e) {
            played = [];
        }
        let unplayed = PLAYLIST.filter(t => !played.includes(t.id));
        if (unplayed.length === 0) {
            played = [];
            unplayed = [...PLAYLIST];
        }
        return { unplayed, played };
    }

    function pickNextTrack() {
        const { unplayed, played } = getUnplayedTracks();
        const randomIndex = Math.floor(Math.random() * unplayed.length);
        const chosen = unplayed[randomIndex];
        played.push(chosen.id);
        try {
            localStorage.setItem(STORAGE_KEY, JSON.stringify(played));
        } catch (e) {}
        return chosen;
    }

    let audioElem = null;
    let currentTrack = null;

    function createAudioPlayerUI() {
        const style = document.createElement('style');
        style.textContent = `
            #audio-controller {
                position: fixed;
                bottom: 20px;
                right: 20px;
                z-index: 9999;
                display: flex;
                align-items: center;
                gap: 10px;
                padding: 8px 14px;
                background: rgba(10, 15, 30, 0.75);
                backdrop-filter: blur(16px);
                -webkit-backdrop-filter: blur(16px);
                border: 1px solid rgba(255, 255, 255, 0.15);
                border-radius: 30px;
                color: #fff;
                font-family: system-ui, -apple-system, sans-serif;
                font-size: 13px;
                box-shadow: 0 8px 24px rgba(0, 0, 0, 0.5);
                transition: opacity 0.3s ease, transform 0.3s ease;
                user-select: none;
            }
            #audio-controller.hidden {
                opacity: 0;
                pointer-events: none;
                transform: translateY(10px);
            }
            .track-meta {
                display: flex;
                flex-direction: column;
                max-width: 170px;
                overflow: hidden;
            }
            .track-title {
                font-weight: 600;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
                color: #e0f2fe;
            }
            .track-artist {
                font-size: 11px;
                color: #94a3b8;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            }
            .audio-btn {
                background: rgba(255, 255, 255, 0.12);
                border: none;
                outline: none;
                cursor: pointer;
                color: #fff;
                width: 28px;
                height: 28px;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 13px;
                transition: background 0.2s, transform 0.1s;
            }
            .audio-btn:hover {
                background: rgba(255, 255, 255, 0.25);
                transform: scale(1.08);
            }
            .audio-btn:active {
                transform: scale(0.94);
            }
        `;
        document.head.appendChild(style);

        const container = document.createElement('div');
        container.id = 'audio-controller';
        container.innerHTML = `
            <button class="audio-btn" id="audio-toggle" title="Play/Pause">▶</button>
            <div class="track-meta">
                <span class="track-title" id="track-name">Loading...</span>
                <span class="track-artist" id="track-by"></span>
            </div>
            <button class="audio-btn" id="audio-next" title="Next Random Track">⏭</button>
        `;
        document.body.appendChild(container);

        const toggleBtn = document.getElementById('audio-toggle');
        const nextBtn = document.getElementById('audio-next');

        toggleBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            if (!audioElem) return;
            if (audioElem.paused) {
                audioElem.play().then(() => {
                    toggleBtn.textContent = '⏸';
                }).catch(() => {});
            } else {
                audioElem.pause();
                toggleBtn.textContent = '▶';
            }
        });

        nextBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            playNext();
        });
    }

    function updateTrackUI(track) {
        const titleElem = document.getElementById('track-name');
        const artistElem = document.getElementById('track-by');
        const toggleBtn = document.getElementById('audio-toggle');
        if (titleElem) titleElem.textContent = track.title;
        if (artistElem) artistElem.textContent = track.artist;
        if (toggleBtn) toggleBtn.textContent = (audioElem && !audioElem.paused) ? '⏸' : '▶';
    }

    function playTrack(track) {
        currentTrack = track;
        const filePath = getAudioPath(track.file);

        if (!audioElem) {
            audioElem = new Audio();
            audioElem.volume = 0.85;
            audioElem.addEventListener('ended', playNext);
            audioElem.addEventListener('play', () => {
                const btn = document.getElementById('audio-toggle');
                if (btn) btn.textContent = '⏸';
            });
            audioElem.addEventListener('pause', () => {
                const btn = document.getElementById('audio-toggle');
                if (btn) btn.textContent = '▶';
            });
        }

        audioElem.src = filePath;
        updateTrackUI(track);

        const playPromise = audioElem.play();
        if (playPromise !== undefined) {
            playPromise.then(() => {
                const btn = document.getElementById('audio-toggle');
                if (btn) btn.textContent = '⏸';
            }).catch(() => {
                // Autoplay blocked by browser policy; wait for first user gesture
                const unlock = () => {
                    if (audioElem && audioElem.paused) {
                        audioElem.play().catch(() => {});
                    }
                    ['click', 'touchstart', 'pointerdown', 'keydown'].forEach(evt => {
                        window.removeEventListener(evt, unlock);
                    });
                };
                ['click', 'touchstart', 'pointerdown', 'keydown'].forEach(evt => {
                    window.addEventListener(evt, unlock, { passive: true });
                });
            });
        }
    }

    function playNext() {
        const next = pickNextTrack();
        playTrack(next);
    }

    window.initAudioPlayer = function() {
        createAudioPlayerUI();
        playNext();
    };

    if (document.readyState === 'loading') {
        window.addEventListener('DOMContentLoaded', window.initAudioPlayer);
    } else {
        window.initAudioPlayer();
    }
})();
