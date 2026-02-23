import { useCallback, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import {
  createEmployee,
  listEmployees,
  type Employee,
} from '../api/client';

export default function EmployeesPage() {
  const [list, setList] = useState<Employee[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [showForm, setShowForm] = useState(false);
  const [employeeId, setEmployeeId] = useState('');
  const [name, setName] = useState('');
  const [role, setRole] = useState('');

  const load = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const data = await listEmployees();
      setList(data ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    load();
  }, [load]);

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    setError('');
    try {
      await createEmployee(employeeId, name, role);
      setEmployeeId('');
      setName('');
      setRole('');
      setShowForm(false);
      load();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to create');
    }
  }

  return (
    <div>
      <div className="page-header">
        <h1>Сотрудники</h1>
        <button
          type="button"
          className="btn btn-primary"
          onClick={() => setShowForm(true)}
        >
          Добавить сотрудника
        </button>
      </div>
      {error && <p className="error">{error}</p>}
      {loading ? (
        <p className="muted">Загрузка…</p>
      ) : (
        <>
          <div className="table-wrap">
            <table className="table">
              <thead>
                <tr>
                  <th>Имя</th>
                  <th>Роль</th>
                  <th>ID</th>
                  <th className="cell-actions">Действия</th>
                </tr>
              </thead>
              <tbody>
                {list.map((e) => (
                  <tr key={e.id}>
                    <td><strong>{e.name}</strong></td>
                    <td>{e.role}</td>
                    <td><span className="muted">{e.id}</span></td>
                    <td className="cell-actions">
                      <Link to={`/tasks?employee_id=${e.id}`} className="btn btn-secondary btn-sm">
                        Задачи
                      </Link>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          {showForm && (
            <div className="card" style={{ marginTop: '1.5rem' }}>
              <h2 className="card-title">Новый сотрудник</h2>
              <form onSubmit={handleCreate}>
                <div className="form-inline" style={{ marginBottom: '1rem' }}>
                  <input
                    className="form-control"
                    placeholder="ID"
                    value={employeeId}
                    onChange={(e) => setEmployeeId(e.target.value)}
                    required
                  />
                  <input
                    className="form-control"
                    placeholder="Имя"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                  />
                  <input
                    className="form-control"
                    placeholder="Роль"
                    value={role}
                    onChange={(e) => setRole(e.target.value)}
                    required
                  />
                </div>
                <div className="form-inline">
                  <button type="submit" className="btn btn-primary">
                    Создать
                  </button>
                  <button
                    type="button"
                    className="btn btn-secondary"
                    onClick={() => setShowForm(false)}
                  >
                    Отмена
                  </button>
                </div>
              </form>
            </div>
          )}
        </>
      )}
    </div>
  );
}
