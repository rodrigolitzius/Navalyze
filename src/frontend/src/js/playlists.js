import { Api } from "./api.js"
import { load_nav_buttons, get_page, PAGE_SIZE } from "./load-page-change.js"
import { format_hours, load_images } from "./util.js";

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "playlists.html")
let page = get_page()

let playlists = await api.get_most_played_playlists(PAGE_SIZE, PAGE_SIZE*page)
playlists = await playlists.json()

let playlist_list = document.getElementById("playlists-list")
let images = new Array()

for (const playlist of playlists) {
    playlist_list.insertAdjacentHTML("beforeend",
        `<a href="playlist.html?id=${playlist.id}" class="playlist-entry">
            <div class="card album-entry">
                <img class="card-img">
                <p class="card-name">${playlist.name}</p>
                <p class="card-footer" title="${playlist.plays}">${format_hours(playlist.played_hours)}</p>
            </div>
        </a>`
    )

    images.push({id: playlist.id, size: 400, img: playlist_list.lastElementChild.querySelector("img")})
}

load_images(api, images)
