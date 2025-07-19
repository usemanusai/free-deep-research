# 🤝 Contributing Guide

## Welcome Contributors!

Thank you for your interest in contributing to the Free Deep Research System! This guide will help you get started with contributing code, documentation, and other improvements to the project.

## 🎯 Ways to Contribute

### Code Contributions
- **Bug Fixes**: Fix issues and improve stability
- **New Features**: Add functionality and capabilities
- **Performance Improvements**: Optimize speed and efficiency
- **Security Enhancements**: Strengthen system security
- **API Improvements**: Enhance API design and functionality

### Documentation Contributions
- **User Guides**: Improve user documentation
- **Developer Docs**: Enhance technical documentation
- **API Documentation**: Update API references
- **Tutorials**: Create learning materials
- **Translations**: Localize content

### Community Contributions
- **Bug Reports**: Identify and report issues
- **Feature Requests**: Suggest improvements
- **Testing**: Help test new features
- **Support**: Help other users in forums
- **Advocacy**: Share the project with others

## 🚀 Getting Started

### Prerequisites

#### Development Environment
- **Node.js**: 18.x or higher
- **Rust**: 1.75.0 or higher
- **Git**: Latest version
- **Docker**: 24.0+ (optional, for containerized development)

#### Accounts
- **GitHub Account**: For code contributions
- **Discord**: For community discussions (optional)

### Setting Up Development Environment

#### 1. Fork and Clone
```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/free-deep-research.git
cd free-deep-research

# Add upstream remote
git remote add upstream https://github.com/huggingfacer04/free-deep-research.git
```

#### 2. Install Dependencies
```bash
# Install Node.js dependencies
npm install

# Install Rust dependencies
cd src-tauri
cargo build
cd ..

# Install frontend dependencies
cd bmad-agent/deep_research_frontend
npm install
cd ../..
```

#### 3. Environment Configuration
```bash
# Copy environment template
cp .env.template .env

# Configure your development settings
nano .env
```

#### 4. Verify Setup
```bash
# Run tests
npm test
cargo test

# Start development server
npm run tauri dev
```

## 📋 Contribution Process

### 1. Choose an Issue

#### Finding Issues
- **Good First Issues**: Look for `good-first-issue` label
- **Help Wanted**: Check `help-wanted` label
- **Bug Reports**: Browse open bug reports
- **Feature Requests**: Review feature request discussions

#### Creating New Issues
```markdown
## Bug Report Template
**Describe the bug**
A clear description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. See error

**Expected behavior**
What you expected to happen.

**Screenshots**
If applicable, add screenshots.

**Environment:**
- OS: [e.g. Windows 10, macOS 12.0, Ubuntu 20.04]
- Browser: [e.g. Chrome 96, Firefox 95]
- Version: [e.g. 3.0.0]
```

### 2. Create a Branch

#### Branch Naming Convention
```bash
# Feature branches
git checkout -b feature/add-new-search-algorithm

# Bug fix branches
git checkout -b fix/resolve-memory-leak

# Documentation branches
git checkout -b docs/update-api-guide

# Refactoring branches
git checkout -b refactor/optimize-database-queries
```

### 3. Make Changes

#### Code Style Guidelines

**TypeScript/JavaScript**
```typescript
// Use meaningful variable names
const researchWorkflowResults = await fetchWorkflowResults(workflowId);

// Use async/await instead of promises
async function processResearchData(data: ResearchData): Promise<ProcessedData> {
  try {
    const processed = await analyzeData(data);
    return processed;
  } catch (error) {
    logger.error('Failed to process research data', error);
    throw error;
  }
}

// Use proper TypeScript types
interface ResearchQuery {
  text: string;
  methodology: 'hybrid' | 'don_lim' | 'nick_scamara';
  maxSources: number;
  timeframe?: string;
}
```

**Rust**
```rust
// Use descriptive function names
pub async fn execute_research_workflow(
    query: ResearchQuery,
    config: WorkflowConfig,
) -> Result<WorkflowResult, WorkflowError> {
    // Implementation
}

// Use proper error handling
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorkflowError {
    #[error("Invalid query: {0}")]
    InvalidQuery(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
```

#### Commit Message Format
```bash
# Format: <type>(<scope>): <description>
# Types: feat, fix, docs, style, refactor, test, chore

# Examples:
git commit -m "feat(api): add new research methodology endpoint"
git commit -m "fix(ui): resolve memory leak in research results component"
git commit -m "docs(guide): update installation instructions for macOS"
git commit -m "test(workflow): add integration tests for research engine"
```

### 4. Testing

#### Running Tests
```bash
# Frontend tests
npm run test

# Backend tests
cd src-tauri
cargo test

# Integration tests
npm run test:integration

# E2E tests
npm run test:e2e
```

#### Writing Tests
```typescript
// Unit test example
describe('ResearchEngine', () => {
  it('should process valid research query', async () => {
    const engine = new ResearchEngine();
    const query = {
      text: 'AI in healthcare',
      methodology: 'hybrid',
      maxSources: 25
    };

    const result = await engine.processQuery(query);

    expect(result).toBeDefined();
    expect(result.sources).toHaveLength(25);
    expect(result.confidence).toBeGreaterThan(0.8);
  });
});
```

```rust
// Rust test example
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_research_workflow_creation() {
        let engine = ResearchEngine::new();
        let query = ResearchQuery {
            text: "AI in healthcare".to_string(),
            methodology: Methodology::Hybrid,
            max_sources: 25,
        };

        let result = engine.create_workflow(query).await;

        assert!(result.is_ok());
        let workflow = result.unwrap();
        assert_eq!(workflow.status, WorkflowStatus::Created);
    }
}
```

### 5. Documentation

#### Code Documentation
```typescript
/**
 * Processes a research query and returns structured results
 * @param query - The research query to process
 * @param options - Optional configuration for processing
 * @returns Promise resolving to processed research results
 * @throws {ValidationError} When query is invalid
 * @throws {NetworkError} When external APIs are unavailable
 */
async function processResearchQuery(
  query: ResearchQuery,
  options?: ProcessingOptions
): Promise<ResearchResults> {
  // Implementation
}
```

```rust
/// Executes a research workflow with the given configuration
/// 
/// # Arguments
/// 
/// * `query` - The research query to execute
/// * `config` - Workflow configuration parameters
/// 
/// # Returns
/// 
/// Returns a `Result` containing the workflow results or an error
/// 
/// # Errors
/// 
/// This function will return an error if:
/// * The query is invalid or empty
/// * Network requests fail
/// * Database operations fail
pub async fn execute_workflow(
    query: ResearchQuery,
    config: WorkflowConfig,
) -> Result<WorkflowResult, WorkflowError> {
    // Implementation
}
```

#### README Updates
When adding new features, update relevant README files:
- Main project README
- Component-specific READMEs
- API documentation
- User guides

### 6. Pull Request

#### PR Template
```markdown
## Description
Brief description of changes made.

## Type of Change
- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] E2E tests pass
- [ ] Manual testing completed

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Code is commented where necessary
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] No breaking changes (or breaking changes documented)
```

#### PR Best Practices
1. **Keep PRs Small**: Focus on single feature or fix
2. **Write Clear Descriptions**: Explain what and why
3. **Include Tests**: Add or update tests for changes
4. **Update Documentation**: Keep docs current
5. **Respond to Feedback**: Address review comments promptly

## 🔍 Code Review Process

### Review Criteria

#### Code Quality
- **Functionality**: Does the code work as intended?
- **Readability**: Is the code easy to understand?
- **Maintainability**: Is the code easy to modify?
- **Performance**: Are there any performance concerns?
- **Security**: Are there any security vulnerabilities?

#### Testing
- **Coverage**: Are new features/fixes tested?
- **Quality**: Are tests meaningful and comprehensive?
- **Edge Cases**: Are edge cases considered?

#### Documentation
- **Completeness**: Is documentation updated?
- **Accuracy**: Is documentation correct?
- **Clarity**: Is documentation easy to understand?

### Review Process
1. **Automated Checks**: CI/CD pipeline runs automatically
2. **Peer Review**: Team members review code
3. **Feedback**: Reviewers provide constructive feedback
4. **Iteration**: Author addresses feedback
5. **Approval**: Reviewers approve changes
6. **Merge**: Changes are merged to main branch

## 🏆 Recognition

### Contributor Recognition
- **Contributors List**: Added to project contributors
- **Release Notes**: Contributions mentioned in releases
- **Community Highlights**: Featured in community updates
- **Badges**: GitHub profile badges for contributions

### Becoming a Maintainer
Regular contributors may be invited to become maintainers with:
- **Commit Access**: Direct commit privileges
- **Review Rights**: Ability to approve PRs
- **Release Management**: Help with releases
- **Community Leadership**: Guide project direction

## 📚 Resources

### Development Resources
- **[Development Guide](./README.md)**: Complete development setup
- **[API Documentation](../api/README.md)**: API reference
- **[Architecture Guide](../architecture/README.md)**: System architecture
- **[Testing Guide](./testing.md)**: Testing strategies

### Community Resources
- **[Discord Server](https://discord.gg/freedeepresearch)**: Real-time discussions
- **[GitHub Discussions](https://github.com/huggingfacer04/free-deep-research/discussions)**: Project discussions
- **[Community Forum](https://community.freedeepresearch.org)**: User community

### Learning Resources
- **[Rust Book](https://doc.rust-lang.org/book/)**: Learn Rust programming
- **[React Documentation](https://reactjs.org/docs/)**: React development
- **[Tauri Guide](https://tauri.app/v1/guides/)**: Tauri framework
- **[TypeScript Handbook](https://www.typescriptlang.org/docs/)**: TypeScript development

## 🆘 Getting Help

### Where to Ask Questions
1. **GitHub Discussions**: General project questions
2. **Discord**: Real-time help and discussions
3. **Issues**: Bug reports and feature requests
4. **Email**: Direct contact for sensitive issues

### Mentorship Program
New contributors can request mentorship:
- **Pairing Sessions**: Work with experienced contributors
- **Code Reviews**: Detailed feedback on contributions
- **Project Guidance**: Help choosing appropriate issues
- **Career Development**: Open source contribution guidance

---

**Thank you for contributing!** 🎉

Your contributions help make Free Deep Research better for everyone. Whether you're fixing a typo or adding a major feature, every contribution is valuable and appreciated.

**Questions?** Join our [Discord community](https://discord.gg/freedeepresearch) or start a [GitHub Discussion](https://github.com/huggingfacer04/free-deep-research/discussions).
