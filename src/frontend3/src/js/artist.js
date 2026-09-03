import { Api, get_image_url } from "./api.js"
import { album_link } from "./html.js";

const api = new Api()

let artist_id = new URLSearchParams(window.location.search).get("id");

let artist = await api.get_artist(artist_id)
artist = await artist.json()

let artist_div = document.getElementById("image-header")

artist_div.insertAdjacentHTML("beforeend",
    `<img src="${await get_image_url(api, artist_id, 300)}">
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
        `<div class="card">
            <img class="card-img" src=${await get_image_url(api, album.id, 400)}>
            <a class="card-name" href="${album_link(album.id)}">${album.name}</a>
            <p class="card-footer" title="${album.plays} plays">${album.played_hours.toFixed(2)}h</p>
        </div>`
    )
}
