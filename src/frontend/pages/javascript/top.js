document.addEventListener('DOMContentLoaded', function () {
    const token = localStorage.getItem('Token');

    if (!token) {
        window.location.replace('../index.html');
        return;
    }

    document.getElementById('back-btn').addEventListener('click', function () {
        window.location.replace('../pages/hub.html');
    });

    document.getElementById('logout-btn').addEventListener('click', function () {
        localStorage.removeItem('Token');
        window.location.replace('../index.html');
    });

    function normaliza(nome) {
        return (nome || '').trim().toLowerCase();
    }

    function urlArtista(id, nome) {
        return 'artist.html?id=' + encodeURIComponent(id) + '&nome=' + encodeURIComponent(nome || '');
    }

    function gradientePorNome(nome) {
        let hash = 0;
        for (let i = 0; i < nome.length; i++) {
            hash = nome.charCodeAt(i) + ((hash << 5) - hash);
        }
        const hue1 = Math.abs(hash) % 360;
        const hue2 = (hue1 + 50) % 360;
        return 'linear-gradient(135deg, hsl(' + hue1 + ', 60%, 28%) 0%, hsl(' + hue2 + ', 55%, 16%) 100%)';
    }

    function inicialDoNome(nome) {
        return nome.trim().charAt(0).toUpperCase();
    }

    // cards de ARTISTA usam <img> com object-fit:cover — resolve enquadramento pra qualquer proporcao
    function montaCardArtista(nome, plays) {
        const card = document.createElement('div');
        card.className = 'top-card top-card-artist';
        card.style.background = gradientePorNome(nome);

        // imagem — começa oculta, aparece quando carrega
        const img = document.createElement('img');
        img.className = 'top-card-img';
        img.alt = nome;
        card.appendChild(img);

        // inicial de fallback — some quando a imagem carrega
        const inicial = document.createElement('div');
        inicial.className = 'top-card-initial';
        inicial.textContent = inicialDoNome(nome);
        card.appendChild(inicial);

        const overlay = document.createElement('div');
        overlay.className = 'top-card-overlay';
        overlay.innerHTML =
            '<div class="top-card-name">' + nome + '</div>' +
            '<div class="top-card-plays">' + plays + (plays === 1 ? ' play' : ' plays') + '</div>';
        card.appendChild(overlay);

        return card;
    }

    function aplicaFotoArtista(card, artistId) {
        if (!artistId) return;

        fetch('/api/relay/getCoverArt?id=' + artistId + '&size=400', {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('sem foto');
                return r.blob();
            })
            .then(function (blob) {
                const url = URL.createObjectURL(blob);
                const img = card.querySelector('.top-card-img');
                if (!img) return;
                img.onload = function () {
                    img.style.opacity = '1';
                    const inicial = card.querySelector('.top-card-initial');
                    if (inicial) inicial.remove();
                };
                img.src = url;
            })
            .catch(function () {});
    }

    // cards de ALBUM/FAIXA continuam usando background-image (capas sao sempre quadradas)
    function aplicaCapa(card, coverId) {
        if (!coverId) return;

        fetch('/api/relay/getCoverArt?id=' + coverId + '&size=300', {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('sem capa');
                return r.blob();
            })
            .then(function (blob) {
                const url = URL.createObjectURL(blob);
                card.style.backgroundImage = 'url(' + url + ')';
            })
            .catch(function () {});
    }

    function montaCard(nome, subtitulo, plays, artistId) {
        const card = document.createElement('div');
        card.className = 'top-card';

        const overlay = document.createElement('div');
        overlay.className = 'top-card-overlay';

        const nomeDiv = document.createElement('div');
        nomeDiv.className = 'top-card-name';
        nomeDiv.textContent = nome;
        overlay.appendChild(nomeDiv);

        if (subtitulo) {
            const subDiv = document.createElement('div');
            subDiv.className = 'top-card-sub';
            subDiv.textContent = subtitulo;

            // Se o id do artista for fornecido, cria um link
            if (artistId) {
                subDiv.classList.add('artist-link');
                subDiv.addEventListener('click', function (ev) {
                    ev.stopPropagation();
                    window.location.href = urlArtista(artistId, subtitulo);
                });
            }

            overlay.appendChild(subDiv);
        }

        const playsDiv = document.createElement('div');
        playsDiv.className = 'top-card-plays';
        playsDiv.textContent = plays + (plays === 1 ? ' play' : ' plays');
        overlay.appendChild(playsDiv);

        card.appendChild(overlay);

        return card;
    }

    function carregaComCapaPropria(endpoint, elementId, montaSubtitulo) {
        const grid = document.getElementById(elementId);

        fetch(endpoint + '?limit=10', {
            headers: { 'Authorization': token }
        })
            .then(function (r) {
                if (!r.ok) throw new Error('erro ' + r.status);
                return r.json();
            })
            .then(function (dados) {
                if (!dados || dados.length === 0) {
                    grid.innerHTML = '<p style="color:#94a3b8;">Nenhum dado encontrado.</p>';
                    return;
                }

                grid.innerHTML = '';

                dados.forEach(function (item) {
                    const subtitulo = montaSubtitulo ? montaSubtitulo(item) : '';
                    const card = montaCard(item.name, subtitulo, item.plays, item.artist_id);
                    grid.appendChild(card);
                    aplicaCapa(card, item.id);
                });
            })
            .catch(function (err) {
                grid.innerHTML = '<p style="color:#f87171;">Erro: ' + err.message + '</p>';
            });
    }

    function renderizaArtistas(artistas) {
        const grid = document.getElementById('grid-artistas');

        if (!artistas || artistas.length === 0) {
            grid.innerHTML = '<p style="color:#94a3b8;">Nenhum dado encontrado.</p>';
            return;
        }

        grid.innerHTML = '';

        artistas.slice(0, 9).forEach(function (artista, index) {
            const card = montaCardArtista(artista.name, artista.plays);
            if (index === 0) {
                card.classList.add('destaque');
            }
            card.addEventListener('click', function () {
                window.location.href = urlArtista(artista.id, artista.name);
            });
            grid.appendChild(card);
            aplicaFotoArtista(card, artista.id);
        });
    }

    fetch('/api/most-played/artists?limit=10', {
        headers: { 'Authorization': token }
    })
        .then(function (r) {
            if (!r.ok) throw new Error('erro ' + r.status);
            return r.json();
        })
        .then(function (artistas) {
            artistas = artistas || [];

            renderizaArtistas(artistas);
            carregaComCapaPropria('/api/most-played/albums', 'grid-albuns', function (item) { return item.artist; });
            carregaComCapaPropria('/api/most-played/tracks', 'grid-faixas', function (item) { return item.artist; });
        })
        .catch(function (err) {
            document.getElementById('grid-artistas').innerHTML = '<p style="color:#f87171;">Erro: ' + err.message + '</p>';
            carregaComCapaPropria('/api/most-played/albums', 'grid-albuns', function (item) { return item.artist; });
            carregaComCapaPropria('/api/most-played/tracks', 'grid-faixas', function (item) { return item.artist; });
        });
});
