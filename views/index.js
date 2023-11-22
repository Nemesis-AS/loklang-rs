window.onload = main;

function main() {
    const mainEl = document.querySelector("main");
    const app = new App(mainEl);
    app.renderPage();
}

class App {
    constructor(mainEl) {
        this.page = "songs";
        this.targetEl = mainEl;
        this.currentSong = "";
        
        this.player = null;

        this.playlist = [];
        this.queue = [];

        this.sliderEl = null;
        this.songImgEl = null;
        this.songTitleEl = null;
        this.songArtistEl = null;

        this.initializePlayer();
        this.addEventListeners();
    }

    initializePlayer() {
        this.player = new Audio();
        this.sliderEl = document.getElementById("seekBar");
        this.songImgEl = document.getElementById("songImg");
        this.songTitleEl = document.getElementById("songTitleEl");
        this.songArtistEl = document.getElementById("songArtistEl");
        const playBtn = document.getElementById("playBtn");
        const currDuration = document.getElementById("currDuration");
        const totalDuration = document.getElementById("totalDuration");

        this.player.addEventListener("timeupdate", e => {
            const value = (e.target.currentTime / e.target.duration) * 100;
            this.sliderEl.value = value;
            currDuration.textContent = this.formatTime(e.target.currentTime);
        });

        this.player.addEventListener("durationchange", e => {
            totalDuration.textContent = this.formatTime(e.target.duration);
        });

        this.sliderEl.addEventListener("input", e => {
           if (!this.player.duration) return;

            const value = (e.target.value / 100) * this.player.duration;
            this.player.currentTime = value;
        });

        playBtn.addEventListener("click", e => {
            if (!this.player.src) return;

            if (this.player.paused) {
                this.player.play();
                e.target.classList.remove("paused");
            } else {
                this.player.pause();
                e.target.classList.add("paused");
            }
        });
    }

    addEventListeners() {
        document.querySelectorAll("button.nav-link").forEach(btn => {
            btn.addEventListener("click", e => {
                this.setPage(e.target.dataset.target);
            });
        });
    }
    
    async playMedia(id) {
        this.player.src = `/api/v1/stream/${id}`;
        this.player.play();

        // @todo! Update the play bar
        const songRes = await fetch(`/api/v1/songs/${id}`);
        const songInfo = await songRes.json(); // @temp

        const imgRes = await fetch(`/api/v1/picture/${songInfo[0].pictures[0].id}`);
        const img = await imgRes.text();

        this.songTitleEl.textContent = songInfo[0].title || "<Title>";
        this.songArtistEl.textContent = (songInfo[0].artists && songInfo[0].artists.join(", ")) || "<Artists>";
        this.songImgEl.src = img;
    }

    async renderPage() {
        switch (this.page) {
            case "home":
                break;
            case "songs":
                const songRes = await fetch("/api/v1/songs");
                const songs = await songRes.json();
                this.renderSongs(songs);
                break;
            case "artists":
                const artistRes = await fetch("/api/v1/artists");
                const artists = await artistRes.json();
                this.renderArtists(artists);
                break;
            case "albums":
                const albumRes = await fetch("/api/v1/albums");
                const albums = await albumRes.json();
                this.renderAlbums(albums);
                break;
        }

        this.targetEl.dataset.page = this.page;

        document.querySelectorAll("button.nav-link").forEach(link => {
            if (link.dataset.target === this.page) {
                link.classList.add("active");
            } else {
                link.classList.remove("active");
            }
        });
    }

    renderSongs(songs) {
        this.clearChildren(this.targetEl);
        for (let idx in songs) {
            const song = songs[idx];
            const songDiv = this.elWithClasses("div", ["song"]);
    
            const iconsDiv = this.elWithClasses("div", ["icons"]);
    
            const input = this.elWithClasses("input");
            input.setAttribute("type", "checkbox");
            iconsDiv.appendChild(input);
    
            const playBtn = this.elWithClasses("button", ["play-btn"]);
            playBtn.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="icon"><path fill-rule="evenodd" d="M4.5 5.653c0-1.426 1.529-2.33 2.779-1.643l11.54 6.348c1.295.712 1.295 2.573 0 3.285L7.28 19.991c-1.25.687-2.779-.217-2.779-1.643V5.653z" clip-rule="evenodd" /></svg>`;
            playBtn.addEventListener("click", e => {
                this.playMedia(song.id);
                document.getElementById("playBtn").classList.remove("paused");
            });
            iconsDiv.appendChild(playBtn);
    
            songDiv.appendChild(iconsDiv);
    
            const songName = this.elWithClasses("span", ["song-name"], song.title || "<Title>");
            const artistName = this.elWithClasses("span", ["song-artist"], (song.artists && song.artists.join(", ")) || "<Artists>");
            const albumName = this.elWithClasses("span", ["song-album"], song.album || "<Album>");
            const duration = this.elWithClasses("span", ["song-duration"], this.formatTime(song.duration) || "<0:00>");
            songDiv.appendChild(songName);
            songDiv.appendChild(artistName);
            songDiv.appendChild(albumName);
            songDiv.appendChild(duration);
    
            this.targetEl.appendChild(songDiv);
        }
    }

    renderArtists(artists) {
        this.clearChildren(this.targetEl);
        for (let i = 0; i < 5; i++) { 
        for (let idx in artists) {
            const artist = artists[idx];
            const artistDiv = this.elWithClasses("button", ["artist"]);
    
            // const iconsDiv = this.elWithClasses("div", ["icons"]);
    
            // const input = this.elWithClasses("input");
            // input.setAttribute("type", "checkbox");
            // iconsDiv.appendChild(input);

            // const playBtn = this.elWithClasses("button", ["play-btn"], "Play");
            // iconsDiv.appendChild(playBtn);
    
            // artistDiv.appendChild(iconsDiv);
    
            const artistName = this.elWithClasses("span", ["artist-name"], artist || "<Name>");
            // const artistName = this.elWithClasses("span", ["song-artist"], (song.artists && song.artists.join(", ")) || "<Artists>");
            // const albumName = this.elWithClasses("span", ["song-album"], song.album || "<Album>");
            // const duration = this.elWithClasses("span", ["song-duration"], song.duration || "<0:00>");
            artistDiv.appendChild(artistName);
            // artistDiv.appendChild(artistName);
            // artistDiv.appendChild(albumName);
            // songDiv.appendChild(duration);
    
            this.targetEl.appendChild(artistDiv);
        }}
    }

    renderAlbums(albums) {
        this.clearChildren(this.targetEl);
        for (let idx in albums) {
            const album = albums[idx];
            const albumDiv = this.elWithClasses("div", ["album"]);
    
            // const iconsDiv = this.elWithClasses("div", ["icons"]);
    
            // const input = this.elWithClasses("input");
            // input.setAttribute("type", "checkbox");
            // iconsDiv.appendChild(input);
    
            // const playBtn = this.elWithClasses("button", ["play-btn"], "Play");
            // iconsDiv.appendChild(playBtn);
    
            // albumDiv.appendChild(iconsDiv);
    
            const albumName = this.elWithClasses("span", ["album-name"], album || "<Name>");
            // const artistName = this.elWithClasses("span", ["song-artist"], (song.artists && song.artists.join(", ")) || "<Artists>");
            // const albumName = this.elWithClasses("span", ["song-album"], song.album || "<Album>");
            // const duration = this.elWithClasses("span", ["song-duration"], song.duration || "<0:00>");
            albumDiv.appendChild(albumName);
            // artistDiv.appendChild(artistName);
            // artistDiv.appendChild(albumName);
            // songDiv.appendChild(duration);
    
            this.targetEl.appendChild(albumDiv);
        }
    }

    setPage(page) {
        this.page = page;
        this.renderPage();
    }

    elWithClasses(elementName, classes, text = null) {
        const el = document.createElement(elementName);
        if (classes) el.classList.add(...classes);
        if (text !== null) el.appendChild(document.createTextNode(text));
    
        return el;
    }

    clearChildren(el) {
        el.textContent = "";
    }

    formatTime(value) {
        const mins = String(Math.floor(value / 60)).padStart(1, "0");
        const secs = String(Math.floor(value % 60)).padStart(2, "0");

        return `${mins}:${secs}`;
    }
}