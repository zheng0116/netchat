let currentMode = 'group';
let ws = null;
const token = localStorage.getItem('token');
const username = localStorage.getItem('username');

if (!token || !username) window.location.href = '/';

function switchChat(mode) {
    currentMode = mode;
    document.getElementById('group-chat-btn').classList.toggle('active', mode === 'group');
    document.getElementById('ai-chat-btn').classList.toggle('active', mode === 'ai');
    document.getElementById('group-messages').style.display = mode === 'group' ? 'flex' : 'none';
    document.getElementById('ai-container').style.display = mode === 'ai' ? 'block' : 'none';
    document.querySelector('.file-button').style.display = 'block';
    document.querySelector('.model-selector').style.display = mode === 'ai' ? 'block' : 'none';
    if (mode === 'group' && (!ws || ws.readyState !== WebSocket.OPEN)) {
        connectWebSocket();
    }
    if (mode === 'ai') {
        switchModel(currentModel);
    }
}

async function connectWebSocket() {
    if (ws?.readyState === WebSocket.OPEN) return true;

    return new Promise((resolve, reject) => {
        ws = new WebSocket(`ws://${window.location.host}/api/ws?token=${encodeURIComponent(token)}&username=${encodeURIComponent(username)}`);

        ws.onopen = () => resolve(true);

        ws.onmessage = (event) => {
            try {
                const data = JSON.parse(event.data);
                addMessage(data.userId, data.message, data.userId === username);
            } catch (error) {
                console.error('解析消息失败:', error);
            }
        };

        ws.onclose = () => {
            ws = null;
            if (currentMode === 'group') setTimeout(connectWebSocket, 1000);
            resolve(false);
        };

        ws.onerror = (error) => {
            ws = null;
            reject(error);
        };

        setTimeout(() => {
            if (ws.readyState !== WebSocket.OPEN) {
                ws.close();
                resolve(false);
            }
        }, 5000);
    });
}

function formatMessage(text) {
    let formattedText = text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    formattedText = formattedText.replace(/```(\w+)?\n([\s\S]*?)```/g, (_, lang, code) =>
        `<pre><code class="language-${lang || ''}">${code.trim()}</code></pre>`
    );
    return formattedText.replace(/`([^`]+)`/g, '<code>$1</code>').replace(/\n/g, '<br>');
}

function addMessage(userId, text, isSelf, isAI = false) {
    const container = currentMode === 'group'
        ? document.getElementById('group-messages')
        : document.getElementById(`${currentModel}-chat`);
    const messageDiv = document.createElement('div');
    messageDiv.className = `message ${isAI ? 'ai' : (isSelf ? 'self' : 'other')}`;

    const avatarDiv = document.createElement('div');
    avatarDiv.className = 'user-avatar';
    avatarDiv.textContent = isAI ? 'AI' : userId.charAt(0).toUpperCase();

    const contentDiv = document.createElement('div');
    contentDiv.className = 'message-content';

    if (text.includes('分享了文件:')) {
        const [_, fileName, fileUrl] = text.match(/分享了文件: (.*?) \[(.*?)\]/);
        contentDiv.innerHTML = `
            <div class="user-id">${userId}</div>
            <a href="${fileUrl}" class="file-message" download><span>📎</span><span>${fileName}</span></a>
        `;
    } else {
        contentDiv.innerHTML = `
            <div class="user-id">${userId}</div>
            <div class="message-text">${formatMessage(text)}</div>
        `;
    }

    if (isSelf) {
        messageDiv.appendChild(contentDiv);
        messageDiv.appendChild(avatarDiv);
    } else {
        messageDiv.appendChild(avatarDiv);
        messageDiv.appendChild(contentDiv);
    }

    container.appendChild(messageDiv);
    container.scrollTop = container.scrollHeight;
    messageDiv.querySelectorAll('pre code').forEach(block => hljs.highlightElement(block));
}

let currentImagePath = null;

function handleFileButtonClick() {
    if (currentMode === 'group') {
        document.getElementById('file-input').click();
    } else {
        document.getElementById('ai-image-input').click();
    }
}

function clearPreview() {
    console.log('Clearing preview, previous path:', currentImagePath);
    currentImagePath = null;
    const container = document.querySelector('.preview-image-container');
    container.style.display = 'none';
    const previewImg = container.querySelector('.preview-image');
    previewImg.src = '';
}

async function handleUpload(file, isAIChat = false) {
    if (!file) return;

    if (isAIChat && !file.type.startsWith('image/')) {
        alert('请上传图片文件');
        return;
    }

    const formData = new FormData();
    formData.append('file', file);
    formData.append('userId', username);

    try {
        const response = await fetch('/api/upload', {
            method: 'POST',
            headers: {
                'Authorization': `Bearer ${token}`
            },
            body: formData
        });

        if (response.ok) {
            const data = await response.json();

            if (isAIChat) {
                console.log('Upload response data:', data);
                currentImagePath = data.url;

                if (!currentImagePath) {
                    console.error('Upload response format:', data);
                    alert('上传成功但无法获取文件路径，响应格式：' + JSON.stringify(data));
                    return;
                }

                const container = document.querySelector('.preview-image-container');
                const previewImg = container.querySelector('.preview-image');
                previewImg.src = currentImagePath;
                container.style.display = 'block';

                document.getElementById('model-selector').value = 'qwen_vl';
                switchModel('qwen_vl');
            } else {
                ws.send(JSON.stringify({
                    userId: username,
                    message: `分享了文件: ${file.name} [${data.url}]`
                }));
            }
        } else {
            const errorText = await response.text();
            console.error('Upload failed:', errorText);
            alert('上传失败：' + errorText);
        }
    } catch (error) {
        console.error('Upload error:', error);
        alert('上传失败：' + error.message);
    }
}

document.getElementById('ai-image-input').addEventListener('change', e => {
    const file = e.target.files[0];
    handleUpload(file, true);
});

document.getElementById('file-input').addEventListener('change', e => {
    const file = e.target.files[0];
    handleUpload(file, false);
});

async function sendMessage() {
    const input = document.getElementById('message-input');
    const message = input.value.trim();
    if (!message) return;

    if (currentMode === 'group') {
        try {
            if (!await connectWebSocket()) {
                addMessage('系统', 'WebSocket 连接失败，正在重试...', false);
                return;
            }
            ws.send(JSON.stringify({ userId: username, message }));
            input.value = '';
        } catch (error) {
            addMessage('系统', '发送失败，请重试', false);
        }
    } else {
        const imagePath = currentImagePath;

        addMessage(username, message, true);
        input.value = '';

        if (imagePath) {
            const imgPreview = document.createElement('img');
            imgPreview.src = imagePath;
            imgPreview.className = 'message-image';
            const lastMessage = document.querySelector(`#${currentModel}-chat .message:last-child .message-content`);
            lastMessage.appendChild(imgPreview);

            clearPreview();
        }

        try {
            const requestBody = {
                message,
                model: imagePath ? 'qwen_vl' : currentModel,
                image_path: imagePath
            };
            console.log('Sending AI chat request:', requestBody);

            const response = await fetch('/api/ai_chat', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': `Bearer ${token}`
                },
                body: JSON.stringify(requestBody)
            });

            if (response.ok) {
                const data = await response.json();
                addMessage('AI 助手', data.response, false, true);
            } else {
                const errorText = await response.text();
                console.error('AI response error:', errorText);
                addMessage('系统', '获取 AI 响应失败，请重试', false);
            }
        } catch (error) {
            console.error('网络错误：', error);
            addMessage('系统', '网络错误，请重试', false);
        }
    }
}

document.getElementById('message-input').addEventListener('keypress', e => {
    if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        sendMessage();
    }
});

let currentModel = 'openai';

function switchModel(model) {
    console.log('Switching model to:', model);
    currentModel = model;
    document.querySelectorAll('.model-chat').forEach(chat => {
        chat.classList.remove('active');
    });
    document.getElementById(`${model}-chat`).classList.add('active');
}

document.addEventListener('DOMContentLoaded', () => {
    currentMode = 'group';
    document.getElementById('group-chat-btn').classList.add('active');
    document.getElementById('ai-chat-btn').classList.remove('active');

    document.getElementById('group-messages').style.display = 'flex';
    document.getElementById('ai-container').style.display = 'none';
    document.querySelector('.model-selector').style.display = 'none';

    connectWebSocket();
});  