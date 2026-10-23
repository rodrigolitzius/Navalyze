import { Chart } from 'chart.js/auto';
import { Api } from "./api"
import { build_time_graph } from "./util"
import autocolors from 'chartjs-plugin-autocolors';

let api = new Api()

let doughnut = document.getElementById("languages")
    .querySelector(".graph-div")
    .querySelector(".doughnut")
    .querySelector("canvas")
let doughnut_api_data = await api.get_languages(5)
doughnut_api_data = await doughnut_api_data.json()

let data = new Array

new Chart(doughnut, {
    type: 'doughnut',
    data: {
        labels: Object.keys(doughnut_api_data),
        datasets: [{
            data: Object.values(doughnut_api_data)
        }]
    },
    options: {
        maintainAspectRatio: false,
        radius: "100%",
    }
})

let line = document.getElementById("languages")
    .querySelector(".graph-div")
    .querySelector(".line")
    .querySelector("canvas")

let line_api_data = await api.get_languages_graph(50, 5)
line_api_data = await line_api_data.json()

build_time_graph(line, line_api_data)
