import { Api } from "./api.js"
import { album_link } from "./html.js";
import { load_nav_buttons, get_page } from "./load-page-change.js"
import { format_hours, load_images, } from "./util.js";

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "albums.html")
let [page_start, page_end] = get_page()

let albums = await api.get_most_played_albums(page_end-page_start, page_start)
albums = await albums.json()

let album_list = document.getElementById("albums-list")
let images = new Array()

for (const album of albums) {
    album_list.insertAdjacentHTML("beforeend",
        `<a href="${album_link(album.id)}" class="album-entry">
            <div class="card album-entry">
                <div class="img-container">
                    <img class="card-img">
                </div>
                <p class="card-name">${album.name}</p>
                <p class="card-footer" title="${album.plays} plays">${format_hours(album.played_hours)}</p>
            </div>
        </a>`
    )

    images.push({id: album.id, size: 400, container: album_list.lastElementChild})
}

load_images(api, images)
