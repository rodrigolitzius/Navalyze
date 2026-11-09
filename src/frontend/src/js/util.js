import { get_image_url } from './api';
import Chart from 'chart.js/auto';
import 'chartjs-adapter-temporal/register';

function relative_time(timestamp_seconds) {
    const diff_seconds = Math.max(0, (Date.now() / 1000) - timestamp_seconds)

    const minutes = Math.floor(diff_seconds / 60)
    if (minutes < 1) return "Now"
    if (minutes < 60) return `${minutes} minutes ago`

    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `${hours} hours ago`

    const days = Math.floor(hours / 24)
    return `${days} days ago`
}

function format_hours(hours) {
    if (hours < (1.0 / 60.0)) {
        return `${Math.round(hours * 60 * 60)}s`
    }

    if (hours < 1) {
        return `${Math.round(hours * 60)}m`
    }

    return `${hours.toFixed(1)}h`
}

function build_time_graph(ctx, api_data) {
    let datasets = new Array
    let timezone = localStorage.getItem("timezone")

    Object.entries(api_data).forEach((k, _) => {
        let data = new Array

        Object.entries(k[1]).forEach((k2, _) => {
            data.push({
                x: Temporal.Instant.fromEpochMilliseconds(k2[0] * 1000).toZonedDateTimeISO(timezone).toString().slice(0, 19),
                y: k2[1]
            })
        })

        datasets.push({
            label: k[0],
            data: data
        })
    })

    new Chart(ctx, {
        type: 'line',
        data: {
            datasets: datasets
        },
        options: {
            maintainAspectRatio: false,
            scales: {
                y: {
                    beginAtZero: true
                },
                x: {
                    type: "time"
                }
            }
        },
    });
}

async function safe_image_url(api, entry_id, size) {
    try {
        return get_image_url(api, entry_id, size)
    } catch {
        return null
    }
}

function lazy(fn) {
    let promise = null
    return () => (promise ??= fn())
}

function dedup_image_urls(api, imgs) {
    let url_map = new Map()

    let result = imgs.map((entry) => {
        let url = url_map.get(entry.id)
        if (!url) {
            url = lazy(() => safe_image_url(api, entry.id, entry.size))
            url_map.set(entry.id, url)
        }

        return {
            id: entry.id, url: url, size: entry.size, container: entry.container
        }
    })

    return result
}

async function load_images(api, imgs) {
    for (const entry of dedup_image_urls(api, imgs)) {
        let url = await entry.url()
        entry.container.querySelector("img").src = url
        entry.container.style.setProperty("--img-url", `url(${url})`)
    }
}

function setup_timezone_select(select) {
    const system_timezone = Intl.DateTimeFormat().resolvedOptions().timeZone
    const current_timezone = localStorage.getItem("timezone") || system_timezone

    const timezones = typeof Intl.supportedValuesOf === "function"
        ? Intl.supportedValuesOf("timeZone")
        : [system_timezone]

    if (!timezones.includes(current_timezone)) {
        timezones.unshift(current_timezone)
    }

    for (const timezone of timezones) {
        const option = document.createElement("option")
        option.value = timezone
        option.textContent = timezone.replaceAll("_", " ")
        if (timezone === current_timezone) option.selected = true
        select.appendChild(option)
    }

    if (!localStorage.getItem("timezone")) {
        localStorage.setItem("timezone", current_timezone)
    }

    select.addEventListener("change", () => {
        localStorage.setItem("timezone", select.value)
    })
}

function sleep_ms(time) {
    return new Promise(resolve => {
        setTimeout(resolve, time);
    });
}

export {relative_time, format_hours, build_time_graph, load_images, setup_timezone_select }
