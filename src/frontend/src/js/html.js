import { Chart } from "chart.js"
import { build_daily_graph, build_time_graph, build_weekly_graph } from "./util"

class GraphDataSource {
    daily() { }
    weekly() { }
    all_time() { }
}

function artist_link(id) {
    return `artist.html?id=${id}`
}

function album_link(id) {
    return `album.html?id=${id}`
}

function track_link(id) {
    return `track.html?id=${id}`
}

function playlist_link(id) {
    return `playlist.html?id=${id}`
}

function build_link_list(element, text, link) {
    element.insertAdjacentHTML("beforeend",
        `<a href="${link}">${text}</a><p>,</p>`
    )
}

function build_round_list(element, entries) {
    for (const entry of entries) {
        if (!entry) { continue; }

        element.insertAdjacentHTML("beforeend",
            `<span class="round-list-item">${entry}</span>`

        )
    }
}

function build_content_stats(element, entries) {
    for (const entry of entries) {
        element.insertAdjacentHTML("beforeend",
            `<div class="content-stats-card">
                <p class="content-stats-label">${entry.label}</p>
                <p class="content-stats-value">${entry.value}</p>
            </div>`
        )
    }
}

async function build_configurable_graph(canvas, element, data_source) {
    let chart = new Chart(canvas, {})

    element.querySelector(".graph-type-time").addEventListener("click", async () => {
        chart.destroy()

        let activity = await data_source.all_time()

        chart = build_time_graph(canvas, activity)
    })

    element.querySelector(".graph-type-daily").addEventListener("click", async () => {
        chart.destroy()

        let activity = await data_source.daily()

        chart = build_daily_graph(canvas, activity)
    })

    element.querySelector(".graph-type-weekly").addEventListener("click", async () => {
        chart.destroy()

        let activity = await data_source.weekly()

        chart = build_weekly_graph(canvas, activity)
    })

    element.querySelector("input[checked]").click()
}

export {GraphDataSource, build_link_list, build_content_stats, build_round_list, build_configurable_graph, artist_link, album_link, track_link, playlist_link}
