import { Api, get_image_url } from "./api.js"
import { format_hours } from "./util.js"
import { build_link_list, album_link, track_link } from "./html.js"
import { load_nav_buttons, get_page, PAGE_SIZE } from "./load-page-change.js"

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "tracks.html")
let page = get_page()

let tracks_response = await api.get_most_played_tracks(PAGE_SIZE, PAGE_SIZE * page)
let tracks = await tracks_response.json()

let list = document.getElementById("tracks-list")
for (let i = 0; i < tracks.length; i++) {
    const track = tracks[i]

    let image_url = null
    try {
        image_url = await get_image_url(api, track.id, 400)
    } catch {
        image_url = null
    }

    list.insertAdjacentHTML("beforeend",
        `<a href="${track_link(track.id)}">
            <div class="track-entry">
                ${image_url ? `<img class="track-img" src="${image_url}">` : `<div class="track-img"></div>`}
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

    let entry = list.lastElementChild

    entry.querySelector(".track-artist").insertAdjacentHTML("beforeend", `<p>${track.artist}</p>`)

    build_link_list(entry.querySelector(".track-album"), track.album, album_link(track.album_id))
}
