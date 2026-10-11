import { Api, get_image_url } from "./api.js"
import { build_link_list, artist_link, album_link, track_link } from "./html.js";
import { load_nav_buttons, get_page, PAGE_SIZE } from "./load-page-change.js"

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "recent.html")
let page = get_page()

let recent = await api.get_recently_played(PAGE_SIZE, PAGE_SIZE*page)
recent = await recent.json()

let recent_html = document.querySelector("#played-list")
for (const song of recent) {
    let played_date = new Date(song.timestamp * 1000)
    let played_date_string = played_date.toLocaleString("pt-br", { timeZone: localStorage.getItem("timezone") || undefined })

    recent_html.insertAdjacentHTML("beforeend",
        `<a href="${track_link(song.id)}">
            <div class="track-entry">
                <img class="track-img" src=${await get_image_url(api, song.id, 400)}>
                <div class="track-contents">
                    <p class="track-title">${song.title}</p>
                    <div class="track-artist link-list"></div>
                    <div class="track-album link-list"></div>
                    <div class="track-time">
                        <p>${played_date_string}</p>
                    </div>
                </div>
            </div>
        </a>`
    )

    for (const artist of song.artists) {
        build_link_list(
            recent_html.lastElementChild.querySelector(".track-artist"),
            artist.name,
            artist_link(artist.id),
        )
    }

    build_link_list(
        recent_html.lastElementChild.querySelector(".track-album"),
        song.album,
        album_link(song.album_id),
    )
}
