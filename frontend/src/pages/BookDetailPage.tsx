import { useParams, Link } from 'react-router-dom'
import { useQuery } from '@tanstack/react-query'
import { 
  Button, 
  Card, 
  Typography, 
  Space, 
  Breadcrumb, 
  Descriptions, 
  Tag,
  List,
  Avatar,
  Empty
} from 'antd'
import {
  ArrowLeftOutlined,
  UserOutlined,
  CalendarOutlined,
  FileTextOutlined,
  PlusOutlined,
  BookOutlined,
  HomeOutlined,
  EditOutlined,
} from '@ant-design/icons'
import { booksAPI } from '../lib/api'
import { Loading, ErrorMessage } from '../components/ui'
import { formatDate } from '../lib/utils'

const { Title, Text, Paragraph } = Typography

export default function BookDetailPage() {
  const { id } = useParams<{ id: string }>()
  const bookId = parseInt(id || '0', 10)

  const { data: book, isLoading, error, refetch } = useQuery({
    queryKey: ['book', bookId],
    queryFn: () => booksAPI.getBook(bookId),
    enabled: !!bookId,
  })

  const { data: notesData } = useQuery({
    queryKey: ['book-notes', bookId],
    queryFn: () => booksAPI.getBookNotes(bookId, { page: 1, per_page: 5 }),
    enabled: !!bookId,
  })

  if (isLoading) {
    return <Loading size="large" text="加载书籍详情中..." />
  }

  if (error || !book) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载书籍详情，请稍后重试"
        onRetry={() => refetch()}
      />
    )
  }

  const notes = notesData?.notes || []

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
              书籍详情
            </Breadcrumb.Item>
          </Breadcrumb>
          
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <div>
              <Title level={3} style={{ margin: 0 }}>
                {book.title}
              </Title>
              <Text type="secondary">
                查看书籍详细信息和相关笔记
              </Text>
            </div>
            <Space>
              <Link to={`/books/${book.id}/edit`}>
                <Button icon={<EditOutlined />}>
                  编辑书籍
                </Button>
              </Link>
              <Link to="/books">
                <Button icon={<ArrowLeftOutlined />}>
                  返回列表
                </Button>
              </Link>
            </Space>
          </div>
        </Space>
      </Card>

      {/* 书籍信息 */}
      <Card title="书籍信息" style={{ marginBottom: 24 }}>
        <Descriptions column={{ xs: 1, sm: 2, md: 2 }} bordered>
          <Descriptions.Item label="书名" span={2}>
            <Text strong style={{ fontSize: 16 }}>{book.title}</Text>
          </Descriptions.Item>
          <Descriptions.Item label="作者">
            <Space>
              <UserOutlined />
              {book.author}
            </Space>
          </Descriptions.Item>
          <Descriptions.Item label="ISBN">
            <Text code>{book.isbn || '暂无'}</Text>
          </Descriptions.Item>
          <Descriptions.Item label="出版社">
            {book.publisher || '暂无'}
          </Descriptions.Item>
          <Descriptions.Item label="页数">
            {book.page_count ? `${book.page_count} 页` : '暂无'}
          </Descriptions.Item>
          <Descriptions.Item label="添加时间" span={2}>
            <Space>
              <CalendarOutlined />
              {formatDate(book.created_at)}
            </Space>
          </Descriptions.Item>
          {book.description && (
            <Descriptions.Item label="简介" span={2}>
              <Paragraph>{book.description}</Paragraph>
            </Descriptions.Item>
          )}
        </Descriptions>
      </Card>

      {/* 相关笔记 */}
      <Card 
        title="相关笔记"
        extra={
          <Link to={`/notes/new?book_id=${book.id}`}>
            <Button type="primary" icon={<PlusOutlined />} size="small">
              添加笔记
            </Button>
          </Link>
        }
      >
        {notes.length === 0 ? (
          <Empty
            image={<FileTextOutlined style={{ fontSize: 48, color: '#d9d9d9' }} />}
            description="还没有添加任何笔记"
          >
            <Link to={`/notes/new?book_id=${book.id}`}>
              <Button type="primary" icon={<PlusOutlined />}>
                添加第一条笔记
              </Button>
            </Link>
          </Empty>
        ) : (
          <>
            <List
              dataSource={notes}
              renderItem={(note) => (
                <List.Item
                  actions={[
                    <Link to={`/notes/${note.id}`} key="view">
                      查看详情
                    </Link>
                  ]}
                >
                  <List.Item.Meta
                    avatar={
                      <Avatar icon={<FileTextOutlined />} style={{ backgroundColor: '#1890ff' }} />
                    }
                    title={
                      <Space>
                        <Link to={`/notes/${note.id}`}>
                          {note.title || '无标题笔记'}
                        </Link>
                        <Tag color="blue">{note.note_type}</Tag>
                      </Space>
                    }
                    description={
                      <div>
                        <Paragraph ellipsis={{ rows: 2 }} style={{ margin: 0 }}>
                          {note.content}
                        </Paragraph>
                        <Text type="secondary" style={{ fontSize: 12 }}>
                          <CalendarOutlined style={{ marginRight: 4 }} />
                          {formatDate(note.created_at)}
                        </Text>
                      </div>
                    }
                  />
                </List.Item>
              )}
            />
            {notesData && notesData.total > 5 && (
              <div style={{ textAlign: 'center', marginTop: 16 }}>
                <Link to={`/notes?book_id=${book.id}`}>
                  <Button>查看所有笔记 ({notesData.total})</Button>
                </Link>
              </div>
            )}
          </>
        )}
      </Card>
    </div>
  )
}