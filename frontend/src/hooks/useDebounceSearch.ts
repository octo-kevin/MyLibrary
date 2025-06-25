import { useState, useCallback, useRef } from 'react'

export function useDebounceSearch(onSearch: (value: string) => void, delay: number = 500) {
  const [inputValue, setInputValue] = useState('')
  const [isComposing, setIsComposing] = useState(false)
  const timeoutRef = useRef<number>()

  const handleInputChange = useCallback((value: string) => {
    setInputValue(value)
    
    // 如果正在中文输入，不触发搜索
    if (isComposing) {
      return
    }

    // 清除之前的定时器
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
    }

    // 设置新的防抖定时器
    timeoutRef.current = window.setTimeout(() => {
      onSearch(value)
    }, delay)
  }, [onSearch, delay, isComposing])

  const handleCompositionStart = useCallback(() => {
    setIsComposing(true)
  }, [])

  const handleCompositionEnd = useCallback((value: string) => {
    setIsComposing(false)
    
    // 中文输入结束后立即触发搜索（带防抖）
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
    }
    
    timeoutRef.current = window.setTimeout(() => {
      onSearch(value)
    }, delay)
  }, [onSearch, delay])

  const clear = useCallback(() => {
    setInputValue('')
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current)
    }
    onSearch('')
  }, [onSearch])

  return {
    inputValue,
    setInputValue,
    handleInputChange,
    handleCompositionStart,
    handleCompositionEnd,
    clear,
    isComposing
  }
}