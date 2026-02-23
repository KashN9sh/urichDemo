const API_BASE = import.meta.env.VITE_API_BASE ?? '/api';

export function getToken(): string | null {
  return localStorage.getItem('token');
}

export function clearToken(): void {
  localStorage.removeItem('token');
}

export function setToken(token: string): void {
  localStorage.setItem('token', token);
}

async function request<T>(
  path: string,
  options: RequestInit & { requireAuth?: boolean } = {}
): Promise<T> {
  const { requireAuth = true, ...init } = options;
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(init.headers as Record<string, string>),
  };
  if (requireAuth) {
    const token = getToken();
    if (token) headers['Authorization'] = `Bearer ${token}`;
  }
  const url = path.startsWith('http') ? path : `${API_BASE}${path}`;
  const res = await fetch(url, { ...init, headers });
  if (res.status === 401) {
    clearToken();
    window.dispatchEvent(new CustomEvent('auth:401'));
    throw new Error('Unauthorized');
  }
  if (!res.ok) {
    const text = await res.text();
    let detail = text;
    try {
      const j = JSON.parse(text);
      detail = j.detail ?? text;
    } catch {
      // use text as is
    }
    throw new Error(detail);
  }
  const contentType = res.headers.get('content-type');
  if (contentType?.includes('application/json')) return res.json() as Promise<T>;
  return undefined as T;
}

// Auth
export interface LoginResponse {
  token?: string;
  user?: { id: string; username: string; role: string };
  detail?: string;
}

export async function login(username: string, password: string): Promise<LoginResponse> {
  return request<LoginResponse>('/auth/login', {
    method: 'POST',
    body: JSON.stringify({ username, password }),
    requireAuth: false,
  });
}

export async function register(
  username: string,
  password: string,
  role = 'user'
): Promise<{ id?: string; username?: string; role?: string; detail?: string }> {
  return request('/auth/register', {
    method: 'POST',
    body: JSON.stringify({ username, password, role }),
    requireAuth: false,
  });
}

// Employees
export interface Employee {
  id: string;
  name: string;
  role: string;
}

export async function listEmployees(search = ''): Promise<Employee[]> {
  const q = search ? `?search=${encodeURIComponent(search)}` : '';
  return request<Employee[]>(`/employees/queries/list_employees${q}`);
}

export async function getEmployee(employeeId: string): Promise<Employee | null> {
  return request<Employee | null>(
    `/employees/queries/get_employee?employee_id=${encodeURIComponent(employeeId)}`
  );
}

export async function createEmployee(
  employeeId: string,
  name: string,
  role: string
): Promise<string> {
  const body = await request<{ ok: boolean; result?: string }>(
    '/employees/commands/create_employee',
    {
      method: 'POST',
      body: JSON.stringify({ employee_id: employeeId, name, role }),
    }
  );
  return body?.result ?? employeeId;
}

// Tasks
export interface Task {
  id: string;
  title: string;
  assignee_id: string;
  status: string;
}

export async function listTasksByEmployee(employeeId: string): Promise<Task[]> {
  return request<Task[]>(
    `/tasks/queries/list_tasks_by_employee?employee_id=${encodeURIComponent(employeeId)}`
  );
}

export async function getTask(taskId: string): Promise<Task | null> {
  return request<Task | null>(
    `/tasks/queries/get_task?task_id=${encodeURIComponent(taskId)}`
  );
}

export async function createTask(
  taskId: string,
  title: string,
  assigneeId: string
): Promise<string> {
  const body = await request<{ ok: boolean; result?: string }>(
    '/tasks/commands/create_task',
    {
      method: 'POST',
      body: JSON.stringify({ task_id: taskId, title, assignee_id: assigneeId }),
    }
  );
  return body?.result ?? taskId;
}

export async function assignTask(taskId: string, assigneeId: string): Promise<void> {
  await request('/tasks/commands/assign_task', {
    method: 'POST',
    body: JSON.stringify({ task_id: taskId, assignee_id: assigneeId }),
  });
}

export async function completeTask(taskId: string): Promise<void> {
  await request('/tasks/commands/complete_task', {
    method: 'POST',
    body: JSON.stringify({ task_id: taskId }),
  });
}
