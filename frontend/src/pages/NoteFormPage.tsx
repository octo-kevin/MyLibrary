import { useEffect } from 'react'
import { useParams, useNavigate, Link, useSearchParams } from 'react-router-dom'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
import { 
  Form, 
  Input, 
  Button, 
  Card, 
  Space, 
  Typography, 
  Select,
  Switch,
  InputNumber,
  message,
  Breadcrumb,
} from 'antd'
import {
  ArrowLeftOutlined,
  SaveOutlined,
  FileTextOutlined,
  HomeOutlined,
} from '@ant-design/icons'
import { notesAPI, booksAPI, type CreateNoteRequest } from '../lib/api'
import { Loading, ErrorMessage } from '../components/ui'

const { Title, Text } = Typography
const { TextArea } = Input

interface FormData {
  title?: string
  content: string
  note_type: 'quote' | 'summary' | 'thought' | 'general'
  book_id?: number
  page_reference?: number
  is_favorite: boolean
  tags: string[]
}

export default function NoteFormPage() {
  const { id } = useParams<{ id: string }>()
  const [searchParams] = useSearchParams()
  const navigate = useNavigate()
  const queryClient = useQueryClient()
  const [form] = Form.useForm()
  const isEditing = !!id
  const noteId = parseInt(id || '0', 10)
  const bookIdFromUrl = searchParams.get('book_id')

  // Fetch note data for editing
  const { data: note, isLoading, error } = useQuery({
    queryKey: ['note', noteId],
    queryFn: () => notesAPI.getNote(noteId),
    enabled: isEditing,
  })

  // Fetch books for selection
  const { data: booksData } = useQuery({
    queryKey: ['books', 'all'],
    queryFn: () => booksAPI.getBooks({ page: 1, per_page: 100 }),
  })

  // Load note data into form when editing
  useEffect(() => {
    if (note) {
      form.setFieldsValue({
        title: note.title || '',
        content: note.content,
        note_type: note.note_type,
        book_id: note.book?.id,
        page_reference: note.page_reference || undefined,
        is_favorite: note.is_favorite || false,
        tags: note.tags || [],
      })
    } else if (bookIdFromUrl) {
      form.setFieldsValue({
        book_id: parseInt(bookIdFromUrl, 10),
      })
    }
  }, [note, form, bookIdFromUrl])

  // Create mutation
  const createMutation = useMutation({
    mutationFn: (data: CreateNoteRequest) => notesAPI.createNote(data),
    onSuccess: () => {
      message.success('笔记创建成功！')
      queryClient.invalidateQueries({ queryKey: ['notes'] })
      navigate('/notes')
    },
    onError: (error: any) => {
      console.error('Create error:', error)
      message.error('创建失败，请稍后重试')
    },
  })

  // Update mutation
  const updateMutation = useMutation({
    mutationFn: (data: CreateNoteRequest) => notesAPI.updateNote(noteId, data),
    onSuccess: () => {
      message.success('笔记更新成功！')
      queryClient.invalidateQueries({ queryKey: ['notes'] })
      queryClient.invalidateQueries({ queryKey: ['note', noteId] })
      navigate('/notes')
    },
    onError: (error: any) => {
      console.error('Update error:', error)
      message.error('更新失败，请稍后重试')
    },
  })

  const handleSubmit = (values: FormData) => {
    const submitData: CreateNoteRequest = {
      title: values.title || null,
      content: values.content,
      note_type: values.note_type,
      book_id: values.book_id || null,
      page_reference: values.page_reference || null,
      is_favorite: values.is_favorite,
      tags: values.tags,
    }

    if (isEditing) {
      updateMutation.mutate(submitData)
    } else {
      createMutation.mutate(submitData)
    }
  }

  const isSubmitting = createMutation.isPending || updateMutation.isPending

  if (isLoading) {
    return <Loading size="large" text="加载笔记信息中..." />
  }

  if (error) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载笔记信息，请稍后重试"
        onRetry={() => window.location.reload()}
      />
    )
  }

  const books = booksData?.books || []

  return (
    <div>
      {/* 面包屑导航 */}
      <Card style={{ marginBottom: 24 }}>
        <Space direction="vertical" size="small" style={{ width: '100%' }}>
          <Breadcrumb>
            <Breadcrumb.Item>
              <Link to="/notes">
                <HomeOutlined style={{ marginRight: 4 }} />
                读书笔记
              </Link>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <FileTextOutlined style={{ marginRight: 4 }} />
              {isEditing ? '编辑笔记' : '添加笔记'}
            </Breadcrumb.Item>
          </Breadcrumb>
          
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <div>
              <Title level={3} style={{ margin: 0 }}>
                {isEditing ? '编辑笔记' : '添加笔记'}
              </Title>
              <Text type="secondary">
                {isEditing ? '修改笔记内容' : '记录您的阅读思考和感悟'}
              </Text>
            </div>
            <Link to="/notes">
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
          style={{ maxWidth: 800 }}
          initialValues={{
            note_type: 'general',
            is_favorite: false,
            tags: [],
          }}
        >
          <Form.Item
            label="笔记标题"
            name="title"
            rules={[
              { max: 200, message: '标题不能超过200个字符' }
            ]}
          >
            <Input
              placeholder="输入笔记标题（可选）"
              size="large"
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="笔记内容"
            name="content"
            rules={[
              { required: true, message: '请输入笔记内容' },
              { max: 10000, message: '内容不能超过10000个字符' }
            ]}
          >
            <TextArea
              placeholder="输入您的读书笔记..."
              rows={8}
              showCount
              maxLength={10000}
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="笔记类型"
            name="note_type"
            rules={[{ required: true, message: '请选择笔记类型' }]}
          >
            <Select size="large" disabled={isSubmitting}>
              <Select.Option value="quote">摘录</Select.Option>
              <Select.Option value="summary">总结</Select.Option>
              <Select.Option value="thought">感想</Select.Option>
              <Select.Option value="general">一般</Select.Option>
            </Select>
          </Form.Item>

          <Form.Item
            label="关联书籍"
            name="book_id"
          >
            <Select
              placeholder="选择关联书籍（可选）"
              size="large"
              allowClear
              disabled={isSubmitting}
              showSearch
              filterOption={(input, option) =>
                (option?.children as string)?.toLowerCase().includes(input.toLowerCase())
              }
            >
              {books.map(book => (
                <Select.Option key={book.id} value={book.id}>
                  {book.title} - {book.author}
                </Select.Option>
              ))}
            </Select>
          </Form.Item>

          <Form.Item
            label="页码引用"
            name="page_reference"
            rules={[
              { type: 'number', min: 1, message: '页码必须大于0' }
            ]}
          >
            <InputNumber
              placeholder="输入页码（可选）"
              size="large"
              style={{ width: '100%' }}
              min={1}
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="标记为收藏"
            name="is_favorite"
            valuePropName="checked"
          >
            <Switch disabled={isSubmitting} />
          </Form.Item>

          <Form.Item
            label="标签"
            name="tags"
            extra="按回车键添加标签"
          >
            <Select
              mode="tags"
              placeholder="添加标签..."
              size="large"
              disabled={isSubmitting}
              style={{ width: '100%' }}
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
                {isEditing ? '更新笔记' : '保存笔记'}
              </Button>
              <Link to="/notes">
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