document.addEventListener('DOMContentLoaded', function() {
    const token = localStorage.getItem('Token');
    const nd_url = localStorage.getItem('Navidrome-url');

    if (!token) {
        window.location.replace('index.html');
        return;
    }

    document.getElementById('backend-url-display').textContent = nd_url;

    document.getElementById('logout-btn').addEventListener('click', function() {
        localStorage.removeItem('Token');
        window.location.replace('../index.html');
    });
});
