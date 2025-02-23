const chat = document.getElementById('chat');
const text = document.getElementById('text');
const uri = 'wss://' + location.host + '/chat';
const ws = new WebSocket(uri);

const message = (data) => {
    const line = document.createElement('p');
    line.innerText = data;
    chat.appendChild(line);
}

const send = () => {
    const msg = text.value;
    ws.send(msg);
    text.value = '';

    message('[You]: ' + msg);
};

ws.onopen = () => {
    chat.innerHTML = '<p><em>Connected!</em></p>';
};

ws.onmessage = (msg) => {
    message(msg.data);
};

ws.onclose = () => {
    chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
};

text.onkeyup = (e) => {
    if (e.key === 'Enter') {
        send();
    }
};

send.onclick = () => {
    send();
};