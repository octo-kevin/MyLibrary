import { useState } from 'react'
import { Outlet, Link, useLocation } from 'react-router-dom'
import { Layout as AntLayout, Menu, Typography } from 'antd'
import {
  BookOutlined,
  FileTextOutlined,
  TagsOutlined,
  ReadOutlined,
} from '@ant-design/icons'

const { Sider, Content } = AntLayout
const { Title, Text } = Typography

const menuItems = [
  {
    key: '/books',
    icon: <BookOutlined />,
    label: <Link to="/books">书籍管理</Link>,
    title: '书籍管理',
  },
  {
    key: '/notes',
    icon: <FileTextOutlined />,
    label: <Link to="/notes">读书笔记</Link>,
    title: '读书笔记',
  },
  {
    key: '/tags',
    icon: <TagsOutlined />,
    label: <Link to="/tags">标签管理</Link>,
    title: '标签管理',
  },
]

export default function Layout() {
  const location = useLocation()
  const [collapsed, setCollapsed] = useState(false)

  // 获取当前路径对应的菜单键
  const selectedKey = menuItems.find(item => 
    location.pathname.startsWith(item.key)
  )?.key || location.pathname

  return (
    <AntLayout>
      {/* 侧边栏 */}
      <Sider
        collapsible
        collapsed={collapsed}
        onCollapse={setCollapsed}
        width={256}
        style={{
          overflow: 'auto',
          height: '100vh',
          position: 'fixed',
          left: 0,
          top: 0,
          bottom: 0,
        }}
      >
        {/* Logo */}
        <div style={{ 
          padding: '24px 16px', 
          textAlign: 'center',
          borderBottom: '1px solid #f0f0f0',
          background: '#fff'
        }}>
          <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
            <ReadOutlined style={{ fontSize: 24, color: '#1890ff', marginRight: collapsed ? 0 : 8 }} />
            {!collapsed && (
              <div>
                <Title level={4} style={{ margin: 0, color: '#1890ff' }}>
                  读书笔记
                </Title>
                <Text type="secondary" style={{ fontSize: 12 }}>
                  个人阅读管理系统
                </Text>
              </div>
            )}
          </div>
        </div>

        {/* 导航菜单 */}
        <Menu
          mode="inline"
          selectedKeys={[selectedKey]}
          items={menuItems}
          style={{
            height: 'calc(100% - 84px)',
            borderRight: 0,
            background: '#fafafa'
          }}
        />
      </Sider>

      {/* 主要内容区域 */}
      <AntLayout style={{ 
        marginLeft: collapsed ? 80 : 256,
        minHeight: '100vh'
      }}>
        <Content
          style={{
            padding: '24px',
            minHeight: '100vh',
            background: '#f5f5f5'
          }}
        >
          <Outlet />
        </Content>
      </AntLayout>
    </AntLayout>
  )
}