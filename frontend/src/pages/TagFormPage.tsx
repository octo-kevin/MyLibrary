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
  message,
  Breadcrumb,
} from 'antd'
import {
  ArrowLeftOutlined,
  SaveOutlined,
  TagsOutlined,
  HomeOutlined,
} from '@ant-design/icons'
import { tagsAPI, type CreateTagRequest } from '../lib/api'
import { Loading, ErrorMessage } from '../components/ui'

const { Title, Text } = Typography
const { TextArea } = Input

interface FormData {
  name: string
  description?: string
}

export default function TagFormPage() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const queryClient = useQueryClient()
  const [form] = Form.useForm()
  const isEditing = !!id
  const tagId = parseInt(id || '0', 10)

  // Fetch tag data for editing
  const { data: tag, isLoading, error } = useQuery({
    queryKey: ['tag', tagId],
    queryFn: () => tagsAPI.getTag(tagId),
    enabled: isEditing,
  })

  // Load tag data into form when editing
  useEffect(() => {
    if (tag) {
      form.setFieldsValue({
        name: tag.name,
        description: tag.description || '',
      })
    }
  }, [tag, form])

  // Create mutation
  const createMutation = useMutation({
    mutationFn: (data: CreateTagRequest) => tagsAPI.createTag(data),
    onSuccess: () => {
      message.success('标签创建成功！')
      queryClient.invalidateQueries({ queryKey: ['tags'] })
      navigate('/tags')
    },
    onError: (error: any) => {
      console.error('Create error:', error)
      message.error('创建失败，请稍后重试')
    },
  })

  // Update mutation
  const updateMutation = useMutation({
    mutationFn: (data: CreateTagRequest) => tagsAPI.updateTag(tagId, data),
    onSuccess: () => {
      message.success('标签更新成功！')
      queryClient.invalidateQueries({ queryKey: ['tags'] })
      queryClient.invalidateQueries({ queryKey: ['tag', tagId] })
      navigate('/tags')
    },
    onError: (error: any) => {
      console.error('Update error:', error)
      message.error('更新失败，请稍后重试')
    },
  })

  const handleSubmit = (values: FormData) => {
    const submitData: CreateTagRequest = {
      name: values.name,
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
    return <Loading size="large" text="加载标签信息中..." />
  }

  if (error) {
    return (
      <ErrorMessage
        title="加载失败"
        message="无法加载标签信息，请稍后重试"
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
              <Link to="/tags">
                <HomeOutlined style={{ marginRight: 4 }} />
                标签管理
              </Link>
            </Breadcrumb.Item>
            <Breadcrumb.Item>
              <TagsOutlined style={{ marginRight: 4 }} />
              {isEditing ? '编辑标签' : '添加标签'}
            </Breadcrumb.Item>
          </Breadcrumb>
          
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
            <div>
              <Title level={3} style={{ margin: 0 }}>
                {isEditing ? '编辑标签' : '添加标签'}
              </Title>
              <Text type="secondary">
                {isEditing ? '修改标签信息' : '创建一个新的标签用于分类笔记'}
              </Text>
            </div>
            <Link to="/tags">
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
            label="标签名称"
            name="name"
            rules={[
              { required: true, message: '请输入标签名称' },
              { max: 50, message: '标签名称不能超过50个字符' }
            ]}
          >
            <Input
              placeholder="输入标签名称"
              size="large"
              disabled={isSubmitting}
            />
          </Form.Item>

          <Form.Item
            label="标签描述"
            name="description"
            rules={[
              { max: 200, message: '描述不能超过200个字符' }
            ]}
          >
            <TextArea
              placeholder="输入标签描述（可选）"
              rows={3}
              showCount
              maxLength={200}
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
                {isEditing ? '更新标签' : '创建标签'}
              </Button>
              <Link to="/tags">
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