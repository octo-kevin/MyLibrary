import { useEffect } from 'react'
import { useParams, useNavigate, Link } from 'react-router-dom'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { 
  Form, 
  Input, 
  Button, 
  Card, 
  Space, 
  Typography, 
  InputNumber,
  message,
  Breadcrumb
} from 'antd'
import {
  ArrowLeftOutlined,
  SaveOutlined,
  BookOutlined,
  HomeOutlined,
} from '@ant-design/icons'
import { booksAPI, type CreateBookRequest } from '../lib/api'
import { Loading, ErrorMessage } from '../components/ui'

const { Title, Text } = Typography
const { TextArea } = Input

interface FormData {
  title: string
  author: string
  isbn?: string
  publisher?: string
  page_count?: number
  description?: string
}

export default function BookFormPage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const queryClient = useQueryClient()
  const [form] = Form.useForm()
  const isEditing = !!id
  const bookId = parseInt(id || '0', 10)

  // Fetch book data for editing
  const { data: book, isLoading, error } = useQuery({
    queryKey: ['book', bookId],
    queryFn: () => booksAPI.getBook(bookId),
    enabled: isEditing,
  })

  // Load book data into form when editing
  useEffect(() => {
    if (book) {
      form.setFieldsValue({
        title: book.title,
        author: book.author,
        isbn: book.isbn || '',
        publisher: book.publisher || '',
        page_count: book.page_count || undefined,
        description: book.description || '',
      })
    }
  }, [book, form])

  // Create mutation
  const createMutation = useMutation({
    mutationFn: (data: CreateBookRequest) => booksAPI.createBook(data),
    onSuccess: () => {
      message.success('书籍创建成功！')
      queryClient.invalidateQueries({ queryKey: ['books'] })
      navigate('/books')
    },
    onError: (error: Error) => {
      console.error('Create error:', error)
      message.error('创建失败，请稍后重试')
    },
  })

  // Update mutation
  const updateMutation = useMutation({
    mutationFn: (data: CreateBookRequest) => booksAPI.updateBook(bookId, data),
    onSuccess: () => {
      message.success('书籍更新成功！')
      queryClient.invalidateQueries({ queryKey: ['books'] })
      queryClient.invalidateQueries({ queryKey: ['book', bookId] })
      navigate('/books')
    },
    onError: (error: Error) => {
      console.error('Update error:', error)
      message.error('更新失败，请稍后重试')
    },
  })

  const handleSubmit = (values: FormData) => {
    const submitData: CreateBookRequest = {
      title: values.title,
      author: values.author,
      isbn: values.isbn || null,
      publisher: values.publisher || null,
      page_count: values.page_count || null,
      description: values.description || null,
    }

    if (isEditing) {
      updateMutation.mutate(submitData)
    } else {
      createMutation.mutate(submitData)
    }
  }

  const isSubmitting = createMutation.isPending || updateMutation.isPending

  if (isLoading) {
    return <Loading size="large" text="加载书籍信息中..." />
  }

  if (error) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载书籍信息，请稍后重试"
        onRetry={() => window.location.reload()}
      />
    )
  }

  return (
    <div>
      {/* 面包屑导航 */}
      <Card style={{ marginBottom: 24 }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Breadcrumb>
            <Breadcrumb.Item>
              <Link to="/books">
                <HomeOutlined style={{ marginRight: 4 }} />
                书籍管理
              </Link>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <BookOutlined style={{ marginRight: 4 }} />
              {isEditing ? '编辑书籍' : '添加书籍'}
            </Breadcrumb.Item>
          </Breadcrumb>
          
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <div>
              <Title level={3} style={{ margin: 0 }}>
                {isEditing ? '编辑书籍' : '添加书籍'}
              </Title>
              <Text type="secondary">
                {isEditing ? '修改书籍信息' : '添加一本新书到您的个人图书馆'}
              </Text>
            </div>
            <Link to="/books">
              <Button icon={<ArrowLeftOutlined />}>
                返回列表
              </Button>
            </Link>
          </div>
        </Space>
      </Card>

      {/* 表单 */}
      <Card>
        <Form
          form={form}
          layout="vertical"
          onFinish={handleSubmit}
          autoComplete="off"
          style={{ maxWidth: 600 }}
        >
          <Form.Item
            label="书名"
            name="title"
            rules={[
              { required: true, message: '请输入书名' },
              { max: 200, message: '书名不能超过200个字符' }
            ]}
          >
            <Input
              placeholder="输入书籍标题"
              size="large"
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="作者"
            name="author"
            rules={[
              { required: true, message: '请输入作者姓名' },
              { max: 100, message: '作者姓名不能超过100个字符' }
            ]}
          >
            <Input
              placeholder="输入作者姓名"
              size="large"
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="ISBN"
            name="isbn"
            rules={[
              { max: 20, message: 'ISBN不能超过20个字符' }
            ]}
          >
            <Input
              placeholder="输入ISBN号码（可选）"
              size="large"
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="出版社"
            name="publisher"
            rules={[
              { max: 100, message: '出版社名称不能超过100个字符' }
            ]}
          >
            <Input
              placeholder="输入出版社名称（可选）"
              size="large"
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="页数"
            name="page_count"
            rules={[
              { type: 'number', min: 1, message: '页数必须大于0' }
            ]}
          >
            <InputNumber
              placeholder="输入书籍页数（可选）"
              size="large"
              style={{ width: '100%' }}
              min={1}
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="简介"
            name="description"
            rules={[
              { max: 1000, message: '简介不能超过1000个字符' }
            ]}
          >
            <TextArea
              placeholder="输入书籍简介（可选）"
              rows={4}
              showCount
              maxLength={1000}
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item style={{ marginBottom: 0 }}>
            <Space>
              <Button
                type="primary"
                htmlType="submit"
                icon={<SaveOutlined />}
                loading={isSubmitting}
                size="large"
              >
                {isEditing ? '更新书籍' : '添加书籍'}
              </Button>
              <Link to="/books">
                <Button size="large" disabled={isSubmitting}>
                  取消
                </Button>
              </Link>
            </Space>
          </Form.Item>
        </Form>
      </Card>
    </div>
  )
}