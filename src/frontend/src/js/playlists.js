import { Api } from "./api.js"
import { playlist_link } from "./html.js";
import { load_nav_buttons, get_page } from "./load-page-change.js"
import { format_hours, load_images } from "./util.js";

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "playlists.html")
let [page_start, page_end] = get_page()

let playlists = await api.get_most_played_playlists(page_end-page_start, page_start)
playlists = await playlists.json()

let playlist_list = document.getElementById("playlists-list")
let images = new Array()

for (const playlist of playlists) {
    playlist_list.insertAdjacentHTML("beforeend",
        `<a href="${playlist_link(playlist.id)}" class="playlist-entry">
            <div class="card playlist-entry">
                <div class="img-container">
                    <img class="card-img">
                </div>
                <p class="card-name">${playlist.name}</p>
                <p class="card-footer" title="${playlist.plays} plays">${format_hours(playlist.played_hours)}</p>
            </div>
        </a>`
    )

    images.push({id: playlist.id, size: 400, container: playlist_list.lastElementChild})
}

load_images(api, images)
