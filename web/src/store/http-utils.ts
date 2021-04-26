const BASE_URL = 'http://localhost:6001';

export const http = async <T>(path: string, init?: RequestInit): Promise<T> => {
  const trimmedPath = path.replace(/^\/+/, '');

  const response = await fetch(`${BASE_URL}/${trimmedPath}`, init);

  return (await response.json()) as T;
};
