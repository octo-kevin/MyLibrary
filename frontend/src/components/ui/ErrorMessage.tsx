import { Button, Typography, Space } from 'antd'
import { ExclamationCircleOutlined, ReloadOutlined } from '@ant-design/icons'

const { Title, Text } = Typography

interface ErrorMessageProps {
  title?: string
  message: string
  onRetry?: () => void
  retryText?: string
}

export default function ErrorMessage({ 
  title = '出错了', 
  message, 
  onRetry, 
  retryText = '重试' 
}: ErrorMessageProps) {
  return (
    <div style={{ 
      display: 'flex', 
      flexDirection: 'column', 
      alignItems: 'center', 
      justifyContent: 'center', 
      padding: '48px 24px',
      textAlign: 'center'
    }}>
      <Space direction="vertical" size="large" align="center">
        <ExclamationCircleOutlined 
          style={{ 
            fontSize: 48, 
            color: '#ff4d4f' 
          }} 
        />
        <div>
          <Title level={4} style={{ margin: '0 0 8px 0', color: '#262626' }}>
            {title}
          </Title>
          <Text type="secondary" style={{ fontSize: 14, maxWidth: 400, display: 'block' }}>
            {message}
          </Text>
        </div>
        {onRetry && (
          <Button 
            type="default" 
            icon={<ReloadOutlined />} 
            onClick={onRetry}
          >
            {retryText}
          </Button>
        )}
      </Space>
    </div>
  )
}