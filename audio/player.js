// Smart Non-Repeating Audio Player for Infinite GPU Fractals with Cover Art,
// Condensed Typography, Cross-Scene Playback Continuity, and Seamless Scene Switching
(function() {
    // Guard: Do not run or play music on landing screen (gallery portal)
    if (document.getElementById('bg-canvas') || (!document.getElementById('fractal-canvas') && !window.location.pathname.includes('-'))) {
        return;
    }

    const PLAYLIST = [
        { id: 1, title: "Fire In My Soul", artist: "Oliver Heldens feat. Shungudzo", file: "audio/tracks/track_01_fire_in_my_soul.mp3", cover: "audio/covers/cover_01.jpg" },
        { id: 2, title: "No Limits (Vocal Mix)", artist: "Danism, Train & DJ Rae", file: "audio/tracks/track_02_no_limits_vocal_mix.mp3", cover: "audio/covers/cover_02.jpg" },
        { id: 3, title: "Say My Name (Sub Focus Remix)", artist: "Morgan Seatree, Sub Focus", file: "audio/tracks/track_03_say_my_name_sub_focus_remix.mp3", cover: "audio/covers/cover_03.jpg" },
        { id: 4, title: "Bambou (Original Mix)", artist: "Sebastien Leger", file: "audio/tracks/track_04_bambou_original_mix.mp3", cover: "audio/covers/cover_04.jpg" },
        { id: 5, title: "TRONCE", artist: "Sili", file: "audio/tracks/track_05_tronce.mp3", cover: "audio/covers/cover_05.jpg" },
        { id: 6, title: "Beautiful", artist: "Brookes Brothers feat. Robert Owens", file: "audio/tracks/track_06_beautiful.mp3", cover: "audio/covers/cover_06.jpg" },
        { id: 7, title: "Escapism (Original Mix)", artist: "cYsmix", file: "audio/tracks/track_07_escapism_original_mix.mp3", cover: "audio/covers/cover_07.jpg" },
        { id: 8, title: "Unity", artist: "TheFatRat", file: "audio/tracks/track_08_unity.mp3", cover: "audio/covers/cover_08.jpg" },
        { id: 9, title: "Underground", artist: "Tantrum Desire", file: "audio/tracks/track_09_underground.mp3", cover: "audio/covers/cover_09.jpg" },
        { id: 10, title: "Rhyme Dust (Dimension Remix)", artist: "MK, Dom Dolla", file: "audio/tracks/track_10_rhyme_dust_dimension_remix.mp3", cover: "audio/covers/cover_10.jpg" },
        { id: 11, title: "Horizon", artist: "1991, Poppy Baskcomb", file: "audio/tracks/track_11_horizon.mp3", cover: "audio/covers/cover_11.jpg" },
        { id: 12, title: "Colours & Lights (Remix)", artist: "GoldFish & Cat Dealers", file: "audio/tracks/track_12_colours_lights_clément_leroux_.mp3", cover: "audio/covers/cover_12.jpg" },
        { id: 13, title: "Mend Your Ways", artist: "PSYQUI", file: "audio/tracks/track_13_mend_your_ways.mp3", cover: "audio/covers/cover_13.jpg" },
        { id: 14, title: "Nights Introlude", artist: "Nightmares On Wax", file: "audio/tracks/track_14_nights_introlude.mp3", cover: "audio/covers/cover_14.jpg" },
        { id: 15, title: "Genesis", artist: "Subsonic", file: "audio/tracks/track_15_genesis.mp3", cover: "audio/covers/cover_15.jpg" },
        { id: 16, title: "Get To Me", artist: "Culture Shock", file: "audio/tracks/track_16_get_to_me.mp3", cover: "audio/covers/cover_16.jpg" },
        { id: 17, title: "Focused", artist: "Soulfreq", file: "audio/tracks/track_17_focused.mp3", cover: "audio/covers/cover_17.jpg" },
        { id: 18, title: "King Of The Swingers", artist: "Krushed & Sorted", file: "audio/tracks/track_18_king_of_the_swingers_gettin_ma.mp3", cover: "audio/covers/cover_18.jpg" },
        { id: 19, title: "Drugs I Like", artist: "nate band", file: "audio/tracks/track_19_drugs_i_like.mp3", cover: "audio/covers/cover_19.jpg" },
        { id: 20, title: "Remember Me", artist: "High Contrast", file: "audio/tracks/track_20_remember_me.mp3", cover: "audio/covers/cover_20.jpg" },
        { id: 21, title: "TANGARA", artist: "Etherwood, Hugh Hardie", file: "audio/tracks/track_21_tangara.mp3", cover: "audio/covers/cover_21.jpg" },
        { id: 22, title: "Beat Keep Rockin'", artist: "Starjunk 95", file: "audio/tracks/track_22_beat_keep_rockin.mp3", cover: "audio/covers/cover_22.jpg" },
        { id: 23, title: "Spectra Ocean Dream Circuit", artist: "Starjunk 95", file: "audio/tracks/track_23_spectra_ocean_dream_circuit.mp3", cover: "audio/covers/cover_23.jpg" },
        { id: 24, title: "Groove District", artist: "Starjunk 95", file: "audio/tracks/track_24_groove_district.mp3", cover: "audio/covers/cover_24.jpg" },
        { id: 25, title: "Tell You What I Did", artist: "Pola & Bryson, Zitah", file: "audio/tracks/track_25_tell_you_what_i_did.mp3", cover: "audio/covers/cover_25.jpg" },
        { id: 26, title: "TAKE ME", artist: "D A N N Y", file: "audio/tracks/track_26_take_me.mp3", cover: "audio/covers/cover_26.jpg" },
        { id: 27, title: "Mirage", artist: "MPH, Skrillex", file: "audio/tracks/track_27_mirage.mp3", cover: "audio/covers/cover_27.jpg" },
        { id: 28, title: "Liberate (Lane 8 Remix)", artist: "Eric Prydz", file: "audio/tracks/track_28_liberate_lane_8_remix.mp3", cover: "audio/covers/cover_28.jpg" },
        { id: 29, title: "Szikra", artist: "Kornél Kovács", file: "audio/tracks/track_29_szikra.mp3", cover: "audio/covers/cover_29.jpg" },
        { id: 30, title: "I Run", artist: "YUSSI", file: "audio/tracks/track_30_i_run.mp3", cover: "audio/covers/cover_30.jpg" },
        { id: 31, title: "On & On", artist: "Chris Lake, Yael Watchman", file: "audio/tracks/track_31_on_on.mp3", cover: "audio/covers/cover_31.jpg" },
        { id: 32, title: "Out For Blood", artist: "QZB", file: "audio/tracks/track_32_out_for_blood.mp3", cover: "audio/covers/cover_32.jpg" },
        { id: 33, title: "Don't Stop", artist: "MUZZ", file: "audio/tracks/track_33_dont_stop.mp3", cover: "audio/covers/cover_33.jpg" },
        { id: 34, title: "Tu Cafe (Mash Up)", artist: "Prodigy", file: "audio/tracks/track_34_tu_cafe_mash_up.mp3", cover: "audio/covers/cover_34.jpg" },
        { id: 35, title: "The People (Mehlor Remix)", artist: "Harrie Summers, Joey Rich", file: "audio/tracks/track_35_the_people_mehlor_remix.mp3", cover: "audio/covers/cover_35.jpg" },
        { id: 36, title: "Spacefunk", artist: "Stussko, Kolter", file: "audio/tracks/track_36_spacefunk.mp3", cover: "audio/covers/cover_36.jpg" },
        { id: 37, title: "Bunker", artist: "Culture Shock", file: "audio/tracks/track_37_bunker.mp3", cover: "audio/covers/cover_37.jpg" },
        { id: 38, title: "On & On (Kanine Remix)", artist: "Sub Focus, bbyclose, Kanine", file: "audio/tracks/track_38_on_on_kanine_remix.mp3", cover: "audio/covers/cover_38.jpg" }
    ];

    const SCENES = [
        { id: 'celestial-heart', name: 'Celestial Heart', path: '../celestial-heart/' },
        { id: 'matrix-vortex', name: 'Cyber Matrix Vortex', path: '../matrix-vortex/' },
        { id: 'vortex-void', name: 'Vortex Void', path: '../vortex-void/' },
        { id: 'celestial-odyssey', name: 'Celestial Odyssey', path: '../celestial-odyssey/' },
        { id: 'matrix-saga', name: 'Matrix Saga', path: '../matrix-saga/' },
        { id: 'cosmic-infinity', name: 'Cosmic Infinity', path: '../cosmic-infinity/' },
        { id: 'vortex-metamorphosis', name: 'Vortex Metamorphosis', path: '../vortex-metamorphosis/' }
    ];

    const STORAGE_KEY = 'valinfinite_played_tracks_v3';
    const STATE_KEY = 'valinfinite_audio_state_v1';

    // Robust path resolution that works across root portal, subdirectories, localhost, and GitHub Pages
    function getAudioPath(relPath) {
        if (!relPath) return '';
        if (relPath.startsWith('http://') || relPath.startsWith('https://')) return relPath;

        const script = document.currentScript || document.querySelector('script[src*="player.js"]');
        if (script && script.src) {
            try {
                const scriptUrl = new URL(script.src, window.location.href);
                const baseUrl = scriptUrl.href.replace(/\/audio\/player\.js(\?.*)?$/, '');
                return `${baseUrl}/${relPath.replace(/^\//, '')}`;
            } catch (e) {
                // fall through
            }
        }

        const inSubdir = window.location.pathname.split('/').filter(Boolean).length > (window.location.hostname.includes('github.io') ? 1 : 0);
        return inSubdir ? '../' + relPath : relPath;
    }

    function getTrackCover(track) {
        if (track.cover) return getAudioPath(track.cover);
        const idStr = String(track.id).padStart(2, '0');
        return getAudioPath(`audio/covers/cover_${idStr}.jpg`);
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

    // Save audio state for seamless cross-scene continuity
    function saveAudioState(isPlayingOverride) {
        if (!currentTrack || !audioElem) return;
        const isPlaying = (typeof isPlayingOverride === 'boolean') ? isPlayingOverride : !audioElem.paused;
        const state = {
            trackId: currentTrack.id,
            currentTime: audioElem.currentTime || 0,
            isPlaying: isPlaying,
            volume: audioElem.volume,
            timestamp: Date.now()
        };
        try {
            localStorage.setItem(STATE_KEY, JSON.stringify(state));
        } catch (e) {}
    }

    let saveTimer = null;
    function throttledSaveState() {
        if (saveTimer) return;
        saveTimer = setTimeout(() => {
            saveTimer = null;
            saveAudioState();
        }, 250);
    }

    window.addEventListener('pagehide', () => saveAudioState());
    window.addEventListener('beforeunload', () => saveAudioState());

    function createAudioPlayerUI() {
        // Ensure Roboto Condensed font is linked in document head
        if (!document.getElementById('font-roboto-condensed')) {
            const link = document.createElement('link');
            link.id = 'font-roboto-condensed';
            link.rel = 'stylesheet';
            link.href = 'https://fonts.googleapis.com/css2?family=Roboto+Condensed:ital,wght@0,400;0,600;0,700;1,400&display=swap';
            document.head.appendChild(link);
        }

        const style = document.createElement('style');
        style.textContent = `
            @import url('https://fonts.googleapis.com/css2?family=Roboto+Condensed:ital,wght@0,400;0,600;0,700;1,400&display=swap');

            #audio-controller {
                position: fixed;
                bottom: 20px;
                right: 20px;
                z-index: 9999;
                display: flex;
                align-items: center;
                gap: 10px;
                padding: 6px 12px 6px 7px;
                background: rgba(8, 12, 24, 0.85);
                backdrop-filter: blur(20px);
                -webkit-backdrop-filter: blur(20px);
                border: 1px solid rgba(255, 255, 255, 0.14);
                border-radius: 28px;
                color: #fff;
                font-family: 'Roboto Condensed', -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
                box-shadow: 0 8px 30px rgba(0, 0, 0, 0.65), 0 0 1px rgba(255, 255, 255, 0.2);
                transition: opacity 0.3s ease, transform 0.3s ease, border-color 0.3s ease, box-shadow 0.3s ease;
                user-select: none;
                -webkit-user-select: none;
                max-width: calc(100vw - 32px);
                box-sizing: border-box;
            }

            #audio-controller.is-playing {
                border-color: rgba(56, 189, 248, 0.4);
                box-shadow: 0 8px 32px rgba(0, 0, 0, 0.7), 0 0 18px rgba(56, 189, 248, 0.16);
            }

            .track-cover-wrap {
                width: 40px;
                height: 40px;
                min-width: 40px;
                border-radius: 8px;
                overflow: hidden;
                position: relative;
                background: rgba(255, 255, 255, 0.06);
                box-shadow: 0 2px 8px rgba(0, 0, 0, 0.4), inset 0 0 0 1px rgba(255, 255, 255, 0.12);
                display: flex;
                align-items: center;
                justify-content: center;
                flex-shrink: 0;
                cursor: pointer;
            }

            .track-cover {
                width: 100%;
                height: 100%;
                object-fit: cover;
                display: block;
                border-radius: 8px;
                transition: opacity 0.25s ease, transform 0.3s ease;
            }

            .track-cover:hover {
                transform: scale(1.05);
            }

            .track-cover-fallback {
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                display: none;
                align-items: center;
                justify-content: center;
                font-size: 16px;
                color: #94a3b8;
                background: rgba(30, 41, 59, 0.8);
            }

            .track-meta {
                display: flex;
                flex-direction: column;
                justify-content: center;
                min-width: 0;
                max-width: 175px;
                flex: 1 1 auto;
                overflow: hidden;
            }

            .track-title {
                font-family: 'Roboto Condensed', sans-serif;
                font-size: 13px;
                font-weight: 700;
                letter-spacing: 0.02em;
                line-height: 1.25;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
                color: #f1f5f9;
            }

            .track-artist {
                font-family: 'Roboto Condensed', sans-serif;
                font-size: 11px;
                font-weight: 400;
                letter-spacing: 0.01em;
                line-height: 1.25;
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
                color: #94a3b8;
            }

            .audio-controls {
                display: flex;
                align-items: center;
                gap: 6px;
                flex-shrink: 0;
            }

            .audio-btn {
                background: rgba(255, 255, 255, 0.10);
                border: 1px solid rgba(255, 255, 255, 0.10);
                outline: none;
                cursor: pointer;
                color: #ffffff;
                width: 29px;
                height: 29px;
                min-width: 29px;
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                font-size: 12px;
                transition: background 0.2s, transform 0.15s, border-color 0.2s, color 0.2s;
                padding: 0;
            }

            .audio-btn:hover {
                background: rgba(56, 189, 248, 0.25);
                border-color: rgba(56, 189, 248, 0.5);
                color: #38bdf8;
                transform: scale(1.08);
            }

            .audio-btn:active {
                transform: scale(0.92);
            }

            .audio-btn svg {
                display: block;
                width: 12px;
                height: 12px;
                fill: currentColor;
                pointer-events: none;
            }

            /* Subtle Desktop Scene Navigation Arrows */
            .scene-nav-arrow {
                position: fixed;
                top: 50%;
                transform: translateY(-50%);
                width: 44px;
                height: 44px;
                border-radius: 50%;
                background: rgba(8, 12, 24, 0.45);
                backdrop-filter: blur(16px);
                -webkit-backdrop-filter: blur(16px);
                border: 1px solid rgba(255, 255, 255, 0.12);
                color: rgba(255, 255, 255, 0.65);
                display: flex;
                align-items: center;
                justify-content: center;
                cursor: pointer;
                z-index: 9000;
                outline: none;
                padding: 0;
                transition: opacity 0.35s ease, transform 0.2s ease, background 0.2s ease, border-color 0.2s ease, color 0.2s ease, box-shadow 0.2s ease;
                opacity: 0.38;
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
                user-select: none;
                -webkit-user-select: none;
            }

            .scene-nav-arrow:hover {
                opacity: 0.95;
                transform: translateY(-50%) scale(1.12);
                background: rgba(14, 23, 42, 0.85);
                border-color: rgba(56, 189, 248, 0.6);
                color: #38bdf8;
                box-shadow: 0 0 22px rgba(56, 189, 248, 0.35), 0 4px 20px rgba(0, 0, 0, 0.6);
            }

            .scene-nav-arrow:active {
                transform: translateY(-50%) scale(0.94);
            }

            .scene-nav-prev {
                left: 20px;
            }

            .scene-nav-next {
                right: 20px;
            }

            body.idle .scene-nav-arrow {
                opacity: 0;
                pointer-events: none;
            }

            body.scene-switching {
                opacity: 0;
                transition: opacity 0.25s ease;
            }

            @media (max-width: 640px) {
                .scene-nav-arrow {
                    display: none !important;
                }
            }

            /* Responsive adjustments for mobile viewports */
            @media (max-width: 600px) {
                #audio-controller {
                    bottom: 12px;
                    right: 12px;
                    max-width: calc(100vw - 24px);
                    padding: 5px 10px 5px 6px;
                    gap: 8px;
                    border-radius: 24px;
                }
                .track-cover-wrap {
                    width: 36px;
                    height: 36px;
                    min-width: 36px;
                    border-radius: 6px;
                }
                .track-cover {
                    border-radius: 6px;
                }
                .track-meta {
                    max-width: 140px;
                }
                .track-title {
                    font-size: 12px;
                }
                .track-artist {
                    font-size: 10.5px;
                }
                .audio-btn {
                    width: 27px;
                    height: 27px;
                    min-width: 27px;
                    font-size: 11px;
                }
            }

            @media (max-width: 380px) {
                #audio-controller {
                    bottom: 8px;
                    right: 8px;
                    max-width: calc(100vw - 16px);
                    padding: 4px 8px 4px 5px;
                    gap: 6px;
                    border-radius: 20px;
                }
                .track-cover-wrap {
                    width: 32px;
                    height: 32px;
                    min-width: 32px;
                    border-radius: 5px;
                }
                .track-cover {
                    border-radius: 5px;
                }
                .track-meta {
                    max-width: 105px;
                }
                .track-title {
                    font-size: 11.5px;
                }
                .track-artist {
                    font-size: 10px;
                }
                .audio-controls {
                    gap: 4px;
                }
                .audio-btn {
                    width: 25px;
                    height: 25px;
                    min-width: 25px;
                    font-size: 10px;
                }
            }
        `;
        document.head.appendChild(style);

        const PLAY_ICON = '<svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>';
        const PAUSE_ICON = '<svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor"><path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/></svg>';
        const NEXT_ICON = '<svg viewBox="0 0 24 24" width="12" height="12" fill="currentColor"><path d="M6 18l8.5-6L6 6v12zM16 6v12h2V6h-2z"/></svg>';

        window.__setToggleIcon = function(btn, isPlaying) {
            if (!btn) return;
            btn.innerHTML = isPlaying ? PAUSE_ICON : PLAY_ICON;
            btn.setAttribute('aria-label', isPlaying ? 'Pause' : 'Play');
        };

        const container = document.createElement('div');
        container.id = 'audio-controller';
        container.setAttribute('aria-label', 'Audio Player');
        container.innerHTML = `
            <div class="track-cover-wrap" id="cover-wrap" title="Click to play/pause">
                <img id="track-cover-img" class="track-cover" src="" alt="Album Cover" loading="eager" />
                <div class="track-cover-fallback" id="track-cover-fallback">🎵</div>
            </div>
            <div class="track-meta">
                <span class="track-title" id="track-name">Loading...</span>
                <span class="track-artist" id="track-by"></span>
            </div>
            <div class="audio-controls">
                <button class="audio-btn" id="audio-toggle" title="Play/Pause" aria-label="Play">${PLAY_ICON}</button>
                <button class="audio-btn" id="audio-next" title="Next Random Track" aria-label="Next Track">${NEXT_ICON}</button>
            </div>
        `;
        document.body.appendChild(container);

        const toggleBtn = document.getElementById('audio-toggle');
        const nextBtn = document.getElementById('audio-next');
        const coverWrap = document.getElementById('cover-wrap');

        const togglePlay = (e) => {
            if (e) e.stopPropagation();
            if (!audioElem) return;
            if (audioElem.paused) {
                audioElem.play().then(() => {
                    window.__setToggleIcon(toggleBtn, true);
                    container.classList.add('is-playing');
                    saveAudioState(true);
                }).catch(() => {});
            } else {
                audioElem.pause();
                window.__setToggleIcon(toggleBtn, false);
                container.classList.remove('is-playing');
                saveAudioState(false);
            }
        };

        toggleBtn.addEventListener('click', togglePlay);
        if (coverWrap) coverWrap.addEventListener('click', togglePlay);

        nextBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            playNext();
        });
    }

    function getCurrentSceneIndex() {
        const path = window.location.pathname;
        const idx = SCENES.findIndex(s => path.includes(s.id));
        return idx >= 0 ? idx : 0;
    }

    function navigateToScene(targetIndex) {
        saveAudioState();
        const target = SCENES[targetIndex];
        if (target) {
            document.body.classList.add('scene-switching');
            window.location.href = target.path;
        }
    }

    function navigateToPrevScene() {
        const current = getCurrentSceneIndex();
        const prev = (current - 1 + SCENES.length) % SCENES.length;
        navigateToScene(prev);
    }

    function navigateToNextScene() {
        const current = getCurrentSceneIndex();
        const next = (current + 1) % SCENES.length;
        navigateToScene(next);
    }

    function createSceneNavUI() {
        const currentIdx = getCurrentSceneIndex();
        const prevIdx = (currentIdx - 1 + SCENES.length) % SCENES.length;
        const nextIdx = (currentIdx + 1) % SCENES.length;
        const prevScene = SCENES[prevIdx];
        const nextScene = SCENES[nextIdx];

        // Subtle desktop arrows
        const prevBtn = document.createElement('button');
        prevBtn.className = 'scene-nav-arrow scene-nav-prev';
        prevBtn.title = `Previous: ${prevScene.name} (Left Arrow / Swipe Right)`;
        prevBtn.setAttribute('aria-label', `Previous: ${prevScene.name}`);
        prevBtn.innerHTML = `<svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 18 9 12 15 6"></polyline></svg>`;
        prevBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            navigateToPrevScene();
        });

        const nextBtn = document.createElement('button');
        nextBtn.className = 'scene-nav-arrow scene-nav-next';
        nextBtn.title = `Next: ${nextScene.name} (Right Arrow / Swipe Left)`;
        nextBtn.setAttribute('aria-label', `Next: ${nextScene.name}`);
        nextBtn.innerHTML = `<svg viewBox="0 0 24 24" width="22" height="22" fill="none" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 18 15 12 9 6"></polyline></svg>`;
        nextBtn.addEventListener('click', (e) => {
            e.stopPropagation();
            navigateToNextScene();
        });

        document.body.appendChild(prevBtn);
        document.body.appendChild(nextBtn);

        // Mobile swipe gestures
        let touchStartX = 0;
        let touchStartY = 0;
        let touchStartTime = 0;
        let isSwiping = false;

        window.addEventListener('touchstart', (e) => {
            if (e.touches.length !== 1) return;
            // Ignore touches on interactive buttons/widgets
            if (e.target.closest('#audio-controller') || e.target.closest('.scene-nav-arrow') || e.target.closest('button') || e.target.closest('a')) {
                return;
            }
            touchStartX = e.touches[0].clientX;
            touchStartY = e.touches[0].clientY;
            touchStartTime = Date.now();
            isSwiping = true;
        }, { passive: true });

        window.addEventListener('touchend', (e) => {
            if (!isSwiping || e.changedTouches.length === 0) return;
            isSwiping = false;

            const touchEndX = e.changedTouches[0].clientX;
            const touchEndY = e.changedTouches[0].clientY;
            const deltaX = touchEndX - touchStartX;
            const deltaY = touchEndY - touchStartY;
            const elapsed = Date.now() - touchStartTime;

            // Horizontal swipe detection: threshold 45px, predominantly horizontal, < 650ms
            if (elapsed < 650 && Math.abs(deltaX) >= 45 && Math.abs(deltaX) > Math.abs(deltaY) * 1.35) {
                if (deltaX < 0) {
                    // Swipe left -> advance to next scene
                    navigateToNextScene();
                } else {
                    // Swipe right -> return to previous scene
                    navigateToPrevScene();
                }
            }
        }, { passive: true });

        // Keyboard arrow keys for scene navigation
        window.addEventListener('keydown', (e) => {
            if (['INPUT', 'TEXTAREA'].includes(e.target.tagName)) return;
            if (e.key === 'ArrowRight') {
                navigateToNextScene();
            } else if (e.key === 'ArrowLeft') {
                navigateToPrevScene();
            }
        });
    }

    function updateTrackUI(track) {
        const titleElem = document.getElementById('track-name');
        const artistElem = document.getElementById('track-by');
        const toggleBtn = document.getElementById('audio-toggle');
        const coverImg = document.getElementById('track-cover-img');
        const coverFallback = document.getElementById('track-cover-fallback');
        const controller = document.getElementById('audio-controller');

        if (titleElem) titleElem.textContent = track.title;
        if (artistElem) artistElem.textContent = track.artist;

        if (coverImg) {
            const coverSrc = getTrackCover(track);
            coverImg.onerror = () => {
                coverImg.style.display = 'none';
                if (coverFallback) coverFallback.style.display = 'flex';
            };
            coverImg.onload = () => {
                coverImg.style.display = 'block';
                if (coverFallback) coverFallback.style.display = 'none';
            };
            coverImg.src = coverSrc;
            coverImg.alt = `${track.title} - ${track.artist}`;
        }

        const isPlaying = audioElem && !audioElem.paused;
        if (toggleBtn && window.__setToggleIcon) window.__setToggleIcon(toggleBtn, isPlaying);
        if (controller) {
            if (isPlaying) controller.classList.add('is-playing');
            else controller.classList.remove('is-playing');
        }
    }

    function playTrack(track, startPosition = 0, shouldPlay = true) {
        currentTrack = track;
        const filePath = getAudioPath(track.file);

        if (!audioElem) {
            audioElem = new Audio();
            audioElem.volume = 0.85;
            audioElem.addEventListener('ended', playNext);
            audioElem.addEventListener('play', () => {
                const btn = document.getElementById('audio-toggle');
                if (btn && window.__setToggleIcon) window.__setToggleIcon(btn, true);
                const ctrl = document.getElementById('audio-controller');
                if (ctrl) ctrl.classList.add('is-playing');
                saveAudioState(true);
            });
            audioElem.addEventListener('pause', () => {
                const btn = document.getElementById('audio-toggle');
                if (btn && window.__setToggleIcon) window.__setToggleIcon(btn, false);
                const ctrl = document.getElementById('audio-controller');
                if (ctrl) ctrl.classList.remove('is-playing');
                saveAudioState(false);
            });
            audioElem.addEventListener('timeupdate', throttledSaveState);
        }

        audioElem.src = filePath;
        updateTrackUI(track);

        let seekDone = false;
        const doSeek = () => {
            if (!seekDone && startPosition > 0) {
                try {
                    if (audioElem.duration && startPosition < audioElem.duration) {
                        audioElem.currentTime = startPosition;
                        seekDone = true;
                    } else if (!isNaN(audioElem.duration) && startPosition >= audioElem.duration) {
                        playNext();
                        return;
                    } else {
                        audioElem.currentTime = startPosition;
                        seekDone = true;
                    }
                } catch (e) {}
            }
        };

        audioElem.addEventListener('loadedmetadata', doSeek, { once: true });
        audioElem.addEventListener('canplay', doSeek, { once: true });

        if (shouldPlay) {
            const playPromise = audioElem.play();
            if (playPromise !== undefined) {
                playPromise.then(() => {
                    doSeek();
                    saveAudioState(true);
                    const btn = document.getElementById('audio-toggle');
                    if (btn && window.__setToggleIcon) window.__setToggleIcon(btn, true);
                    const ctrl = document.getElementById('audio-controller');
                    if (ctrl) ctrl.classList.add('is-playing');
                }).catch(() => {
                    // Autoplay blocked by browser policy; wait for first user gesture
                    const unlock = () => {
                        if (audioElem && audioElem.paused) {
                            doSeek();
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
        } else {
            const btn = document.getElementById('audio-toggle');
            if (btn && window.__setToggleIcon) window.__setToggleIcon(btn, false);
            const ctrl = document.getElementById('audio-controller');
            if (ctrl) ctrl.classList.remove('is-playing');
        }
    }

    function playNext() {
        const next = pickNextTrack();
        playTrack(next, 0, true);
    }

    window.initAudioPlayer = function() {
        // Guard: NEVER run or play music on landing screen (gallery portal)
        if (document.getElementById('bg-canvas') || (!document.getElementById('fractal-canvas') && !window.location.pathname.includes('-'))) {
            return;
        }

        createAudioPlayerUI();
        createSceneNavUI();

        // Check for saved playback state to continue seamlessly across scene transitions
        let state = null;
        try {
            state = JSON.parse(localStorage.getItem(STATE_KEY));
        } catch (e) {}

        if (state && state.trackId) {
            const track = PLAYLIST.find(t => t.id === state.trackId);
            if (track) {
                let resumePos = typeof state.currentTime === 'number' ? state.currentTime : 0;
                // Add elapsed transition time if music was playing
                if (state.isPlaying && state.timestamp) {
                    const elapsed = (Date.now() - state.timestamp) / 1000;
                    if (elapsed > 0 && elapsed < 8) {
                        resumePos += elapsed;
                    }
                }
                const shouldPlay = state.isPlaying !== false;
                playTrack(track, resumePos, shouldPlay);
                return;
            }
        }

        // Default initial playback: pick random track
        playNext();
    };

    if (document.readyState === 'loading') {
        window.addEventListener('DOMContentLoaded', window.initAudioPlayer);
    } else {
        window.initAudioPlayer();
    }
})();
