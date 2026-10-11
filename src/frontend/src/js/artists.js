import { Api, get_image_url } from "./api.js"
import { load_nav_buttons, get_page, PAGE_SIZE } from "./load-page-change.js"
import { format_hours } from "./util.js";

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "artists.html")
let page = get_page()

let artists = await api.get_most_played_artists(PAGE_SIZE, PAGE_SIZE*page)
artists = await artists.json()

let artist_list = document.getElementById("artists-list")

for (const artist of artists) {
    artist_list.insertAdjacentHTML("beforeend",
        `<a href="artist.html?id=${artist.id}" class="album-entry">
            <div class="card album-entry">
                <img class="card-img" src="${await get_image_url(api, artist.id, 400)}">
                <p class="card-name">${artist.name}</p>
                <p class="card-footer" title="${artist.plays}">${format_hours(artist.played_hours)}</p>
            </div>
        </a>`
    )
}
