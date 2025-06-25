import axios from 'axios'

// API Client配置
export const apiClient = axios.create({
  baseURL: 'http://localhost:8080/api',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
})

// 响应拦截器 - 统一错误处理
apiClient.interceptors.response.use(
  (response) => response.data,
  (error) => {
    console.error('API Error:', error)
    return Promise.reject(error)
  }
)

// 类型定义
export interface Book {
  id: number
  title: string
  author: string
  isbn?: string
  publisher?: string
  page_count?: number
  description?: string
  created_at: string
  updated_at: string
}

export interface CreateBookRequest {
  title: string
  author: string
  isbn?: string
  publisher?: string
  page_count?: number
  description?: string
}

export interface Note {
  id: number
  book_id: number
  note_type: 'quote' | 'summary' | 'thought' | 'general'
  title?: string
  content: string
  page_reference?: number
  is_favorite: boolean
  created_at: string
  updated_at: string
  book?: Book
  tags?: string[]
}

export interface CreateNoteRequest {
  book_id: number
  note_type: 'quote' | 'summary' | 'thought' | 'general'
  title?: string
  content: string
  page_reference?: number
  is_favorite?: boolean
  tags?: string[]
}

export interface Tag {
  id: number
  name: string
  slug: string
  usage_count: number
  created_at: string
  updated_at: string
}

export interface CreateTagRequest {
  name: string
}

export interface PaginationParams {
  page?: number
  per_page?: number
  search?: string
  note_type?: string
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  per_page: number
  total_pages: number
}

export interface BookListResponse {
  books: Book[]
  total: number
  page: number
  per_page: number
  total_pages: number
}

export interface NoteListResponse {
  notes: Note[]
  total: number
  page: number
  per_page: number
  total_pages: number
}

export interface TagListResponse {
  tags: Tag[]
  total: number
  page: number
  per_page: number
  total_pages: number
}

// Books API
export const booksAPI = {
  getBooks: (params?: PaginationParams): Promise<BookListResponse> =>
    apiClient.get('/books', { params }),
  
  getBook: (id: number): Promise<Book> =>
    apiClient.get(`/books/${id}`),
  
  createBook: (data: CreateBookRequest): Promise<Book> =>
    apiClient.post('/books', data),
  
  updateBook: (id: number, data: Partial<CreateBookRequest>): Promise<Book> =>
    apiClient.put(`/books/${id}`, data),
  
  deleteBook: (id: number): Promise<void> =>
    apiClient.delete(`/books/${id}`),
  
  getBookNotes: (id: number, params?: PaginationParams): Promise<NoteListResponse> =>
    apiClient.get(`/books/${id}/notes`, { params }),
}

// Notes API
export const notesAPI = {
  getNotes: (params?: PaginationParams): Promise<NoteListResponse> =>
    apiClient.get('/notes', { params }),
  
  getNote: (id: number): Promise<Note> =>
    apiClient.get(`/notes/${id}`),
  
  createNote: (data: CreateNoteRequest): Promise<Note> =>
    apiClient.post('/notes', data),
  
  updateNote: (id: number, data: Partial<CreateNoteRequest>): Promise<Note> =>
    apiClient.put(`/notes/${id}`, data),
  
  deleteNote: (id: number): Promise<void> =>
    apiClient.delete(`/notes/${id}`),
  
  updateNoteTags: (id: number, tags: string[]): Promise<Note> =>
    apiClient.put(`/notes/${id}/tags`, { tags }),
}

// Tags API
export const tagsAPI = {
  getTags: (params?: PaginationParams): Promise<TagListResponse> =>
    apiClient.get('/tags', { params }),
  
  getPopularTags: (): Promise<Tag[]> =>
    apiClient.get('/tags/popular'),
  
  getTag: (id: number): Promise<Tag> =>
    apiClient.get(`/tags/${id}`),
  
  createTag: (data: CreateTagRequest): Promise<Tag> =>
    apiClient.post('/tags', data),
  
  updateTag: (id: number, data: Partial<CreateTagRequest>): Promise<Tag> =>
    apiClient.put(`/tags/${id}`, data),
  
  deleteTag: (id: number): Promise<void> =>
    apiClient.delete(`/tags/${id}`),
}