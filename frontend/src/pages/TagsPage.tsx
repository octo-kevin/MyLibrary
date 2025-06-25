import { useState } from 'react'
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query'
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
  Tabs,
  Badge,
  Tooltip,
  Modal,
  message,
} from 'antd'
import {
  PlusOutlined,
  SearchOutlined,
  TagsOutlined,
  TrophyOutlined,
  EditOutlined,
  DeleteOutlined,
  ExclamationCircleOutlined,
  EyeOutlined,
} from '@ant-design/icons'
import { tagsAPI, type PaginationParams } from '../lib/api'
import { PageHeader, Loading, ErrorMessage } from '../components/ui'

const { Text } = Typography
const { TabPane } = Tabs

export default function TagsPage() {
  const queryClient = useQueryClient()
  const [activeTab, setActiveTab] = useState<'all' | 'popular'>('all')
  const [searchQuery, setSearchQuery] = useState('')
  const [currentPage, setCurrentPage] = useState(1)

  const queryParams: PaginationParams = {
    page: currentPage,
    per_page: 50,
    ...(searchQuery && { search: searchQuery }),
  }

  const { data: allTagsData, isLoading: isLoadingAll, error: allTagsError, refetch: refetchAll } = useQuery({
    queryKey: ['tags', queryParams],
    queryFn: () => tagsAPI.getTags(queryParams),
    enabled: activeTab === 'all',
  })

  const { data: popularTags, isLoading: isLoadingPopular, error: popularTagsError, refetch: refetchPopular } = useQuery({
    queryKey: ['tags', 'popular'],
    queryFn: () => tagsAPI.getPopularTags(),
    enabled: activeTab === 'popular',
  })

  // Delete mutation
  const deleteMutation = useMutation({
    mutationFn: (id: number) => tagsAPI.deleteTag(id),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['tags'] })
      message.success('标签删除成功')
    },
    onError: (error) => {
      console.error('Delete error:', error)
      message.error('删除失败，请稍后重试')
    },
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

  const handleDeleteClick = (tag: { id: number; name: string }) => {
    Modal.confirm({
      title: '删除标签',
      icon: <ExclamationCircleOutlined />,
      content: (
        <div>
          <p>确定要删除标签 <Text strong>「{tag.name}」</Text> 吗？</p>
          <div style={{ 
            background: '#fff2f0', 
            border: '1px solid #ffccc7', 
            borderRadius: 6, 
            padding: 12, 
            marginTop: 12 
          }}>
            <Text type="warning" style={{ fontSize: 13 }}>
              ⚠️ 该标签将从所有相关笔记中移除，此操作不可恢复。
            </Text>
          </div>
        </div>
      ),
      okText: '确认删除',
      cancelText: '取消',
      okType: 'danger',
      onOk: () => deleteMutation.mutate(tag.id),
    })
  }

  const isLoading = activeTab === 'all' ? isLoadingAll : isLoadingPopular
  const error = activeTab === 'all' ? allTagsError : popularTagsError
  const refetch = activeTab === 'all' ? refetchAll : refetchPopular

  if (isLoading) {
    return <Loading size="large" text="加载标签中..." />
  }

  if (error) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载标签列表，请稍后重试"
        onRetry={() => refetch()}
      />
    )
  }

  const allTags = allTagsData?.tags || []
  const totalPages = allTagsData?.total_pages || 1
  const total = allTagsData?.total || 0

  // 表格列定义
  const columns = [
    {
      title: '标签名称',
      dataIndex: 'name',
      key: 'name',
      width: '30%',
      render: (name: string) => (
        <Space>
          <TagsOutlined style={{ color: '#1890ff' }} />
          <Text strong style={{ fontSize: 14 }}>{name}</Text>
        </Space>
      ),
    },
    {
      title: '标识符',
      dataIndex: 'slug',
      key: 'slug',
      width: '25%',
      render: (slug: string) => (
        <Text code style={{ fontSize: 12 }}>{slug}</Text>
      ),
    },
    {
      title: '使用次数',
      dataIndex: 'usage_count',
      key: 'usage_count',
      width: '15%',
      render: (count: number) => (
        <Badge count={count} style={{ backgroundColor: '#52c41a' }} />
      ),
    },
    {
      title: '创建时间',
      dataIndex: 'created_at',
      key: 'created_at',
      width: '20%',
      render: (date: string) => (
        <Text type="secondary" style={{ fontSize: 13 }}>
          {new Date(date).toLocaleDateString('zh-CN')}
        </Text>
      ),
    },
    {
      title: '操作',
      key: 'actions',
      width: '10%',
      render: (_: any, record: any) => (
        <Space>
          <Tooltip title="查看笔记">
            <Link to={`/notes?search=${record.name}`}>
              <Button type="text" icon={<EyeOutlined />} size="small" />
            </Link>
          </Tooltip>
          <Tooltip title="编辑">
            <Link to={`/tags/${record.id}/edit`}>
              <Button type="text" icon={<EditOutlined />} size="small" />
            </Link>
          </Tooltip>
          <Tooltip title="删除">
            <Button 
              type="text" 
              danger 
              icon={<DeleteOutlined />} 
              size="small"
              onClick={() => handleDeleteClick(record)}
              loading={deleteMutation.isPending}
            />
          </Tooltip>
        </Space>
      ),
    },
  ]

  return (
    <div>
      <PageHeader
        title="标签管理"
        description={`管理您的笔记标签，共 ${total} 个标签`}
        extra={
          <Link to="/tags/new">
            <Button type="primary" icon={<PlusOutlined />} size="large">
              添加标签
            </Button>
          </Link>
        }
      />

      {/* 标签页切换 */}
      <Card style={{ marginBottom: 24 }}>
        <Tabs 
          activeKey={activeTab} 
          onChange={(key) => setActiveTab(key as 'all' | 'popular')}
          items={[
            {
              key: 'all',
              label: (
                <span>
                  <TagsOutlined />
                  全部标签
                </span>
              ),
            },
            {
              key: 'popular',
              label: (
                <span>
                  <TrophyOutlined />
                  热门标签
                </span>
              ),
            },
          ]}
        />

        {/* 搜索栏 (仅在全部标签页显示) */}
        {activeTab === 'all' && (
          <div style={{ marginTop: 16 }}>
            <Input.Search
              placeholder="搜索标签名称..."
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
          </div>
        )}
      </Card>

      {/* 内容区域 */}
      <Card>
        {activeTab === 'all' ? (
          // 全部标签表格视图
          allTags.length === 0 ? (
            <Empty
              image={<TagsOutlined style={{ fontSize: 64, color: '#d9d9d9' }} />}
              description={
                <div>
                  <Text style={{ fontSize: 16, display: 'block', marginBottom: 8 }}>
                    {searchQuery ? '没有找到相关标签' : '还没有创建任何标签'}
                  </Text>
                  <Text type="secondary">
                    {searchQuery ? '尝试调整搜索关键词' : '标签会在您添加笔记时自动创建，也可以手动创建'}
                  </Text>
                </div>
              }
            >
              {!searchQuery && (
                <Link to="/tags/new">
                  <Button type="primary" icon={<PlusOutlined />}>
                    创建第一个标签
                  </Button>
                </Link>
              )}
            </Empty>
          ) : (
            <>
              <Table
                columns={columns}
                dataSource={allTags}
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
                    pageSize={50}
                    onChange={setCurrentPage}
                    showSizeChanger={false}
                    showQuickJumper
                  />
                </div>
              )}
            </>
          )
        ) : (
          // 热门标签视图
          popularTags && popularTags.length === 0 ? (
            <Empty
              image={<TrophyOutlined style={{ fontSize: 64, color: '#d9d9d9' }} />}
              description={
                <div>
                  <Text style={{ fontSize: 16, display: 'block', marginBottom: 8 }}>
                    暂无热门标签
                  </Text>
                  <Text type="secondary">
                    添加更多笔记和标签来查看热门趋势，发现您的知识脉络
                  </Text>
                </div>
              }
            />
          ) : (
            <div>
              {/* 标签云 */}
              <div style={{ marginBottom: 32 }}>
                <Text strong style={{ fontSize: 16, display: 'block', marginBottom: 16 }}>
                  <TagsOutlined style={{ marginRight: 8 }} />
                  标签云
                </Text>
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: 12 }}>
                  {popularTags?.map((tag) => {
                    // 根据使用次数计算字体大小
                    const maxUsage = Math.max(...(popularTags?.map(t => t.usage_count) || [1]))
                    const minSize = 14
                    const maxSize = 24
                    const fontSize = minSize + (tag.usage_count / maxUsage) * (maxSize - minSize)
                    
                    return (
                      <Tooltip key={tag.id} title={`${tag.usage_count} 次使用`}>
                        <Tag
                          style={{ 
                            fontSize: `${fontSize}px`,
                            padding: '8px 16px',
                            borderRadius: 20,
                            cursor: 'pointer',
                            transition: 'all 0.3s ease'
                          }}
                          color="blue"
                        >
                          {tag.name}
                        </Tag>
                      </Tooltip>
                    )
                  })}
                </div>
              </div>

              {/* 使用排行 */}
              <div>
                <Text strong style={{ fontSize: 16, display: 'block', marginBottom: 16 }}>
                  <TrophyOutlined style={{ marginRight: 8 }} />
                  使用排行
                </Text>
                <div style={{ display: 'grid', gap: 12 }}>
                  {popularTags?.slice(0, 10).map((tag, index) => (
                    <Card key={tag.id} size="small" style={{ borderRadius: 8 }}>
                      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                        <div style={{ display: 'flex', alignItems: 'center' }}>
                          <Badge 
                            count={index + 1} 
                            style={{ 
                              backgroundColor: index === 0 ? '#faad14' : index === 1 ? '#d9d9d9' : index === 2 ? '#d46b08' : '#1890ff',
                              marginRight: 12 
                            }} 
                          />
                          <TagsOutlined style={{ color: '#8c8c8c', marginRight: 8 }} />
                          <Text strong style={{ fontSize: 14 }}>{tag.name}</Text>
                        </div>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
                          <Text type="secondary" style={{ fontSize: 13 }}>
                            {tag.usage_count} 次使用
                          </Text>
                          <Link to={`/notes?search=${tag.name}`}>
                            <Button type="primary" size="small">
                              查看笔记
                            </Button>
                          </Link>
                        </div>
                      </div>
                    </Card>
                  ))}
                </div>
              </div>
            </div>
          )
        )}
      </Card>
    </div>
  )
}