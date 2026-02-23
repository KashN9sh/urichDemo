import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { register } from '../api/client';
import { useAuth } from '../context/AuthContext';

export default function LoginPage() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');
  const [isRegister, setIsRegister] = useState(false);
  const { login } = useAuth();
  const navigate = useNavigate();

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setError('');
    if (isRegister) {
      try {
        const res = await register(username, password);
        if (res.detail) {
          setError(res.detail);
          return;
        }
        setIsRegister(false);
        setError('');
        setPassword('');
      } catch (e) {
        setError(e instanceof Error ? e.message : 'Registration failed');
      }
      return;
    }
    const result = await login(username, password);
    if (result.ok) navigate('/', { replace: true });
    else setError(result.error ?? 'Login failed');
  }

  return (
    <div className="login-page">
      <div className="login-card card">
        <h1>{isRegister ? 'Регистрация' : 'Вход'}</h1>
        <form onSubmit={handleSubmit}>
          <div className="form-group">
            <label htmlFor="username">Логин</label>
            <input
              id="username"
              type="text"
              className="form-control"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              required
              autoComplete="username"
            />
          </div>
          <div className="form-group">
            <label htmlFor="password">Пароль</label>
            <input
              id="password"
              type="password"
              className="form-control"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              required
              autoComplete={isRegister ? 'new-password' : 'current-password'}
            />
          </div>
          {error && <p className="error">{error}</p>}
          <button type="submit" className="btn btn-primary">
            {isRegister ? 'Зарегистрироваться' : 'Войти'}
          </button>
        </form>
        <p className="login-toggle">
          {isRegister ? (
            <>
              Уже есть аккаунт?{' '}
              <button type="button" className="btn btn-ghost" onClick={() => setIsRegister(false)}>
                Войти
              </button>
            </>
          ) : (
            <>
              Нет аккаунта?{' '}
              <button type="button" className="btn btn-ghost" onClick={() => setIsRegister(true)}>
                Зарегистрироваться
              </button>
            </>
          )}
        </p>
      </div>
    </div>
  );
}
