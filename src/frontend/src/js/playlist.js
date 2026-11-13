import { Api } from "./api.js"
import { build_content_stats, track_link } from "./html.js";
import { format_hours, build_time_graph, load_images } from "./util.js";
import { CeriousScroll } from '@ceriousdevtech/cerious-scroll';

const api = new Api()

let playlist_id = new URLSearchParams(window.location.search).get("id");

let playlist = await api.get_playlist(playlist_id)
playlist = await playlist.json()

let header = document.getElementById("img-header")
load_images(api, [{ id: playlist_id, size: 700, container: header }])

let content = header.querySelector(".content")

content.querySelector(".title").textContent = playlist.name

build_content_stats(content.querySelector(".content-stats"), [
    { label: "Plays", value: `${playlist.plays}` },
    { label: "Time played", value: format_hours(playlist.played_hours) },
])

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
