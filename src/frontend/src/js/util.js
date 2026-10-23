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
    console.log(api_data)
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

export {relative_time, format_hours, build_time_graph }
