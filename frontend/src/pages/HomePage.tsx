import { useEffect } from 'react'
import { useNavigate } from 'react-router-dom'

export default function HomePage() {
  const navigate = useNavigate()

  useEffect(() => {
    // Redirect to books page as the default landing page
    navigate('/books', { replace: true })
  }, [navigate])

  return null
}