const form = document.getElementById('loginform');
const btn = document.getElementById('login-btn');

form.addEventListener('submit', function(event) {
    event.preventDefault();

    const nd_url = document.getElementById('navidrome-url').value;
    const nome = document.getElementById('nome').value;
    const senha = document.getElementById('senha').value;

    if (!nd_url || !nome || !senha) {
        alert('Preencha todos os campos');
        return;
    }

    const textooriginal = btn.textContent;
    btn.textContent = 'Entrando...';
    btn.disabled = true;

    fetch('/api/login', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ username: nome, password: senha, url: nd_url })
    }).then(function(response) {
        if (!response.ok) {
            return response.json().catch(() => ({})).then(function(data) {
                const msg = data.error || 'Erro HTTP ' + response.status;
                throw new Error(msg);
            });
        }
        return response.json().then(function(json) {
            if (json.id) {
                localStorage.setItem('Token', json.id);
                localStorage.setItem('Navidrome-url', nd_url);
                window.location.replace('pages/hub.html');
            } else {
                alert('Login falhou: token não recebido');
            }
        });
    }).catch(function(error) {
        alert(error.message);
    }).finally(function() {
        btn.textContent = textooriginal;
        btn.disabled = false;
    });
});
