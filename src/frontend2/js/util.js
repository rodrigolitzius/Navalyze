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

export {relative_time}
