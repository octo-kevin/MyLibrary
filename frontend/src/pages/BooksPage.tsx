import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Link } from 'react-router-dom'
import { useDebounceSearch } from '../hooks/useDebounceSearch'
import { 
  Button, 
  Input, 
  Table, 
  Card, 
  Space, 
  Tag, 
  Typography,
  Pagination,
  Empty,
  Avatar,
  Tooltip
} from 'antd'
import {
  PlusOutlined,
  SearchOutlined,
  BookOutlined,
  UserOutlined,
  CalendarOutlined,
  EditOutlined,
  EyeOutlined,
} from '@ant-design/icons'
import { booksAPI, type PaginationParams } from '../lib/api'
import { PageHeader, Loading, ErrorMessage } from '../components/ui'
import { formatDate } from '../lib/utils'

const { Text } = Typography

export default function BooksPage() {
  const [searchQuery, setSearchQuery] = useState('')
  const [currentPage, setCurrentPage] = useState(1)

  const queryParams: PaginationParams = {
    page: currentPage,
    per_page: 20,
    ...(searchQuery && { search: searchQuery }),
  }

  const { data, isLoading, error, refetch } = useQuery({
    queryKey: ['books', queryParams],
    queryFn: () => booksAPI.getBooks(queryParams),
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
    return <Loading size="large" text="加载书籍列表中..." />
  }

  if (error) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载书籍列表，请稍后重试"
        onRetry={() => refetch()}
      />
    )
  }

  const books = data?.books || []
  const totalPages = data?.total_pages || 1
  const total = data?.total || 0

  // 表格列定义
  const columns = [
    {
      title: '书籍信息',
      dataIndex: 'title',
      key: 'title',
      width: '40%',
      render: (title: string, record: any) => (
        <div style={{ display: 'flex', alignItems: 'center' }}>
          <Avatar 
            size={48}
            icon={<BookOutlined />} 
            style={{ 
              backgroundColor: '#1890ff', 
              marginRight: 12,
              flexShrink: 0
            }} 
          />
          <div style={{ minWidth: 0, flex: 1 }}>
            <Link to={`/books/${record.id}`}>
              <Text strong style={{ fontSize: 16, color: '#1890ff' }}>
                {title}
              </Text>
            </Link>
            {record.description && (
              <div style={{ marginTop: 4 }}>
                <Text type="secondary" style={{ fontSize: 13 }}>
                  {record.description.length > 100
                    ? `${record.description.slice(0, 100)}...`
                    : record.description}
                </Text>
              </div>
            )}
          </div>
        </div>
      ),
    },
    {
      title: '作者',
      dataIndex: 'author',
      key: 'author',
      width: '20%',
      render: (author: string) => (
        <Space>
          <UserOutlined style={{ color: '#8c8c8c' }} />
          <Text>{author}</Text>
        </Space>
      ),
    },
    {
      title: 'ISBN',
      dataIndex: 'isbn',
      key: 'isbn',
      width: '15%',
      render: (isbn: string) => (
        <Text code style={{ fontSize: 12 }}>
          {isbn || '—'}
        </Text>
      ),
    },
    {
      title: '添加时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: '15%',
      render: (date: string) => (
        <Space>
          <CalendarOutlined style={{ color: '#8c8c8c' }} />
          <Text type="secondary" style={{ fontSize: 13 }}>
            {formatDate(date)}
          </Text>
        </Space>
      ),
    },
    {
      title: '操作',
      key: 'actions',
      width: '10%',
      render: (_: any, record: any) => (
        <Space>
          <Tooltip title="查看详情">
            <Link to={`/books/${record.id}`}>
              <Button type="text" icon={<EyeOutlined />} size="small" />
            </Link>
          </Tooltip>
          <Tooltip title="编辑">
            <Link to={`/books/${record.id}/edit`}>
              <Button type="text" icon={<EditOutlined />} size="small" />
            </Link>
          </Tooltip>
        </Space>
      ),
    },
  ]

  return (
    <div>
      <PageHeader
        title="书籍管理"
        description={`管理您的个人藏书，共 ${total} 本书籍`}
        extra={
          <Link to="/books/new">
            <Button type="primary" icon={<PlusOutlined />} size="large">
              添加书籍
            </Button>
          </Link>
        }
      >
        {/* 搜索栏 */}
        <Card style={{ marginBottom: 0 }}>
          <Input.Search
            placeholder="搜索书名或作者..."
            size="large"
            prefix={<SearchOutlined />}
            value={inputValue}
            onChange={(e) => handleInputChange(e.target.value)}
            onCompositionStart={handleCompositionStart}
            onCompositionEnd={(e) => handleCompositionEnd(e.currentTarget.value)}
            onClear={clear}
            style={{ maxWidth: 400 }}
            allowClear
          />
        </Card>
      </PageHeader>

      {/* 书籍列表 */}
      <Card>
        {books.length === 0 ? (
          <Empty
            image={<BookOutlined style={{ fontSize: 64, color: '#d9d9d9' }} />}
            description={
              <div>
                <Text style={{ fontSize: 16, display: 'block', marginBottom: 8 }}>
                  {searchQuery ? '没有找到相关书籍' : '还没有添加任何书籍'}
                </Text>
                <Text type="secondary">
                  {searchQuery ? '尝试调整搜索关键词' : '开始建立您的个人图书馆，记录阅读足迹'}
                </Text>
              </div>
            }
          >
            {!searchQuery && (
              <Link to="/books/new">
                <Button type="primary" icon={<PlusOutlined />}>
                  添加第一本书籍
                </Button>
              </Link>
            )}
          </Empty>
        ) : (
          <>
            <Table
              columns={columns}
              dataSource={books}
              rowKey="id"
              pagination={false}
              scroll={{ x: 800 }}
              rowHoverable
            />
            
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