import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/react'
import LoadingSpinner from '../LoadingSpinner'

describe('LoadingSpinner', () => {
  it('renders with default props', () => {
    render(<LoadingSpinner />)
    
    const spinner = screen.getByRole('status', { hidden: true })
    expect(spinner).toBeInTheDocument()
  })

  it('renders with custom message', () => {
    const message = 'Loading data...'
    render(<LoadingSpinner message={message} />)
    
    expect(screen.getByText(message)).toBeInTheDocument()
  })

  it('applies correct size classes', () => {
    const { rerender } = render(<LoadingSpinner size="sm" />)
    let spinner = screen.getByRole('status', { hidden: true }).firstChild
    expect(spinner).toHaveClass('h-4', 'w-4')

    rerender(<LoadingSpinner size="md" />)
    spinner = screen.getByRole('status', { hidden: true }).firstChild
    expect(spinner).toHaveClass('h-8', 'w-8')

    rerender(<LoadingSpinner size="lg" />)
    spinner = screen.getByRole('status', { hidden: true }).firstChild
    expect(spinner).toHaveClass('h-12', 'w-12')
  })

  it('applies custom className', () => {
    const customClass = 'custom-spinner-class'
    render(<LoadingSpinner className={customClass} />)
    
    const container = screen.getByRole('status', { hidden: true })
    expect(container).toHaveClass(customClass)
  })

  it('has proper accessibility attributes', () => {
    render(<LoadingSpinner />)
    
    const spinner = screen.getByRole('status', { hidden: true })
    expect(spinner).toBeInTheDocument()
  })

  it('renders without message when not provided', () => {
    render(<LoadingSpinner />)
    
    const message = screen.queryByText(/loading/i)
    expect(message).not.toBeInTheDocument()
  })
})
