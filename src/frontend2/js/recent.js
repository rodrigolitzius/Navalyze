import { Api, get_image_url } from "./api.js"
import { build_link_list, artist_link, album_link } from "./html.js";

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
                <div class="song-artist link-list"></div>
                <div class="song-album link-list"></div>
            </div>
        </div>`
    )

    for (const artist of song.artists) {
        build_link_list(
            recent_html.lastElementChild.querySelector(".song-artist"),
            artist.name,
            artist_link(artist.id),
        )
    }

    build_link_list(
        recent_html.lastElementChild.querySelector(".song-album"),
        song.album,
        album_link(song.album_id),
    )
}
