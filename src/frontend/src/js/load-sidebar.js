import { Temporal } from 'temporal-polyfill'

const page = document.getElementById("layout")

let sidebar_html = await fetch("sidebar.html")
sidebar_html = await sidebar_html.text()

let popover_filter = await fetch("filter-popover.html")
popover_filter = await popover_filter.text()

page.insertAdjacentHTML("afterbegin", sidebar_html)
page.insertAdjacentHTML("afterbegin", popover_filter)

let last_focused_input = null;

document.querySelectorAll("input#start-ts, input#end-ts").forEach(function (element) {
    let timezone = localStorage.getItem("timezone")
    let start_ts = localStorage.getItem("start_ts")
    let end_ts = localStorage.getItem("end_ts")

    if (element.id == "start-ts" && start_ts != null) {
        let date = Temporal.Instant.fromEpochMilliseconds(start_ts * 1000).toZonedDateTimeISO(timezone)
        element.value = date.toString().slice(0, 16)
    }

    if (element.id == "end-ts" && end_ts != null) {
        let date = Temporal.Instant.fromEpochMilliseconds(end_ts * 1000).toZonedDateTimeISO(timezone)
        element.value = date.toString().slice(0, 16)
    }

    element.addEventListener("focus", function() {
        last_focused_input = element
    })
})

document.querySelectorAll("button#ts-set-time").forEach(function (button) {
    button.addEventListener("click", function () {
        if (last_focused_input == null) {
            return
        }

        let now = Temporal.Now.zonedDateTimeISO(localStorage.getItem("timezone"))

        let button_class = button.className

        let new_time = null

        // ===== Relative time =====
        if (button_class == "ts-set-now") {
            new_time = now
        }

        if (button_class == "ts-set-last-week") {
            new_time = now.subtract({weeks: 1})
        }

        if (button_class == "ts-set-last-month") {
            new_time = now.subtract({months: 1})
        }

        if (button_class == "ts-set-last-year") {
            new_time = now.subtract({years: 1})
        }

        // ===== Absolute Time ======
        if (button_class == "ts-set-january") {
            new_time = now.with({month: 1, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-february") {
            new_time = now.with({month: 2, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-march") {
            new_time = now.with({month: 3, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-april") {
            new_time = now.with({month: 4, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-may") {
            new_time = now.with({month: 5, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-june") {
            new_time = now.with({month: 6, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-july") {
            new_time = now.with({month: 7, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-august") {
            new_time = now.with({month: 8, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-september") {
            new_time = now.with({month: 9, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-october") {
            new_time = now.with({month: 10, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-november") {
            new_time = now.with({month: 11, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (button_class == "ts-set-december") {
            new_time = now.with({month: 12, day: 1, hour: 0, minute: 0, second: 0})
        }

        if (new_time != null) {
            last_focused_input.value = new_time.toString().slice(0, 16)
        }

        console.log(plain.toString())
    })
})

document.getElementById("submit-filter").addEventListener("click", function () {
    let timezone = localStorage.getItem("timezone")

    let start = document.getElementById("start-ts").value.slice(0, 16)
    let end = document.getElementById("end-ts").value.slice(0, 16)

    let start_ts = Math.floor(Temporal.PlainDateTime.from(start).toZonedDateTime(timezone).epochMilliseconds / 1000)
    let end_ts = Math.floor(Temporal.PlainDateTime.from(end).toZonedDateTime(timezone).epochMilliseconds / 1000)

    localStorage.setItem("start_ts", start_ts)
    localStorage.setItem("end_ts", end_ts)

    window.location.reload()
})

document.getElementById("clear-filter").addEventListener("click", function () {
    localStorage.removeItem("start_ts")
    localStorage.removeItem("end_ts")

    window.location.reload()
})
