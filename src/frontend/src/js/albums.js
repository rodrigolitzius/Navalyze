import { Api, get_image_url } from "./api.js"
import { load_nav_buttons, get_page, PAGE_SIZE } from "./load-page-change.js"
import { format_hours } from "./util.js";

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "albums.html")
let page = get_page()

let albums = await api.get_most_played_albums(PAGE_SIZE, PAGE_SIZE*page)
albums = await albums.json()

let album_list = document.getElementById("albums-list")

for (const album of albums) {
    album_list.insertAdjacentHTML("beforeend",
        `<a href="album.html?id=${album.id}" class="album-entry">
            <div class="card album-entry">
                <img class="card-img" src="${await get_image_url(api, album.id, 400)}">
                <p class="card-name">${album.name}</p>
                <p class="card-footer" title="${album.plays}">${format_hours(album.played_hours)}</p>
            </div>
        </a>`
    )
}
