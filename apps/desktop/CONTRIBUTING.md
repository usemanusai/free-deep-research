# Contributing to Free Deep Research System

Thank you for your interest in contributing to the Free Deep Research System! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Code Standards](#code-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Issue Reporting](#issue-reporting)

## Code of Conduct

### Our Pledge

We are committed to providing a welcoming and inclusive environment for all contributors, regardless of background, experience level, or identity.

### Expected Behavior

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Personal attacks or trolling
- Publishing private information without permission
- Any conduct that would be inappropriate in a professional setting

## Getting Started

### Prerequisites

- **Rust** 1.75+ with Cargo
- **Node.js** 20.x+ with npm
- **Git** for version control
- **Code Editor** (VS Code recommended with Rust and TypeScript extensions)

### Development Environment Setup

```bash
# 1. Fork and clone the repository
git clone https://github.com/YOUR_USERNAME/free-deep-research.git
cd free-deep-research/bmad-agent/free-deep-research

# 2. Install dependencies
cargo install tauri-cli
npm install

# 3. Setup development tools
npm install -g concurrently
cargo install cargo-watch

# 4. Setup pre-commit hooks
npm install husky --save-dev
npx husky install

# 5. Start development environment
npm run dev
```

## Development Workflow

### Branch Naming Convention

Use descriptive branch names with the following prefixes:

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test improvements
- `chore/` - Maintenance tasks

Examples:
- `feature/api-key-bulk-import`
- `fix/rate-limit-calculation`
- `docs/installation-guide`

### Commit Message Format

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

Examples:
```
feat(api): add bulk API key import functionality
fix(rate-limit): correct calculation for monthly limits
docs: update installation instructions for Linux
```

## Code Standards

### Rust Guidelines

#### Formatting and Style
```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Run both with strict settings
cargo fmt -- --check
cargo clippy -- -D warnings
```

#### Best Practices
- Use `#[derive(Debug)]` for all structs
- Implement proper error handling with custom error types
- Write comprehensive documentation for public APIs
- Use meaningful variable and function names
- Prefer explicit types over type inference in public APIs
- Handle all `Result` types explicitly

#### Example Code Style
```rust
/// Manages API key operations with rate limiting
#[derive(Debug, Clone)]
pub struct ApiKeyManager {
    keys: HashMap<Uuid, ApiKey>,
    rate_limiter: RateLimiter,
}

impl ApiKeyManager {
    /// Creates a new API key manager
    /// 
    /// # Arguments
    /// 
    /// * `config` - Configuration for rate limiting
    /// 
    /// # Returns
    /// 
    /// A new `ApiKeyManager` instance
    pub fn new(config: RateLimitConfig) -> AppResult<Self> {
        let rate_limiter = RateLimiter::new(config)?;
        
        Ok(Self {
            keys: HashMap::new(),
            rate_limiter,
        })
    }
}
```

### TypeScript Guidelines

#### ESLint and Prettier
```bash
# Lint code
npm run lint

# Format code
npm run format

# Type checking
npm run type-check
```

#### Best Practices
- Use strict TypeScript settings
- Prefer functional components with hooks
- Use proper TypeScript types (avoid `any`)
- Implement proper error boundaries
- Use meaningful component and variable names
- Write comprehensive unit tests

#### Example Code Style
```typescript
interface ApiKeyFormProps {
  onSubmit: (apiKey: CreateApiKeyRequest) => Promise<void>;
  initialValues?: Partial<CreateApiKeyRequest>;
  isLoading?: boolean;
}

export const ApiKeyForm: React.FC<ApiKeyFormProps> = ({
  onSubmit,
  initialValues,
  isLoading = false,
}) => {
  const [formData, setFormData] = useState<CreateApiKeyRequest>({
    service: ServiceProvider.OPENROUTER,
    name: '',
    key: '',
    rateLimit: 100,
    ...initialValues,
  });

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    
    try {
      await onSubmit(formData);
    } catch (error) {
      console.error('Failed to submit API key:', error);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      {/* Form implementation */}
    </form>
  );
};
```

## Testing Guidelines

### Test Coverage Requirements

- **Minimum Coverage**: 80% for new code
- **Target Coverage**: 90% for critical components
- **Required Tests**: All public APIs must have tests

### Running Tests

```bash
# Frontend tests
npm test                    # Unit tests
npm run test:e2e           # End-to-end tests
npm run test:coverage      # Coverage report

# Backend tests
cargo test                 # Unit and integration tests
cargo test --release      # Performance tests

# Full test suite
npm run test:all          # All tests
```

### Test Structure

#### Rust Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_key_creation() {
        let manager = ApiKeyManager::new(default_config()).unwrap();
        let request = CreateApiKeyRequest {
            service: ServiceProvider::OpenRouter,
            name: "Test Key".to_string(),
            key: "test-key".to_string(),
            rate_limit: 100,
        };

        let result = manager.add_key(request).await;
        assert!(result.is_ok());
    }
}
```

#### TypeScript Tests
```typescript
import { render, screen, fireEvent } from '@testing-library/react';
import { ApiKeyForm } from './ApiKeyForm';

describe('ApiKeyForm', () => {
  it('should submit form with correct data', async () => {
    const mockOnSubmit = jest.fn();
    
    render(<ApiKeyForm onSubmit={mockOnSubmit} />);
    
    fireEvent.change(screen.getByLabelText('Name'), {
      target: { value: 'Test Key' }
    });
    
    fireEvent.click(screen.getByRole('button', { name: 'Submit' }));
    
    expect(mockOnSubmit).toHaveBeenCalledWith({
      name: 'Test Key',
      // ... other expected values
    });
  });
});
```

## Documentation

### Documentation Requirements

- Update README.md for significant changes
- Maintain inline code documentation
- Update API documentation for interface changes
- Include examples in documentation
- Keep changelog updated

### Documentation Style

- Use clear, concise language
- Include code examples
- Provide context and rationale
- Use proper markdown formatting
- Include diagrams where helpful

## Pull Request Process

### Before Submitting

1. **Ensure all tests pass**
   ```bash
   npm run test:all
   cargo test
   ```

2. **Run quality checks**
   ```bash
   npm run validate
   cargo clippy
   cargo fmt -- --check
   ```

3. **Update documentation**
   - Update README if needed
   - Add/update inline documentation
   - Update changelog

### PR Template

When creating a pull request, include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
```

### Review Process

1. **Automated Checks**: CI/CD pipeline runs tests and quality checks
2. **Code Review**: At least one maintainer reviews the code
3. **Testing**: Reviewer tests the changes locally if needed
4. **Approval**: Maintainer approves and merges the PR

## Issue Reporting

### Bug Reports

Include the following information:

```markdown
**Environment**
- OS: [e.g., Windows 10, macOS 12, Ubuntu 20.04]
- Application Version: [e.g., 1.0.0]
- Node.js Version: [e.g., 20.10.0]
- Rust Version: [e.g., 1.75.0]

**Description**
Clear description of the bug

**Steps to Reproduce**
1. Step one
2. Step two
3. Step three

**Expected Behavior**
What should happen

**Actual Behavior**
What actually happens

**Screenshots/Logs**
If applicable, add screenshots or error logs
```

### Feature Requests

Include the following information:

```markdown
**Problem Statement**
What problem does this feature solve?

**Proposed Solution**
Describe your proposed solution

**Alternatives Considered**
Other solutions you've considered

**Additional Context**
Any other context or screenshots
```

## Getting Help

- **Documentation**: [Project Wiki](https://github.com/usemanusai/free-deep-research/wiki)
- **Discussions**: [GitHub Discussions](https://github.com/usemanusai/free-deep-research/discussions)
- **Issues**: [GitHub Issues](https://github.com/usemanusai/free-deep-research/issues)

Thank you for contributing to the Free Deep Research System! 🚀
