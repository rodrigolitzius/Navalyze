import { Api, get_image_url } from "./api.js"

const api = new Api()

let album_id = new URLSearchParams(window.location.search).get("id");

let album = await api.get_album(album_id)
album = await album.json()

let album_div = document.getElementById("image-header")

album_div.insertAdjacentHTML("beforeend",
    `<img src="${await get_image_url(api, album_id, 1000)}">
    <div class="content">
        <h1>${album.name}</h1>
        <div class="properties"></div>
    </div>`
)

let album_content = album_div.querySelector(".properties")

let properties = [
    album.year ? `${album.year}` : null,
]

for (const property of properties) {
    if (!property) { continue; }

    album_content.insertAdjacentHTML("beforeend",
        `<span>${property}</span>`
    )
}

console.log(album)

let max = Math.max(...album.tracks.map(item => item.played_hours))

let track_list = document.getElementById("track-list")

for (const track of album.tracks) {
    track_list.insertAdjacentHTML("beforeend",
        `<div class="track">
            <div class="left">
                <span class="track-title">${track.name}</span>
                <p class="track-time" title="${track.plays} plays">${track.played_hours.toFixed(2)}h</p>
            </div>
            <span class="track-bar"</span>
        </div>`
    )

    track_list.querySelector(".track:last-of-type").style.setProperty("--ratio", track.played_hours/max)
}
