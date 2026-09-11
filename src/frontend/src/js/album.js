import { Api, get_image_url } from "./api.js"
import { build_link_list, artist_link, track_link } from "./html.js";

const api = new Api()

let album_id = new URLSearchParams(window.location.search).get("id");

let album = await api.get_album(album_id)
album = await album.json()

let album_div = document.getElementById("image-header")

album_div.insertAdjacentHTML("beforeend",
    `<img src="${await get_image_url(api, album_id, 1000)}">
    <div class="content">
        <h1>${album.name}</h1>
        <div id="artists" class="link-list"></div>
        <div id="properties" class="round-list"></div>
    </div>`
)

for (const artist of album.artists) {
    build_link_list(album_div.querySelector("#artists"), artist.name, artist_link(artist.id))
}

let album_content = album_div.querySelector("#properties")

let properties = [
    album.year ? `${album.year}` : null,
]

for (const property of properties) {
    if (!property) { continue; }

    album_content.insertAdjacentHTML("beforeend",
        `<span class="round-list-item">${property}</span>`
    )
}

let max = Math.max(...album.tracks.map(item => item.played_hours))

let track_list = document.getElementById("track-list")

for (const track of album.tracks) {
    track_list.insertAdjacentHTML("beforeend",
        `<div class="track">
            <div class="left">
                <a class="track-title" href="${track_link(track.id)}">${track.name}</a>
                <p class="track-time" title="${track.plays} plays">${track.played_hours.toFixed(2)}h</p>
            </div>
            <span class="track-bar"</span>
        </div>`
    )

    track_list.querySelector(".track:last-of-type").style.setProperty("--ratio", track.played_hours/max)
}