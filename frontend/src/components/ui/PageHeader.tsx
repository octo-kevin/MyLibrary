import { type ReactNode } from 'react'
import { Typography, Space } from 'antd'

const { Title, Text } = Typography

interface PageHeaderProps {
  title: string
  description?: string
  extra?: ReactNode
  children?: ReactNode
}

export default function PageHeader({ title, description, extra, children }: PageHeaderProps) {
  return (
    <div className="page-header">
      <div style={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'flex-start',
        marginBottom: children ? 16 : 0 
      }}>
        <div>
          <Title level={3} className="page-title">
            {title}
          </Title>
          {description && (
            <Text type="secondary" className="page-description">
              {description}
            </Text>
          )}
        </div>
        {extra && (
          <Space>
            {extra}
          </Space>
        )}
      </div>
      {children}
    </div>
  )
}