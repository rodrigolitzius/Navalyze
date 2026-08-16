import { Api, get_image_url } from "./api.js"

const api = new Api()

let artist_id = new URLSearchParams(window.location.search).get("id");

let artist = await api.get_artist(artist_id)
artist = await artist.json()

let artist_div = document.getElementById("image-header")

artist_div.insertAdjacentHTML("beforeend",
    `<img src="${await get_image_url(api, artist_id, 300)}">
    <div class="content">
        <h1>${artist.name}</h1>
        <div class="properties"></div>
    </div>`
)

let artist_properties = artist_div.querySelector(".properties")

let properties = [
    `${artist.album_count} albums`,
    artist.artist_type ? `${artist.artist_type}` : null,
    artist.gender ? `${artist.gender}` : null
]

for (const property of properties) {
    if (!property) { continue; }

    artist_properties.insertAdjacentHTML("beforeend",
        `<span>${property}</span>`
    )
}

let albums = document.getElementById("albums")
for (const album of artist.albums) {
    albums.insertAdjacentHTML("beforeend",
        `<div class="card">
            <img class="card-img" src=${await get_image_url(api, album.id, 400)}>
            <a href="album.html?id=${album.id}">
                <p class="card-name">${album.name}</p>
            </a>
            <p class="card-footer" title="${album.plays} plays">${album.played_hours.toFixed(2)}h</p>
        </div>`
    )
}
