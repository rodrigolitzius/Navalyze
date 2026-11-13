import { Api, get_image_url } from "./api.js"
import { format_hours, load_images } from "./util.js"
import { build_link_list, album_link, track_link } from "./html.js"
import { load_nav_buttons, get_page } from "./load-page-change.js"

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "tracks.html")
let [page_start, page_end] = get_page()

let artist_id = new URLSearchParams(window.location.search).get("artist");

let tracks = null

if (artist_id) {
    tracks = await api.get_artist(artist_id, "tracks", page_end-page_start, page_start)
    tracks = await tracks.json()
    tracks = tracks.tracks
} else {
    tracks = await api.get_most_played_tracks(page_end-page_start, page_start)
    tracks = await tracks.json()
}

let list = document.getElementById("tracks-list")
let images = new Array()
for (let i = 0; i < tracks.length; i++) {
    const track = tracks[i]

    list.insertAdjacentHTML("beforeend",
        `<a href="${track_link(track.id)}">
            <div class="track-entry">
                <img class="track-img">
                <div class="track-contents">
                    <p class="track-title">${track.name}</p>
                    <div class="track-artist link-list"></div>
                    <div class="track-album link-list"></div>
                    <div class="track-time">
                        <p>${format_hours(track.played_hours)} · ${track.plays} plays</p>
                    </div>
                </div>
            </div>
        </a>
        `
    )

    images.push({id: track.id, size: 400, container: list.lastElementChild})

    let entry = list.lastElementChild

    entry.querySelector(".track-artist").insertAdjacentHTML("beforeend", `<p>${track.artist}</p>`)

    build_link_list(entry.querySelector(".track-album"), track.album, album_link(track.album_id))
}

load_images(api, images)
