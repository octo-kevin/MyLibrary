import { useState, useEffect } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Link, useSearchParams } from 'react-router-dom'
import { useDebounceSearch } from '../hooks/useDebounceSearch'
import { 
  Button, 
  Input, 
  Card, 
  Space, 
  Tag, 
  Typography,
  Pagination,
  Empty,
  Row,
  Col,
  Badge,
  Tooltip,
  Avatar
} from 'antd'
import {
  PlusOutlined,
  SearchOutlined,
  FileTextOutlined,
  BookOutlined,
  CalendarOutlined,
  EditOutlined,
  EyeOutlined,
  StarFilled,
  TagsOutlined,
} from '@ant-design/icons'
import { notesAPI, type PaginationParams } from '../lib/api'
import { PageHeader, Loading, ErrorMessage } from '../components/ui'
import { formatDate } from '../lib/utils'

const { Text, Paragraph } = Typography

export default function NotesPage() {
  const [searchParams] = useSearchParams()
  const [searchQuery, setSearchQuery] = useState('')
  const [currentPage, setCurrentPage] = useState(1)
  const [selectedType, setSelectedType] = useState<string>('all')

  // Initialize search query from URL params
  useEffect(() => {
    const urlSearch = searchParams.get('search')
    if (urlSearch) {
      setSearchQuery(urlSearch)
    }
  }, [searchParams])

  const queryParams: PaginationParams = {
    page: currentPage,
    per_page: 20,
    ...(searchQuery && { search: searchQuery }),
    ...(selectedType !== 'all' && { note_type: selectedType }),
  }

  const { data, isLoading, error, refetch } = useQuery({
    queryKey: ['notes', queryParams],
    queryFn: () => notesAPI.getNotes(queryParams),
  })

  const handleSearch = (value: string) => {
    setSearchQuery(value)
    setCurrentPage(1)
  }

  const {
    inputValue,
    handleInputChange,
    handleCompositionStart,
    handleCompositionEnd,
    clear
  } = useDebounceSearch(handleSearch, 300)

  if (isLoading) {
    return <Loading size="large" text="加载笔记列表中..." />
  }

  if (error) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载笔记列表，请稍后重试"
        onRetry={() => refetch()}
      />
    )
  }

  const notes = data?.notes || []
  const totalPages = data?.total_pages || 1
  const total = data?.total || 0

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

  const noteTypes = [
    { key: 'all', label: '全部', color: 'default' },
    { key: 'quote', label: '摘录', color: 'blue' },
    { key: 'summary', label: '总结', color: 'green' },
    { key: 'thought', label: '感想', color: 'purple' },
    { key: 'general', label: '一般', color: 'default' },
  ]

  return (
    <div>
      <PageHeader
        title="读书笔记"
        description={`记录您的阅读思考，共 ${total} 条笔记`}
        extra={
          <Link to="/notes/new">
            <Button type="primary" icon={<PlusOutlined />} size="large">
              添加笔记
            </Button>
          </Link>
        }
      >
        {/* 搜索和筛选 */}
        <Card>
          <Space direction="vertical" size="large" style={{ width: '100%' }}>
            {/* 搜索框 */}
            <Input.Search
              placeholder="搜索笔记标题或内容..."
              size="large"
              prefix={<SearchOutlined />}
              value={inputValue}
              onChange={(e) => handleInputChange(e.target.value)}
              onCompositionStart={handleCompositionStart}
              onCompositionEnd={(e) => handleCompositionEnd(e.currentTarget.value)}
              onClear={clear}
              style={{ maxWidth: 500 }}
              allowClear
            />
            
            {/* 笔记类型筛选 */}
            <div>
              <Space wrap>
                <Text strong style={{ marginRight: 8 }}>
                  <TagsOutlined style={{ marginRight: 4 }} />
                  笔记类型：
                </Text>
                {noteTypes.map(type => (
                  <Tag.CheckableTag
                    key={type.key}
                    checked={selectedType === type.key}
                    onChange={() => {
                      setSelectedType(type.key)
                      setCurrentPage(1)
                    }}
                    style={{ 
                      padding: '4px 12px',
                      borderRadius: 16,
                      fontSize: 13
                    }}
                  >
                    {type.label}
                  </Tag.CheckableTag>
                ))}
              </Space>
            </div>
          </Space>
        </Card>
      </PageHeader>

      {/* 笔记列表 */}
      <Card>
        {notes.length === 0 ? (
          <Empty
            image={<FileTextOutlined style={{ fontSize: 64, color: '#d9d9d9' }} />}
            description={
              <div>
                <Text style={{ fontSize: 16, display: 'block', marginBottom: 8 }}>
                  {searchQuery ? '没有找到相关笔记' : '还没有添加任何笔记'}
                </Text>
                <Text type="secondary">
                  {searchQuery ? '尝试调整搜索关键词' : '开始记录您的阅读思考，留下珍贵的智慧足迹'}
                </Text>
              </div>
            }
          >
            {!searchQuery && (
              <Link to="/notes/new">
                <Button type="primary" icon={<PlusOutlined />}>
                  写下第一条笔记
                </Button>
              </Link>
            )}
          </Empty>
        ) : (
          <>
            {/* 笔记网格 */}
            <Row gutter={[16, 16]}>
              {notes.map((note) => (
                <Col key={note.id} xs={24} sm={12} lg={8} xl={6}>
                  <Card
                    size="small"
                    hoverable
                    style={{ height: '100%' }}
                    actions={[
                      <Tooltip title="查看详情">
                        <Link to={`/notes/${note.id}`}>
                          <EyeOutlined />
                        </Link>
                      </Tooltip>,
                      <Tooltip title="编辑">
                        <Link to={`/notes/${note.id}/edit`}>
                          <EditOutlined />
                        </Link>
                      </Tooltip>,
                    ]}
                  >
                    {/* 笔记头部 */}
                    <div style={{ marginBottom: 12 }}>
                      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 8 }}>
                        <Tag color={getNoteTypeColor(note.note_type)} style={{ margin: 0 }}>
                          {getNoteTypeName(note.note_type)}
                        </Tag>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                          {note.is_favorite && (
                            <StarFilled style={{ color: '#faad14', fontSize: 14 }} />
                          )}
                          <Text type="secondary" style={{ fontSize: 12 }}>
                            <CalendarOutlined style={{ marginRight: 4 }} />
                            {formatDate(note.created_at)}
                          </Text>
                        </div>
                      </div>
                    </div>

                    {/* 笔记标题 */}
                    {note.title && (
                      <Link to={`/notes/${note.id}`}>
                        <Text strong style={{ fontSize: 14, color: '#1890ff', display: 'block', marginBottom: 8 }}>
                          {note.title}
                        </Text>
                      </Link>
                    )}

                    {/* 笔记内容预览 */}
                    <Paragraph
                      ellipsis={{ rows: 3 }}
                      style={{ 
                        fontSize: 13, 
                        color: '#595959', 
                        marginBottom: 12,
                        minHeight: 60
                      }}
                    >
                      {note.content}
                    </Paragraph>

                    {/* 书籍信息 */}
                    {note.book && (
                      <div style={{ 
                        background: '#fafafa', 
                        padding: 8, 
                        borderRadius: 6, 
                        marginBottom: 8,
                        display: 'flex',
                        alignItems: 'center'
                      }}>
                        <BookOutlined style={{ color: '#1890ff', marginRight: 6, fontSize: 12 }} />
                        <Link to={`/books/${note.book.id}`} style={{ flex: 1 }}>
                          <Text style={{ fontSize: 12 }} ellipsis>
                            {note.book.title}
                          </Text>
                        </Link>
                        {note.page_reference && (
                          <Badge 
                            count={`P${note.page_reference}`} 
                            style={{ 
                              backgroundColor: '#f0f0f0', 
                              color: '#666',
                              fontSize: 10,
                              height: 18,
                              lineHeight: '18px'
                            }} 
                          />
                        )}
                      </div>
                    )}

                    {/* 标签 */}
                    {note.tags && note.tags.length > 0 && (
                      <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
                        {note.tags.slice(0, 3).map((tag, index) => (
                          <Tag key={index} size="small" style={{ fontSize: 11, margin: 0 }}>
                            {tag}
                          </Tag>
                        ))}
                        {note.tags.length > 3 && (
                          <Text type="secondary" style={{ fontSize: 11 }}>
                            +{note.tags.length - 3}
                          </Text>
                        )}
                      </div>
                    )}
                  </Card>
                </Col>
              ))}
            </Row>

            {/* 分页 */}
            {totalPages > 1 && (
              <div style={{ 
                marginTop: 24, 
                display: 'flex', 
                justifyContent: 'space-between', 
                alignItems: 'center',
                flexWrap: 'wrap',
                gap: 16
              }}>
                <Text type="secondary">
                  第 <Text strong>{currentPage}</Text> 页，共 <Text strong>{totalPages}</Text> 页
                </Text>
                <Pagination
                  current={currentPage}
                  total={total}
                  pageSize={20}
                  onChange={setCurrentPage}
                  showSizeChanger={false}
                  showQuickJumper
                />
              </div>
            )}
          </>
        )}
      </Card>
    </div>
  )
}