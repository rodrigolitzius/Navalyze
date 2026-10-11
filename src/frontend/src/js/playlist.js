import { Api, get_image_url } from "./api.js"
import { build_link_list, artist_link, track_link } from "./html.js";
import { format_hours } from "./util.js";

const api = new Api()

let playlist_id = new URLSearchParams(window.location.search).get("id");

let playlist = await api.get_playlist(playlist_id)
playlist = await playlist.json()

let playlist_div = document.getElementById("image-header")

playlist_div.insertAdjacentHTML("beforeend",
    `<img src="${await get_image_url(api, playlist_id, 700)}">
    <div class="content">
        <h1>${playlist.name}</h1>
        <div id="artists" class="link-list"></div>
        <div id="properties" class="round-list"></div>
    </div>`
)

let playlist_content = playlist_div.querySelector("#properties")

let properties = [
    playlist.year ? `${playlist.year}` : null,
]

for (const property of properties) {
    if (!property) { continue; }

    playlist_content.insertAdjacentHTML("beforeend",
        `<span class="round-list-item">${property}</span>`
    )
}

let max = Math.max(...playlist.tracks.map(item => item.played_hours))

let track_list = document.getElementById("track-list")

for (const track of playlist.tracks) {
    track_list.insertAdjacentHTML("beforeend",
        `<div class="track">
            <div class="left">
                <a class="track-title" href="${track_link(track.id)}">${track.name}</a>
                <p class="track-time" title="${track.plays} plays">${format_hours(track.played_hours)}</p>
            </div>
            <span class="track-bar"</span>
        </div>`
    )

    track_list.querySelector(".track:last-of-type").style.setProperty("--ratio", track.played_hours/max)
}
