import { Api } from "./api.js"
import { load_nav_buttons, get_page } from "./load-page-change.js"
import { format_hours, load_images } from "./util.js";

let api = new Api()

load_nav_buttons(document.querySelector("#nav-buttons"), "artists.html")
let [page_start, page_end] = get_page()

let artists = await api.get_most_played_artists(page_end-page_start, page_start)
artists = await artists.json()

let artist_list = document.getElementById("artists-list")
let images = new Array()

for (const artist of artists) {
    artist_list.insertAdjacentHTML("beforeend",
        `<a href="artist.html?id=${artist.id}" class="album-entry">
            <div class="card album-entry">
                <img class="card-img">
                <p class="card-name">${artist.name}</p>
                <p class="card-footer" title="${artist.plays}">${format_hours(artist.played_hours)}</p>
            </div>
        </a>`
    )

    images.push({id: artist.id, size: 400, img: artist_list.lastElementChild.querySelector("img")})
}

load_images(api, images)
