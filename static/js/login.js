const showError = (message) => {
    const errorDiv = document.getElementById('error');
    const loginContainer = document.querySelector('.login-container');
    errorDiv.textContent = message;
    errorDiv.style.display = 'block';
    loginContainer.classList.add('shake');
    setTimeout(() => loginContainer.classList.remove('shake'), 500);
};

document.getElementById('loginForm').addEventListener('submit', async (e) => {
    e.preventDefault();
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;

    try {
        const response = await fetch('/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password }),
        });

        if (response.ok) {
            const data = await response.json();
            localStorage.setItem('token', data.token);
            localStorage.setItem('username', data.username);
            window.location.href = '/chat';
        } else {
            const error = await response.text();
            showError(error || '登录失败，请检查用户名和密码');
        }
    } catch (error) {
        showError('网络错误，请稍后重试');
    }
});