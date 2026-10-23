import { Chart } from 'chart.js/auto';
import { Api } from "./api"
import { build_time_graph } from "./util"

let api = new Api()

function build_doughnut_graph(ctx, data) {
    new Chart(ctx, {
        type: 'doughnut',
        data: {
            labels: Object.keys(data),
            datasets: [{
                data: Object.values(data)
            }]
        },
        options: {
            maintainAspectRatio: false,
            radius: "100%",
        }
    })
}

async function languages_graphs(graph_div) {
    let doughnut = graph_div
        .querySelector(".doughnut")
        .querySelector("canvas")
    let doughnut_api_data = await api.get_languages(5)
    doughnut_api_data = await doughnut_api_data.json()

    build_doughnut_graph(doughnut, doughnut_api_data)

    let line = graph_div
        .querySelector(".line")
        .querySelector("canvas")

    let line_api_data = await api.get_languages_graph(50, 5)
    line_api_data = await line_api_data.json()

    build_time_graph(line, line_api_data)
}

async function genres_graphs(graph_div) {
    let doughnut = graph_div
        .querySelector(".doughnut")
        .querySelector("canvas")
    let genres = await api.get_most_played_genres(5)
    genres = await genres.json()

    let doughnut_api_data = {}
    for (const genre of genres) {
        doughnut_api_data[`${genre.name}`] = genre.played_hours
    }

    build_doughnut_graph(doughnut, doughnut_api_data)

    let line = graph_div
        .querySelector(".line")
        .querySelector("canvas")

    let line_api_data = await api.get_genres_graph(50, 5)
    line_api_data = await line_api_data.json()

    build_time_graph(line, line_api_data)
}

async function release_types_graphs(graph_div) {
    let doughnut = graph_div
        .querySelector(".doughnut")
        .querySelector("canvas")
    let doughnut_api_data = await api.get_release_types(5)
    doughnut_api_data = await doughnut_api_data.json()

    build_doughnut_graph(doughnut, doughnut_api_data)

    let line = graph_div
        .querySelector(".line")
        .querySelector("canvas")

    let line_api_data = await api.get_release_types_graph(50, 5)
    line_api_data = await line_api_data.json()

    build_time_graph(line, line_api_data)
}

languages_graphs(document.getElementById("languages").querySelector(".graph-div"))
genres_graphs(document.getElementById("genres").querySelector(".graph-div"))
release_types_graphs(document.getElementById("release-types").querySelector(".graph-div"))
