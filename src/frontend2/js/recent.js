import { Api, get_image_url } from "./api.js"

const api = new Api()

let page = new URLSearchParams(window.location.search).get("page");

if (!page) {
    page = 0
}

let recent = await api.get_recently_played(30, 30*page)
recent = await recent.json()

let recent_html = document.querySelector("main")
for (const song of recent) {
    recent_html.insertAdjacentHTML("beforeend",
        `<div class="card">
            <img class="card-img" src=${await get_image_url(api, song.id, 400)}>
            <div class="card-contents">
                <p class="card-name">${song.title}</p>
                <p class="card-item">${song.artist}</p>
                <a href="album.html?id=${song.album_id}">
                    <p class="card-item">${song.album}</p>
                </a>
            </div>
        </div>`
    )
}
