# Developer Guide

## Overview

This guide provides comprehensive information for developers working on the Free Deep Research System. It covers setup, architecture, development workflows, and best practices.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Architecture Overview](#architecture-overview)
3. [Development Environment](#development-environment)
4. [Project Structure](#project-structure)
5. [Development Workflow](#development-workflow)
6. [Testing Strategy](#testing-strategy)
7. [Performance Guidelines](#performance-guidelines)
8. [Security Guidelines](#security-guidelines)
9. [Deployment](#deployment)
10. [Troubleshooting](#troubleshooting)

## Quick Start

### Prerequisites

- **Node.js**: v18.0.0 or higher
- **Rust**: Latest stable version
- **Git**: Latest version
- **VS Code**: Recommended IDE with extensions

### Setup

```bash
# Clone the repository
git clone https://github.com/your-org/free-deep-research.git
cd free-deep-research

# Install dependencies
npm install

# Install Rust dependencies
cd src-tauri && cargo build && cd ..

# Set up environment
npm run setup:dev

# Start development server
npm run dev
```

### First Run

1. **Configure API Keys**: Add your API keys in the settings panel
2. **Test System Health**: Check the monitoring dashboard
3. **Create Test Workflow**: Try creating a simple research workflow
4. **Explore Templates**: Browse available research templates

## Architecture Overview

### System Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Frontend      │    │   Tauri Core    │    │   Backend       │
│   (React/TS)    │◄──►│   (Rust)        │◄──►│   Services      │
│                 │    │                 │    │   (Rust)        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   UI Components │    │   IPC Bridge    │    │   API Manager   │
│   State Mgmt    │    │   Commands      │    │   Research Eng  │
│   Routing       │    │   Events        │    │   Security      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Technology Stack

**Frontend:**
- **React 18**: UI framework with concurrent features
- **TypeScript**: Type safety and developer experience
- **Tailwind CSS**: Utility-first styling
- **React Query**: Server state management
- **React Router**: Client-side routing
- **Vite**: Build tool and dev server

**Backend:**
- **Rust**: Systems programming language
- **Tauri**: Desktop app framework
- **Tokio**: Async runtime
- **Serde**: Serialization framework
- **SQLite**: Local database

**Development Tools:**
- **ESLint**: Code linting
- **Prettier**: Code formatting
- **Vitest**: Unit testing
- **Playwright**: E2E testing
- **Husky**: Git hooks

## Development Environment

### VS Code Setup

Install recommended extensions:

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "tauri-apps.tauri-vscode",
    "bradlc.vscode-tailwindcss",
    "esbenp.prettier-vscode",
    "ms-vscode.vscode-typescript-next"
  ]
}
```

### Environment Variables

Create `.env.local`:

```bash
# Development settings
VITE_APP_ENV=development
VITE_API_BASE_URL=http://localhost:1420

# Feature flags
VITE_ENABLE_ANALYTICS=true
VITE_ENABLE_DEBUG=true

# External services (optional for development)
VITE_OPENROUTER_API_URL=https://openrouter.ai/api/v1
VITE_SERPAPI_URL=https://serpapi.com/search
```

### Development Scripts

```bash
# Development
npm run dev              # Start dev server
npm run dev:debug        # Start with debug logging
npm run dev:clean        # Clean start (clear cache)

# Building
npm run build            # Build for production
npm run build:debug     # Build with debug info
npm run preview         # Preview production build

# Testing
npm run test            # Run unit tests
npm run test:watch      # Run tests in watch mode
npm run test:e2e        # Run E2E tests
npm run test:coverage   # Generate coverage report

# Code Quality
npm run lint            # Run ESLint
npm run lint:fix        # Fix ESLint issues
npm run format          # Format with Prettier
npm run type-check      # TypeScript type checking

# Performance
npm run analyze         # Bundle analysis
npm run perf:lighthouse # Lighthouse audit
```

## Project Structure

```
free-deep-research/
├── src/                          # Frontend source code
│   ├── components/               # React components
│   │   ├── common/              # Reusable components
│   │   ├── dashboard/           # Dashboard components
│   │   ├── research/            # Research workflow components
│   │   ├── templates/           # Template management
│   │   └── monitoring/          # System monitoring
│   ├── hooks/                   # Custom React hooks
│   ├── services/                # API service layer
│   ├── utils/                   # Utility functions
│   ├── types/                   # TypeScript type definitions
│   └── styles/                  # Global styles
├── src-tauri/                   # Tauri backend
│   ├── src/                     # Rust source code
│   │   ├── commands/            # Tauri commands
│   │   ├── services/            # Business logic services
│   │   └── models/              # Data models
│   └── Cargo.toml              # Rust dependencies
├── docs/                        # Documentation
├── tests/                       # Test files
└── public/                      # Static assets
```

### Component Organization

```
components/
├── common/                      # Shared components
│   ├── ErrorBoundary.tsx       # Error handling
│   ├── LoadingSpinner.tsx      # Loading states
│   ├── LazyWrapper.tsx         # Lazy loading
│   └── Modal.tsx               # Modal dialogs
├── dashboard/                   # Dashboard specific
│   ├── ExecutiveDashboard.tsx  # Main dashboard
│   ├── MetricsCard.tsx         # Metric displays
│   └── ActivityFeed.tsx        # Recent activity
└── research/                    # Research workflow
    ├── WorkflowCreator.tsx     # Workflow creation
    ├── WorkflowList.tsx        # Workflow management
    └── ProgressTracker.tsx     # Progress monitoring
```

## Development Workflow

### Git Workflow

1. **Feature Branches**: Create feature branches from `main`
2. **Conventional Commits**: Use conventional commit messages
3. **Pull Requests**: All changes go through PR review
4. **Automated Testing**: CI runs tests on all PRs

### Commit Message Format

```
type(scope): description

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test changes
- `chore`: Build/tooling changes

**Examples:**
```bash
feat(api): add template execution endpoint
fix(ui): resolve loading spinner positioning
docs(readme): update installation instructions
```

### Code Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are included and passing
- [ ] Documentation is updated
- [ ] Performance impact considered
- [ ] Security implications reviewed
- [ ] Accessibility requirements met

## Testing Strategy

### Unit Testing

```typescript
// Example unit test
import { render, screen } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import ApiKeyManager from './ApiKeyManager'

describe('ApiKeyManager', () => {
  it('displays API keys correctly', async () => {
    const queryClient = new QueryClient({
      defaultOptions: { queries: { retry: false } }
    })
    
    render(
      <QueryClientProvider client={queryClient}>
        <ApiKeyManager />
      </QueryClientProvider>
    )
    
    expect(screen.getByText('API Key Management')).toBeInTheDocument()
  })
})
```

### Integration Testing

```typescript
// Example integration test
import { test, expect } from '@playwright/test'

test('research workflow creation', async ({ page }) => {
  await page.goto('/research')
  
  await page.click('[data-testid="create-workflow"]')
  await page.fill('[data-testid="workflow-name"]', 'Test Workflow')
  await page.fill('[data-testid="workflow-query"]', 'AI research')
  await page.click('[data-testid="submit-workflow"]')
  
  await expect(page.locator('[data-testid="workflow-created"]')).toBeVisible()
})
```

### Performance Testing

```typescript
// Example performance test
import { performanceMonitor } from '@/utils/performance'

test('component render performance', () => {
  const measurementId = performanceMonitor.startMeasurement(
    'component-render',
    'component'
  )
  
  render(<ExpensiveComponent />)
  
  const metrics = performanceMonitor.endMeasurement(measurementId)
  expect(metrics.duration).toBeLessThan(100) // 100ms budget
})
```

## Performance Guidelines

### Component Performance

1. **React.memo**: Memoize expensive components
2. **useMemo/useCallback**: Memoize expensive calculations
3. **Lazy Loading**: Load components on demand
4. **Code Splitting**: Split large bundles

### Bundle Optimization

1. **Tree Shaking**: Remove unused code
2. **Dynamic Imports**: Load code on demand
3. **Vendor Splitting**: Separate vendor code
4. **Asset Optimization**: Optimize images and fonts

### Performance Monitoring

```typescript
import { usePerformanceMonitoring } from '@/utils/performance'

function MyComponent() {
  const { startMeasurement, endMeasurement } = usePerformanceMonitoring()
  
  useEffect(() => {
    const id = startMeasurement('component-mount', 'component')
    return () => endMeasurement(id)
  }, [])
  
  // Component logic
}
```

## Security Guidelines

### Input Validation

```typescript
import { validateInput, sanitize } from '@/utils/validation'

function handleUserInput(input: string) {
  // Validate
  const validation = validateInput.string(input, 100)
  if (!validation.isValid) {
    throw new Error(validation.errors.join(', '))
  }
  
  // Sanitize
  const sanitized = sanitize.string(validation.sanitized)
  
  return sanitized
}
```

### Secure API Calls

```typescript
import { useSecurity } from '@/hooks/useSecurity'

function SecureComponent() {
  const { secureInvoke } = useSecurity()
  
  const handleApiCall = async () => {
    try {
      const result = await secureInvoke('sensitive_command', data, {
        rateLimitKey: 'sensitive-operation',
        auditEvent: {
          event_type: 'sensitive_operation',
          risk_level: 'high'
        }
      })
      return result
    } catch (error) {
      // Handle security errors
    }
  }
}
```

## Deployment

### Build Process

```bash
# Production build
npm run build

# Tauri build
npm run tauri:build

# Build with analysis
npm run analyze
```

### Environment Configuration

```bash
# Production environment
NODE_ENV=production
VITE_APP_ENV=production
VITE_API_BASE_URL=https://api.yourapp.com
```

### Release Process

1. **Version Bump**: Update version in `package.json` and `Cargo.toml`
2. **Changelog**: Update `CHANGELOG.md`
3. **Build**: Create production build
4. **Test**: Run full test suite
5. **Tag**: Create git tag
6. **Release**: Create GitHub release

## Troubleshooting

### Common Issues

**Build Errors:**
```bash
# Clear cache and reinstall
rm -rf node_modules package-lock.json
npm install

# Clear Tauri cache
cd src-tauri && cargo clean && cd ..
```

**Type Errors:**
```bash
# Regenerate types
npm run type-check
```

**Performance Issues:**
```bash
# Analyze bundle
npm run analyze

# Check performance
npm run perf:lighthouse
```

### Debug Mode

Enable debug logging:

```typescript
// In development
if (process.env.NODE_ENV === 'development') {
  window.__DEBUG__ = true
}
```

### Logging

```typescript
import { logger } from '@/utils/logger'

// Structured logging
logger.info('User action', {
  action: 'create_workflow',
  userId: 'user123',
  metadata: { workflowType: 'research' }
})
```

## Contributing

### Getting Started

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

### Code Standards

- Follow TypeScript strict mode
- Use functional components with hooks
- Write comprehensive tests
- Document public APIs
- Follow accessibility guidelines

### Review Process

1. Automated checks must pass
2. Code review by maintainers
3. Manual testing if needed
4. Merge after approval

## Resources

- [React Documentation](https://react.dev)
- [Tauri Documentation](https://tauri.app)
- [Rust Book](https://doc.rust-lang.org/book/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Tailwind CSS](https://tailwindcss.com/docs)

## Support

- **Issues**: GitHub Issues for bug reports
- **Discussions**: GitHub Discussions for questions
- **Security**: Email security@yourapp.com for security issues
