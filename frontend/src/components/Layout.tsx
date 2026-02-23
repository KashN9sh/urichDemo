import { NavLink, Outlet } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';

export default function Layout() {
  const { logout } = useAuth();

  return (
    <div className="app-layout">
      <aside className="sidebar">
        <div className="sidebar-brand">Urich Demo</div>
        <nav className="sidebar-nav">
          <NavLink to="/" className={({ isActive }) => `sidebar-link${isActive ? ' active' : ''}`} end>
            Сотрудники
          </NavLink>
          <NavLink to="/tasks" className={({ isActive }) => `sidebar-link${isActive ? ' active' : ''}`}>
            Задачи
          </NavLink>
        </nav>
        <div className="sidebar-api">
          <span className="sidebar-api-title">API</span>
          <a href={`http://${window.location.hostname}:8001/docs`} target="_blank" rel="noopener noreferrer" className="sidebar-api-link">
            Auth
          </a>
          <a href={`http://${window.location.hostname}:8002/docs`} target="_blank" rel="noopener noreferrer" className="sidebar-api-link">
            Employees
          </a>
          <a href={`http://${window.location.hostname}:8003/docs`} target="_blank" rel="noopener noreferrer" className="sidebar-api-link">
            Tasks
          </a>
        </div>
        <div className="sidebar-footer">
          <button type="button" className="btn btn-ghost" onClick={logout}>
            Выйти
          </button>
        </div>
      </aside>
      <main className="main-content">
        <Outlet />
      </main>
    </div>
  );
}
