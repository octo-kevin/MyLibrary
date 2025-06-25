import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import './index.css'

// 导入页面组件
import Layout from './components/Layout'
import HomePage from './pages/HomePage'
import BooksPage from './pages/BooksPage'
import BookDetailPage from './pages/BookDetailPage'
import BookFormPage from './pages/BookFormPage'
import NotesPage from './pages/NotesPage'
import NoteDetailPage from './pages/NoteDetailPage'
import NoteFormPage from './pages/NoteFormPage'
import TagsPage from './pages/TagsPage'
import TagFormPage from './pages/TagFormPage'

// 创建查询客户端
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 5 * 60 * 1000, // 5分钟
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
})

// 创建路由器
const router = createBrowserRouter([
  {
    path: '/',
    element: <Layout />,
    children: [
      {
        index: true,
        element: <HomePage />,
      },
      {
        path: 'books',
        element: <BooksPage />,
      },
      {
        path: 'books/new',
        element: <BookFormPage />,
      },
      {
        path: 'books/:id',
        element: <BookDetailPage />,
      },
      {
        path: 'books/:id/edit',
        element: <BookFormPage />,
      },
      {
        path: 'notes',
        element: <NotesPage />,
      },
      {
        path: 'notes/new',
        element: <NoteFormPage />,
      },
      {
        path: 'notes/:id',
        element: <NoteDetailPage />,
      },
      {
        path: 'notes/:id/edit',
        element: <NoteFormPage />,
      },
      {
        path: 'tags',
        element: <TagsPage />,
      },
      {
        path: 'tags/new',
        element: <TagFormPage />,
      },
      {
        path: 'tags/:id/edit',
        element: <TagFormPage />,
      },
    ],
  },
])

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  )
}

export default App