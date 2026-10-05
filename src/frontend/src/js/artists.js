import { Api, get_image_url } from "./api.js"
import { build_link_list, artist_link, album_link, track_link } from "./html.js";

let api = new Api()

const PAGE_SIZE = 20

let page = new URLSearchParams(window.location.search).get("page");
page = page ? parseInt(page, 10) : 0
if (Number.isNaN(page) || page < 0) page = 0

document.getElementById("next-page").addEventListener("click", function() {
    window.location.replace(`artists.html?page=${page+1}`)
});

document.getElementById("previous-page").addEventListener("click", function () {
    let next = page-1
    if (page <= 0) {
        next = 0
    }
    window.location.replace(`artists.html?page=${next}`)
});

let artists = await api.get_most_played_artists(PAGE_SIZE, PAGE_SIZE*page)
artists = await artists.json()

let artist_list = document.getElementById("artists-list")

for (const artist of artists) {
    artist_list.insertAdjacentHTML("beforeend",
        `<a href="artist.html?id=${artist.id}" class="album-entry">
            <div class="card album-entry">
                <img class="card-img" src="${await get_image_url(api, artist.id, 400)}">
                <p class="card-name">${artist.name}</p>
                <p class="card-footer" title="${artist.plays}">${artist.played_hours.toFixed(2)}h</p>
            </div>
        </a>`
    )
}
