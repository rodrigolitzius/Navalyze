import { Api } from "./api.js"
import { track_link } from "./html.js";
import { format_hours, build_time_graph, load_images } from "./util.js";
import { CeriousScroll } from '@ceriousdevtech/cerious-scroll';

const api = new Api()

let playlist_id = new URLSearchParams(window.location.search).get("id");

let playlist = await api.get_playlist(playlist_id)
playlist = await playlist.json()

let playlist_div = document.getElementById("image-header")

playlist_div.insertAdjacentHTML("beforeend",
    `<img>
    <div class="content">
        <h1>${playlist.name}</h1>
        <div id="artists" class="link-list"></div>
        <div id="properties" class="round-list"></div>
    </div>`
)

load_images(api, [{id: playlist_id, size: 700, img: playlist_div.querySelector("img")}])

let playlist_content = playlist_div.querySelector("#properties")

let properties = [
    playlist.year ? `${playlist.year}` : null,
]

for (const property of properties) {
    if (!property) { continue; }

    playlist_content.insertAdjacentHTML("beforeend",
        `<span class="round-list-item">${property}</span>`
    )
}

let playlist_activity = await api.get_playlist_graph(playlist_id, 50)
playlist_activity = await playlist_activity.json()

build_time_graph(document.getElementById("graph-playlist"), {[playlist.name]: playlist_activity})

let track_list = document.getElementById("track-list")
let max = Math.max(...playlist.tracks.map(item => item.played_hours))

const data = playlist.tracks.map((element, index) => ({
    id: element.id,
    name: element.name,
    plays: element.plays,
    played_hours: element.played_hours,
    ratio: element.played_hours/max
}))

console.log(data)

function render() {
    scroller.renderViewport(
        track_list.clientHeight,
        track_list,
        (index, element) => {
            element.innerHTML =
            `<div class="track" style="--ratio: ${data[index].ratio};">
                <div class="left">
                    <a class="track-title" href="${track_link(data[index].id)}">${data[index].name}</a>
                    <p class="track-time" title="${data[index].plays} plays">${format_hours(data[index].played_hours)}</p>
                </div>
                <span class="track-bar"</span>
            </div>`;
        }
    )
}

let scroller = new CeriousScroll(track_list, playlist.tracks.length, {onScroll: render})
render()
