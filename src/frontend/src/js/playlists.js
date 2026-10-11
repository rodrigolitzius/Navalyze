import { Api, get_image_url } from "./api.js"

let api = new Api()

const PAGE_SIZE = 20

let page = new URLSearchParams(window.location.search).get("page");
page = page ? parseInt(page, 10) : 0
if (Number.isNaN(page) || page < 0) page = 0

document.getElementById("next-page").addEventListener("click", function() {
    window.location.replace(`playlists.html?page=${page+1}`)
});

document.getElementById("previous-page").addEventListener("click", function () {
    let next = page-1
    if (page <= 0) {
        next = 0
    }
    window.location.replace(`playlists.html?page=${next}`)
});

let playlists = await api.get_most_played_playlists(PAGE_SIZE, PAGE_SIZE*page)
playlists = await playlists.json()

let playlist_list = document.getElementById("playlists-list")

for (const playlist of playlists) {
    playlist_list.insertAdjacentHTML("beforeend",
        `<a href="playlist.html?id=${playlist.id}" class="playlist-entry">
            <div class="card album-entry">
                <img class="card-img" src="${await get_image_url(api, playlist.id, 400)}">
                <p class="card-name">${playlist.name}</p>
                <p class="card-footer" title="${playlist.plays}">${playlist.played_hours.toFixed(2)}h</p>
            </div>
        </a>`
    )
}
