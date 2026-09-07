const page = document.getElementById("layout")

let sidebar_html = await fetch("sidebar.html")
sidebar_html = await sidebar_html.text()

page.insertAdjacentHTML("afterbegin", sidebar_html)
