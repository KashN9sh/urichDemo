import { useCallback, useEffect, useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import {
  assignTask,
  completeTask,
  createTask,
  listEmployees,
  listTasksByEmployee,
  type Employee,
  type Task,
} from '../api/client';

function StatusBadge({ status }: { status: string }) {
  const className =
    status === 'completed'
      ? 'badge badge--completed'
      : status === 'assigned'
        ? 'badge badge--assigned'
        : 'badge badge--open';
  return <span className={className}>{status}</span>;
}

export default function TasksPage() {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const selectedEmployeeId = searchParams.get('employee_id') ?? '';
  const [employees, setEmployees] = useState<Employee[]>([]);
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState('');
  const [showForm, setShowForm] = useState(false);
  const [title, setTitle] = useState('');
  const [assigneeId, setAssigneeId] = useState('');
  const [taskId, setTaskId] = useState('');
  const [assignTaskId, setAssignTaskId] = useState('');
  const [assignNewAssigneeId, setAssignNewAssigneeId] = useState('');

  const loadEmployees = useCallback(async () => {
    try {
      const data = await listEmployees();
      setEmployees(data ?? []);
      if (data?.length && !assigneeId) setAssigneeId(data[0].id);
      if (data?.length && !assignNewAssigneeId) setAssignNewAssigneeId(data[0].id);
    } catch {
      // ignore
    }
  }, [assigneeId, assignNewAssigneeId]);

  const loadTasks = useCallback(async () => {
    if (!selectedEmployeeId) {
      setTasks([]);
      setLoading(false);
      return;
    }
    setLoading(true);
    setError('');
    try {
      const data = await listTasksByEmployee(selectedEmployeeId);
      setTasks(data ?? []);
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to load');
    } finally {
      setLoading(false);
    }
  }, [selectedEmployeeId]);

  useEffect(() => {
    loadEmployees();
  }, [loadEmployees]);

  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  async function handleCreate(e: React.FormEvent) {
    e.preventDefault();
    setError('');
    try {
      const id = taskId || `task-${Date.now()}`;
      await createTask(id, title, assigneeId);
      setTitle('');
      setTaskId('');
      setAssigneeId(employees[0]?.id ?? '');
      setShowForm(false);
      loadTasks();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed to create');
    }
  }

  async function handleComplete(id: string) {
    setError('');
    try {
      await completeTask(id);
      loadTasks();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed');
    }
  }

  async function handleAssign(e: React.FormEvent) {
    e.preventDefault();
    if (!assignTaskId || !assignNewAssigneeId) return;
    setError('');
    try {
      await assignTask(assignTaskId, assignNewAssigneeId);
      setAssignTaskId('');
      setAssignNewAssigneeId(employees[0]?.id ?? '');
      loadTasks();
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Failed');
    }
  }

  const employeeName = (id: string) => employees.find((e) => e.id === id)?.name ?? id;

  return (
    <div>
      <div className="page-header">
        <h1>Задачи</h1>
      </div>
      <div className="form-inline" style={{ marginBottom: '1rem' }}>
        <label htmlFor="employee-select" className="muted" style={{ marginRight: '0.5rem' }}>
          Сотрудник:
        </label>
        <select
          id="employee-select"
          className="form-control"
          value={selectedEmployeeId}
          onChange={(e) =>
            navigate(`/tasks?employee_id=${encodeURIComponent(e.target.value)}`, {
              replace: true,
            })
          }
          style={{ minWidth: '200px' }}
        >
          <option value="">— выбрать —</option>
          {employees.map((e) => (
            <option key={e.id} value={e.id}>
              {e.name} ({e.id})
            </option>
          ))}
        </select>
        {!selectedEmployeeId && (
          <span className="muted" style={{ marginLeft: '0.5rem' }}>
            Выберите сотрудника или перейдите из списка сотрудников.
          </span>
        )}
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
                  <th>Название</th>
                  <th>Исполнитель</th>
                  <th>Статус</th>
                  <th className="cell-actions">Действия</th>
                </tr>
              </thead>
              <tbody>
                {tasks.map((t) => (
                  <tr key={t.id}>
                    <td><strong>{t.title}</strong></td>
                    <td>{employeeName(t.assignee_id)}</td>
                    <td><StatusBadge status={t.status} /></td>
                    <td className="cell-actions">
                      {t.status !== 'completed' && (
                        <>
                          <button
                            type="button"
                            className="btn btn-secondary btn-sm"
                            onClick={() => handleComplete(t.id)}
                          >
                            Выполнена
                          </button>
                          <button
                            type="button"
                            className="btn btn-secondary btn-sm"
                            onClick={() => setAssignTaskId(t.id)}
                          >
                            Назначить
                          </button>
                        </>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
          {assignTaskId && (
            <div className="card" style={{ marginTop: '1.5rem' }}>
              <h2 className="card-title">Назначить задачу</h2>
              <form onSubmit={handleAssign} className="form-inline">
                <span className="muted" style={{ marginRight: '0.5rem' }}>
                  Задачу на исполнителя:
                </span>
                <select
                  className="form-control"
                  value={assignNewAssigneeId}
                  onChange={(e) => setAssignNewAssigneeId(e.target.value)}
                  style={{ minWidth: '160px' }}
                >
                  {employees.map((e) => (
                    <option key={e.id} value={e.id}>
                      {e.name}
                    </option>
                  ))}
                </select>
                <button type="submit" className="btn btn-primary">
                  OK
                </button>
                <button
                  type="button"
                  className="btn btn-secondary"
                  onClick={() => setAssignTaskId('')}
                >
                  Отмена
                </button>
              </form>
            </div>
          )}
          {!showForm ? (
            <button
              type="button"
              className="btn btn-primary"
              style={{ marginTop: '1.5rem' }}
              onClick={() => setShowForm(true)}
            >
              Добавить задачу
            </button>
          ) : (
            <div className="card" style={{ marginTop: '1.5rem' }}>
              <h2 className="card-title">Новая задача</h2>
              <form onSubmit={handleCreate}>
                <div className="form-inline" style={{ marginBottom: '1rem' }}>
                  <input
                    className="form-control"
                    placeholder="ID задачи (опционально)"
                    value={taskId}
                    onChange={(e) => setTaskId(e.target.value)}
                  />
                  <input
                    className="form-control"
                    placeholder="Название"
                    value={title}
                    onChange={(e) => setTitle(e.target.value)}
                    required
                  />
                  <select
                    className="form-control"
                    value={assigneeId}
                    onChange={(e) => setAssigneeId(e.target.value)}
                    style={{ minWidth: '160px' }}
                  >
                    {employees.map((e) => (
                      <option key={e.id} value={e.id}>
                        {e.name}
                      </option>
                    ))}
                  </select>
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
