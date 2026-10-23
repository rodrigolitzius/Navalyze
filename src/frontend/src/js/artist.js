import { Temporal } from "temporal-polyfill";
import { Api, get_image_url } from "./api.js"
import { album_link } from "./html.js";
import { format_hours, build_time_graph } from "./util.js";

const api = new Api()

let artist_id = new URLSearchParams(window.location.search).get("id");

let artist = await api.get_artist(artist_id)
artist = await artist.json()

let artist_div = document.getElementById("image-header")

artist_div.insertAdjacentHTML("beforeend",
    `<img src="${await get_image_url(api, artist_id, 700)}">
    <div class="content">
        <h1>${artist.name}</h1>
        <div class="round-list"></div>
    </div>`
)

let artist_properties = artist_div.querySelector(".round-list")

let properties = [
    `${artist.album_count} albums`,
    artist.artist_type ? `${artist.artist_type}` : null,
    artist.gender ? `${artist.gender}` : null
]

for (const property of properties) {
    if (!property) { continue; }

    artist_properties.insertAdjacentHTML("beforeend",
        `<span class="round-list-item">${property}</span>`
    )
}

let albums = document.getElementById("albums")
for (const album of artist.albums) {
    albums.insertAdjacentHTML("beforeend",
        `<a href="${album_link(album.id)}" class="card">
            <div>
                <img class="card-img" src=${await get_image_url(api, album.id, 400)}>
                <p class="card-name">${album.name}</p>
                <p class="card-footer" title="${album.plays} plays">${format_hours(album.played_hours)}</p>
            </div>
        </a>`
    )
}

let artist_activity = await api.get_artist_graph(artist_id, 50)
artist_activity = await artist_activity.json()

build_time_graph(document.getElementById("graph-artist"), {[artist.name]: artist_activity})
