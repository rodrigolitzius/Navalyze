import { Api, get_image_url } from "./api.js"

const api = new Api()

let page = new URLSearchParams(window.location.search).get("page");

if (!page) {
    page = 0
} else {
    page = parseInt(page, 10)
}

document.getElementById("next-page").addEventListener("click", function() {
    window.location.replace(`recent.html?page=${page+1}`)
});

document.getElementById("previous-page").addEventListener("click", function () {
    let next = page-1
    if (page <= 0) {
        next = 0
    }
    window.location.replace(`recent.html?page=${next}`)
});

let recent = await api.get_recently_played(30, 30*page)
recent = await recent.json()

let recent_html = document.querySelector("main")
for (const song of recent) {
    recent_html.insertAdjacentHTML("beforeend",
        `<div class="song-entry">
            <img class="song-img" src=${await get_image_url(api, song.id, 400)}>
            <div class="song-contents">
                <p class="song-title">${song.title}</p>
                <p class="song-artist">${song.artist}</p>
                <a class="song-album" href="album.html?id=${song.album_id}">
                    <p>${song.album}</p>
                </a>
            </div>
        </div>`
    )
}
