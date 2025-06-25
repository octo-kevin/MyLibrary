import { Spin, Typography } from 'antd'
import { LoadingOutlined } from '@ant-design/icons'

const { Text } = Typography

export interface LoadingProps {
  size?: 'small' | 'default' | 'large'
  text?: string
  className?: string
}

export default function Loading({ size = 'default', text, className }: LoadingProps) {
  const antIcon = <LoadingOutlined style={{ fontSize: size === 'small' ? 16 : size === 'large' ? 32 : 24 }} spin />

  return (
    <div 
      className={className}
      style={{ 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center', 
        justifyContent: 'center', 
        padding: '32px' 
      }}
    >
      <Spin indicator={antIcon} size={size} />
      {text && (
        <Text 
          type="secondary" 
          style={{ 
            marginTop: 16,
            fontSize: size === 'small' ? 12 : size === 'large' ? 16 : 14 
          }}
        >
          {text}
        </Text>
      )}
    </div>
  )
}