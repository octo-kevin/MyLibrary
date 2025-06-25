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
  Avatar,
  Tooltip,
} from 'antd'
import {
  ArrowLeftOutlined,
  FileTextOutlined,
  BookOutlined,
  HomeOutlined,
  EditOutlined,
  StarFilled,
  StarOutlined,
  CalendarOutlined,
  TagsOutlined,
} from '@ant-design/icons'
import { notesAPI } from '../lib/api'
import { Loading, ErrorMessage } from '../components/ui'
import { formatDate } from '../lib/utils'

const { Title, Text, Paragraph } = Typography

export default function NoteDetailPage() {
  const { id } = useParams<{ id: string }>()
  const noteId = parseInt(id || '0', 10)

  const { data: note, isLoading, error, refetch } = useQuery({
    queryKey: ['note', noteId],
    queryFn: () => notesAPI.getNote(noteId),
    enabled: !!noteId,
  })

  if (isLoading) {
    return <Loading size="large" text="加载笔记详情中..." />
  }

  if (error || !note) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载笔记详情，请稍后重试"
        onRetry={() => refetch()}
      />
    )
  }

  const getNoteTypeColor = (type: string) => {
    switch (type) {
      case 'quote': return 'blue'
      case 'summary': return 'green'
      case 'thought': return 'purple'
      default: return 'default'
    }
  }

  const getNoteTypeName = (type: string) => {
    switch (type) {
      case 'quote': return '摘录'
      case 'summary': return '总结'
      case 'thought': return '感想'
      default: return '笔记'
    }
  }

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
              笔记详情
            </Breadcrumb.Item>
          </Breadcrumb>
          
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <div style={{ flex: 1 }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 4 }}>
                <Title level={3} style={{ margin: 0 }}>
                  {note.title || '无标题笔记'}
                </Title>
                {note.is_favorite && (
                  <StarFilled style={{ color: '#faad14', fontSize: 20 }} />
                )}
              </div>
              <Space>
                <Tag color={getNoteTypeColor(note.note_type)}>
                  {getNoteTypeName(note.note_type)}
                </Tag>
                <Text type="secondary">
                  <CalendarOutlined style={{ marginRight: 4 }} />
                  {formatDate(note.created_at)}
                </Text>
              </Space>
            </div>
            <Space>
              <Link to={`/notes/${note.id}/edit`}>
                <Button icon={<EditOutlined />}>
                  编辑笔记
                </Button>
              </Link>
              <Link to="/notes">
                <Button icon={<ArrowLeftOutlined />}>
                  返回列表
                </Button>
              </Link>
            </Space>
          </div>
        </Space>
      </Card>

      {/* 笔记内容 */}
      <Card title="笔记内容" style={{ marginBottom: 24 }}>
        <Paragraph style={{ fontSize: 16, lineHeight: 1.8, whiteSpace: 'pre-wrap' }}>
          {note.content}
        </Paragraph>
      </Card>

      {/* 笔记信息 */}
      <Card title="笔记信息" style={{ marginBottom: 24 }}>
        <Descriptions column={{ xs: 1, sm: 2, md: 2 }} bordered>
          <Descriptions.Item label="笔记类型">
            <Tag color={getNoteTypeColor(note.note_type)}>
              {getNoteTypeName(note.note_type)}
            </Tag>
          </Descriptions.Item>
          <Descriptions.Item label="收藏状态">
            <Space>
              {note.is_favorite ? (
                <>
                  <StarFilled style={{ color: '#faad14' }} />
                  <Text>已收藏</Text>
                </>
              ) : (
                <>
                  <StarOutlined style={{ color: '#d9d9d9' }} />
                  <Text type="secondary">未收藏</Text>
                </>
              )}
            </Space>
          </Descriptions.Item>
          {note.book && (
            <>
              <Descriptions.Item label="关联书籍">
                <Space>
                  <BookOutlined style={{ color: '#1890ff' }} />
                  <Link to={`/books/${note.book.id}`}>
                    {note.book.title}
                  </Link>
                </Space>
              </Descriptions.Item>
              <Descriptions.Item label="作者">
                {note.book.author}
              </Descriptions.Item>
            </>
          )}
          {note.page_reference && (
            <Descriptions.Item label="页码引用">
              第 {note.page_reference} 页
            </Descriptions.Item>
          )}
          <Descriptions.Item label="创建时间">
            <Space>
              <CalendarOutlined />
              {formatDate(note.created_at)}
            </Space>
          </Descriptions.Item>
          {note.updated_at && note.updated_at !== note.created_at && (
            <Descriptions.Item label="更新时间">
              <Space>
                <CalendarOutlined />
                {formatDate(note.updated_at)}
              </Space>
            </Descriptions.Item>
          )}
        </Descriptions>
      </Card>

      {/* 标签 */}
      {note.tags && note.tags.length > 0 && (
        <Card title="标签" extra={<TagsOutlined />}>
          <Space wrap>
            {note.tags.map((tag, index) => (
              <Tag 
                key={index} 
                style={{ 
                  padding: '4px 12px',
                  fontSize: 14,
                  borderRadius: 16
                }}
              >
                {tag}
              </Tag>
            ))}
          </Space>
        </Card>
      )}
    </div>
  )
}