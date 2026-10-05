import { Api, get_image_url } from "./api.js"
import { build_link_list, artist_link, album_link, track_link } from "./html.js";

let api = new Api()

const PAGE_SIZE = 20

let page = new URLSearchParams(window.location.search).get("page");
page = page ? parseInt(page, 10) : 0
if (Number.isNaN(page) || page < 0) page = 0

document.getElementById("next-page").addEventListener("click", function() {
    window.location.replace(`albums.html?page=${page+1}`)
});

document.getElementById("previous-page").addEventListener("click", function () {
    let next = page-1
    if (page <= 0) {
        next = 0
    }
    window.location.replace(`albums.html?page=${next}`)
});

let albums = await api.get_most_played_albums(PAGE_SIZE, PAGE_SIZE*page)
albums = await albums.json()

let album_list = document.getElementById("albums-list")

for (const album of albums) {
    album_list.insertAdjacentHTML("beforeend",
        `<a href="album.html?id=${album.id}" class="album-entry">
            <div class="card album-entry">
                <img class="card-img" src="${await get_image_url(api, album.id, 400)}">
                <p class="card-name">${album.name}</p>
                <p class="card-footer" title="${album.plays}">${album.played_hours.toFixed(2)}h</p>
            </div>
        </a>`
    )
}
